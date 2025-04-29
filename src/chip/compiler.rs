use std::{collections::{HashMap, HashSet}, vec};

use super::{chip::ChipType, chip_description::ChipDescription, circuit_description::CircuitDescription, types::{ChipAndPin, Link, NodeType, NodeTypeMap}};

pub struct ChipCompiler {}

type IDNode = (usize, NodeType);

impl ChipCompiler {
    pub fn compile(circuit: CircuitDescription) -> ChipDescription {
        
        let mut chip_pin_to_id_node: HashMap<ChipAndPin, IDNode> = HashMap::new();

        let mut num_nodes = Self::add_inputs_and_outputs(&mut chip_pin_to_id_node, &circuit);
        num_nodes = Self::add_all_nands(&mut chip_pin_to_id_node, &circuit, num_nodes);        
        Self::add_buffer_nodes(&mut chip_pin_to_id_node, &circuit, num_nodes);

        let mut forward_links: HashMap<usize, Vec<usize>> = HashMap::new();
        let mut back_links: HashMap<usize, Vec<usize>> = HashMap::new();

        Self::add_internal_links(&mut forward_links, &mut back_links, &circuit, &chip_pin_to_id_node);
        Self::add_external_links(&mut forward_links, &mut back_links, &circuit, &chip_pin_to_id_node);

        let mut id_type_map: HashMap<usize, NodeType> = chip_pin_to_id_node.iter()
            .map(|(_, (id, node_type))| (*id, *node_type))
            .collect();

        Self::prune_unused_buffer_nodes(&mut forward_links, &mut back_links, &mut id_type_map);
        
        Self::explode_buffers(&mut forward_links, &mut back_links, &mut id_type_map);

        let links: Vec<Link> = Self::construct_links(&forward_links);

        ChipDescription::new(id_type_map, links)
    }
    
    // Add nodes for each circuit ground, supply, input and output
    fn add_inputs_and_outputs(chip_pin_to_id_node: &mut HashMap<ChipAndPin, IDNode>, circuit: &CircuitDescription) -> usize {
        let mut node_id: usize = 0;
        for (id, chip_type) in &circuit.chip_types {
            match chip_type {
                ChipType::Ground => {
                    chip_pin_to_id_node.insert(ChipAndPin::new(*id, 0), (node_id, NodeType::Ground));
                    node_id += 1;
                },
                ChipType::Supply => {
                    chip_pin_to_id_node.insert(ChipAndPin::new(*id, 0), (node_id, NodeType::Supply));
                    node_id += 1;
                },
                ChipType::Input => {
                    chip_pin_to_id_node.insert(ChipAndPin::new(*id, 0), (node_id, NodeType::Input));
                    node_id += 1;
                },
                ChipType::Output => {
                    chip_pin_to_id_node.insert(ChipAndPin::new(*id, 0), (node_id, NodeType::Output));
                    node_id += 1;
                }
                _ => {}
            };
        }
        return node_id;
    }
    
    // "explode" all chips: add nand nodes for each nand in each chip
    fn add_all_nands(chip_pin_to_id_node: &mut HashMap<ChipAndPin, IDNode>, circuit: &CircuitDescription, num_nodes: usize) -> usize {
        let mut new_node_id = num_nodes;

        for (chip_id, description) in &circuit.chip_descriptions {
            for (old_node_id, node_type) in &description.id_type_map {
                if node_type == &NodeType::NAnd {
                    let chip_and_pin = ChipAndPin::new(*chip_id, *old_node_id);
                    chip_pin_to_id_node.insert(chip_and_pin, (new_node_id, NodeType::NAnd));
                    new_node_id += 1;
                }
            }
        }

        return new_node_id;
    }
    
    // add temporary "buffer" nodes for each ground, supply, input and output in each chip
    fn add_buffer_nodes(chip_pin_to_id_node: &mut HashMap<ChipAndPin, (usize, NodeType)>, circuit: &CircuitDescription, num_nodes: usize) -> usize {
        let mut new_node_id = num_nodes;

        for (chip_id, description) in &circuit.chip_descriptions {
            for (old_node_id, node_type) in &description.id_type_map {
                let chip_and_pin = ChipAndPin::new(*chip_id, *old_node_id);
                match node_type {
                    NodeType::Ground => {
                        chip_pin_to_id_node.insert(chip_and_pin, (new_node_id, NodeType::Buffer));
                        new_node_id += 1;
                    },
                    NodeType::Supply => {
                        chip_pin_to_id_node.insert(chip_and_pin, (new_node_id, NodeType::Buffer));
                        new_node_id += 1;
                    },
                    NodeType::Input => {
                        chip_pin_to_id_node.insert(chip_and_pin, (new_node_id, NodeType::Buffer));
                        new_node_id += 1;
                    },
                    NodeType::Output => {
                        chip_pin_to_id_node.insert(chip_and_pin, (new_node_id, NodeType::Buffer));
                        new_node_id += 1;
                    },
                    _ => {},
                }
            }
        }

        return new_node_id;
    }
    
