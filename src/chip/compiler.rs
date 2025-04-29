use std::collections::HashMap;

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
        let mut back_links: HashMap<usize, usize> = HashMap::new();

        Self::add_internal_links(&mut forward_links, &mut back_links, &circuit, &chip_pin_to_id_node);
        Self::add_external_links(&mut forward_links, &mut back_links, &circuit, &chip_pin_to_id_node);

        let unused_buffers: Vec<ChipAndPin> = Self::prune_unsourced_buffer_nodes(&mut forward_links, &mut back_links, &chip_pin_to_id_node);
        for chip_and_pin in unused_buffers {
            chip_pin_to_id_node.remove(&chip_and_pin);
        }
        
        let buffer_chip_pins: Vec<ChipAndPin> = Self::explode_buffers(&mut forward_links, &mut back_links, &mut chip_pin_to_id_node);
        for chip_and_pin in buffer_chip_pins {
            chip_pin_to_id_node.remove(&chip_and_pin);
        }

        let node_types: NodeTypeMap = Self::construct_type_map(&chip_pin_to_id_node);
        let links: Vec<Link> = Self::construct_links(&forward_links);

        ChipDescription::new(node_types, links)
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
        back_links: &mut HashMap<usize, usize>, 
        circuit: &CircuitDescription,
        chip_pin_to_id_node: &HashMap<ChipAndPin, IDNode>) 
    {
        for (chip_id, description) in &circuit.chip_descriptions {
            for (old_node_id, old_target_ids) in &description.forward_links {
                let chip_pin = ChipAndPin::new(*chip_id, *old_node_id);
                let (new_node_id, _) = chip_pin_to_id_node.get(&chip_pin).expect("Old node not found!");

                let new_target_ids: Vec<usize> = old_target_ids.iter()
                    .map(|old| ChipAndPin::new(*chip_id, *old))
                    .map(|cp| chip_pin_to_id_node.get(&cp))
                    .map(|id_node_type| id_node_type.expect("chip and pin missing!"))
                    .map(|id_node_type| id_node_type.0)
                    .collect();

                forward_links.insert(*new_node_id, new_target_ids.clone());
                for target in &new_target_ids {
                    back_links.insert(*target, *new_node_id);
                }
            }
        }
    }

    // create node links for each circuit link
    fn add_external_links(
        forward_links: &mut HashMap<usize, Vec<usize>>, 
        back_links: &mut HashMap<usize, usize>, 
        circuit: &CircuitDescription, 
        chip_pin_to_id_node: &HashMap<ChipAndPin, IDNode>)
    {
        for (chip_id, pin_links) in &circuit.forward_links {
            for (pin, old_targets) in pin_links {
                let chip_pin = ChipAndPin::new(*chip_id, *pin);
                let (new_node_id, _) = chip_pin_to_id_node.get(&chip_pin).expect("Old node not found!");

                let new_target_ids: Vec<usize> = old_targets.iter()
                    .map(|cp| chip_pin_to_id_node.get(&cp))
                    .map(|id_node_type| id_node_type.expect("chip and pin missing!"))
                    .map(|id_node_type| id_node_type.0)
                    .collect();

                forward_links.insert(*new_node_id, new_target_ids.clone());
                for target in &new_target_ids {
                    back_links.insert(*target, *new_node_id);
                }
            }
        }
    }
    
    // delete each buffer node that have no link targets, and any links targeting them
    fn prune_unsourced_buffer_nodes(
        forward_links: &mut HashMap<usize, Vec<usize>>, 
        back_links: &mut HashMap<usize, usize>, 
        chip_pin_to_id_node: &HashMap<ChipAndPin, IDNode>)
    -> Vec<ChipAndPin> {
        let unused_buffers: &Vec<(&ChipAndPin, &IDNode)> = &chip_pin_to_id_node.iter()
            .filter(|(_, (_, node_type))| node_type == &NodeType::Buffer)
            .filter(|(_, (buffer_id, _))| forward_links.entry(*buffer_id).or_default().len() == 0)
            .collect();

        for (_, (buffer_id, _)) in unused_buffers {
            forward_links.remove(buffer_id);
            back_links.remove(buffer_id);
        }

        let unused_buffer_caps: Vec<ChipAndPin> = unused_buffers.iter()
            .map(|(buffer_cap, _)| **buffer_cap)
            .collect();

        unused_buffer_caps
    }
    
    // "explode" the remaining buffer nodes: for each source, add a link to each target
    // where a source is a buffer node, replace with the buffer node's source
    // where a target is a buffer node, add links instead for each of *its* targets
    fn explode_buffers(
        forward_links: &mut HashMap<usize, Vec<usize>>, 
        back_links: &mut HashMap<usize, usize>, 
        chip_pin_to_id_node: &mut HashMap<ChipAndPin, IDNode>)
    -> Vec<ChipAndPin> {
        let id_types: HashMap<usize, NodeType> = chip_pin_to_id_node.iter()
            .map(|(_, (id, node_type))| (*id, *node_type))
            .collect();

        loop
        {
            let target_ids_with_buffer_source: Vec<(usize, usize)> = back_links.iter()
                .filter(|(_, source)| id_types.get(*source).unwrap() == &NodeType::Buffer)
                .map(|(target, source)| (*target, *source))
                .collect();

            if target_ids_with_buffer_source.len() == 0 {
                break;
            }
            
            for (target, buffer_source) in &target_ids_with_buffer_source {
                let ancestor: usize = *back_links.get(&buffer_source).expect("source not found in backlinks!");
                
                back_links.insert(*target, ancestor);
            }

            for (_, buffer_id) in &target_ids_with_buffer_source {
                back_links.remove(buffer_id);
            }
        }

        // now reconstruct forward links from the remaining back links
        forward_links.clear();
        for (target, source) in back_links.iter() {
            forward_links.entry(*source).or_insert(vec![]).push(*target);
        }
        
        // purge buffers from the chip_pin_node_id map
        let buffer_chip_pins: Vec<ChipAndPin> = chip_pin_to_id_node.iter()
            .filter(|(_, (_, node_type))| node_type == &NodeType::Buffer)
            .map(|(chip_pin, _)| *chip_pin)
            .collect();

        buffer_chip_pins
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
    forward_links: {5: [4], 2: [9], 3: [5]}, back_links: {5: [3], 4: [5], 9: [2]}, 
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