    // add node links for each internal chip node link
    fn add_internal_links(
        forward_links: &mut HashMap<usize, Vec<usize>>, 
        back_links: &mut HashMap<usize, Vec<usize>>, 
        circuit: &CircuitDescription,
        chip_pin_to_id_node: &HashMap<ChipAndPin, IDNode>) 
    {
        for (chip_id, description) in &circuit.chip_descriptions {
            for (old_source_node_id, old_target_ids) in &description.forward_links {
                let source_chip_pin = ChipAndPin::new(*chip_id, *old_source_node_id);
                let (new_source_id, _) = chip_pin_to_id_node.get(&source_chip_pin).expect("Old node not found!");

                let new_target_ids: Vec<usize> = old_target_ids.iter()
                    .map(|old| ChipAndPin::new(*chip_id, *old))
                    .map(|cp| chip_pin_to_id_node.get(&cp))
                    .map(|id_node_type| id_node_type.expect("chip and pin missing!"))
                    .map(|id_node_type| id_node_type.0)
                    .collect();

                forward_links.insert(*new_source_id, new_target_ids.clone());
                for target in &new_target_ids {
                    back_links.entry(*target).or_default().push(*new_source_id);
                }
            }
        }
    }

    // create node links for each circuit link
    fn add_external_links(
        forward_links: &mut HashMap<usize, Vec<usize>>, 
        back_links: &mut HashMap<usize, Vec<usize>>, 
        circuit: &CircuitDescription, 
        chip_pin_to_id_node: &HashMap<ChipAndPin, IDNode>)
    {
        for (chip_id, pin_links) in &circuit.forward_links {
            for (pin, old_targets) in pin_links {
                let chip_pin = ChipAndPin::new(*chip_id, *pin);
                let (new_source_id, _) = chip_pin_to_id_node.get(&chip_pin).expect("Old node not found!");

                let new_target_ids: Vec<usize> = old_targets.iter()
                    .map(|cp| chip_pin_to_id_node.get(&cp))
                    .map(|id_node_type| id_node_type.expect("chip and pin missing!"))
                    .map(|id_node_type| id_node_type.0)
                    .collect();

                forward_links.insert(*new_source_id, new_target_ids.clone());
                for target in &new_target_ids {
                    back_links.entry(*target).or_default().push(*new_source_id);
                }
            }
        }
    }
    
    // delete each buffer node that have no link targets, and any links targeting them
    fn prune_unused_buffer_nodes(
        forward_links: &mut HashMap<usize, Vec<usize>>, 
        back_links: &mut HashMap<usize, Vec<usize>>, 
        id_types: &mut HashMap<usize, NodeType>)
    {
        let unsourced_buffers: HashSet<usize> = id_types.iter()
            .filter(|(_, node_type)| *node_type == &NodeType::Buffer)
            .filter(|(buffer_id, _)| forward_links.entry(**buffer_id).or_default().len() == 0)
            .map(|(buffer_id, _)| *buffer_id)
            .collect();

        let untargeted_buffers: HashSet<usize> = id_types.iter()
            .filter(|(_, node_type)| *node_type == &NodeType::Buffer)
            .filter(|(buffer_id, _)| back_links.entry(**buffer_id).or_default().len() == 0)
            .map(|(buffer_id, _)| *buffer_id)
            .collect();

        let unused_buffers = unsourced_buffers.union(&untargeted_buffers);

        for buffer_id in unused_buffers {
            forward_links.remove(&buffer_id);
            back_links.remove(&buffer_id);
            id_types.remove(&buffer_id);
        }
    }
    
    // "explode" the remaining buffer nodes: for each buffer source,
    // add a link from each of its ancestors to each of its targets, then delete itself
    // repeat until there are no more buffers
    fn explode_buffers(
        forward_links: &mut HashMap<usize, Vec<usize>>,
        back_links: &mut HashMap<usize, Vec<usize>>,
        id_types: &mut HashMap<usize, NodeType>,
    ) {
        // 1. Identify all buffer node IDs first.
        //    We collect them into a separate list to avoid issues with modifying
        //    the maps while iterating over them.
        let buffer_ids: Vec<usize> = id_types
            .iter()
            .filter(|(_, node_type)| node_type == &&NodeType::Buffer)
            .map(|(&id, _)| id)
            .collect();

        // 2. Process each buffer node.
        for buffer_id in buffer_ids {
            // Check if the buffer still exists (it might have been removed if
            // it was part of a chain processed earlier, though this specific
            // implementation processes all identified buffers once).
            if id_types.get(&buffer_id) != Some(&NodeType::Buffer) {
                continue;
            }

            // 2a. Get sources (predecessors) and targets (successors).
            // Clone the lists to avoid borrowing issues while modifying the maps.
            // Use empty Vec if the buffer has no incoming/outgoing links.
            let sources = back_links.get(&buffer_id).cloned().unwrap_or_default();
            let targets = forward_links.get(&buffer_id).cloned().unwrap_or_default();

            // 2b. Rewire: Connect each source directly to each target.
            for &source_id in &sources {
                // Ensure the source node still exists and is not a buffer itself
                // (though this function assumes it only processes buffers identified initially)
                if id_types.contains_key(&source_id) {
                    for &target_id in &targets {
                        // Ensure the target node still exists
                        if id_types.contains_key(&target_id) {
                            // Add forward link: source -> target
                            if let Some(source_fwd_links) = forward_links.get_mut(&source_id) {
                                // Optional: Prevent duplicate links if necessary
                                // if !source_fwd_links.contains(&target_id) {
                                    source_fwd_links.push(target_id);
                                // }
                            }

                            // Add backward link: target <- source
                            if let Some(target_back_links) = back_links.get_mut(&target_id) {
                                // Optional: Prevent duplicate links if necessary
                                // if !target_back_links.contains(&source_id) {
                                    target_back_links.push(source_id);
                                // }
                            }
                        }
                    }
                }
            }

            // 2c. Remove original links involving the buffer node.
            // Remove buffer_id from the forward list of each source.
            for &source_id in &sources {
                if let Some(source_fwd_links) = forward_links.get_mut(&source_id) {
                    source_fwd_links.retain(|&id| id != buffer_id);
                }
            }
            // Remove buffer_id from the backward list of each target.
            for &target_id in &targets {
                if let Some(target_back_links) = back_links.get_mut(&target_id) {
                    target_back_links.retain(|&id| id != buffer_id);
                }
            }

            // 2d. Remove the buffer node itself from all maps.
            forward_links.remove(&buffer_id);
            back_links.remove(&buffer_id);
            id_types.remove(&buffer_id);
        }

        // Optional: Post-processing cleanup - Remove empty entries from maps
        // forward_links.retain(|_, v| !v.is_empty());
        // back_links.retain(|_, v| !v.is_empty());
        // Note: id_types doesn't need this cleanup based on link emptiness.

        // Optional: If duplicate links were added and need removal, clean them up now.
        // This is more complex and might be better handled during insertion or
        // by using a different data structure if uniqueness is critical.
        // Example using HashSet for deduplication (apply to both forward and back links):
        // for links in forward_links.values_mut() {
        //     let unique_links: HashSet<_> = links.drain(..).collect();
        //     links.extend(unique_links);
        // }
        // for links in back_links.values_mut() {
        //     let unique_links: HashSet<_> = links.drain(..).collect();
        //     links.extend(unique_links);
        // }
    }

    fn construct_type_map(chip_pin_to_id_node: &HashMap<ChipAndPin, IDNode>) -> HashMap<usize, NodeType> {
        chip_pin_to_id_node.iter()
            .map(|(_, (id, node_type))| (*id, *node_type))
            .collect()
    }
    
    fn construct_links(forward_links: &HashMap<usize, Vec<usize>>) -> Vec<Link> {
        let mut links: Vec<Link> = vec![];
        for (source, targets) in forward_links {
            for target in targets {
                links.push(Link::new(*source, *target));
            }
        }
        links
    }
}

/*
left: ChipDescription { 
    layout: PinLayout { 
        ground_pins: [1], 
        supply_pins: [0], 
        input_pins: [3, 2], 
        output_pins: [4], 
        num_pins: 5, 
        id_pin_map: {4: 1, 1: 4, 2: 2, 0: 3, 3: 0}, 
        pin_id_map: {3: 0, 0: 3, 2: 2, 4: 1, 1: 4} 
    }, 
    id_type_map: {3: Input, 4: Output, 5: NAnd, 2: Input, 0: Supply, 1: Ground}, 
    forward_links: {5: [4], 2: [9], 3: [5]}, 
    back_links: {5: [3], 4: [5], 9: [2]}, 
    num_nands: 1, 
    is_valid: false 
}
right: ChipDescription { 
    layout: PinLayout { 
        ground_pins: [0], 
        supply_pins: [1], 
        input_pins: [2, 3], 
        output_pins: [4], 
        num_pins: 5, 
        id_pin_map: {1: 2, 4: 1, 3: 4, 0: 0, 2: 3}, 
        pin_id_map: {1: 4, 0: 0, 4: 3, 2: 1, 3: 2} 
    }, 
    id_type_map: {0: Ground, 4: Output, 1: Supply, 5: NAnd, 2: Input, 3: Input}, 
    forward_links: {2: [5], 3: [5], 5: [4]}, 
    back_links: {4: [5], 5: [2, 3]}, 
    num_nands: 1, 
    is_valid: true 
}
 */