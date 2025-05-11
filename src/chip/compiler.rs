use std::{collections::{HashMap, HashSet}, vec};

use crate::chip_pin;

use super::{chip::ChipType, chip_description::ChipDescription, circuit_description::CircuitDescription, types::{ChipAndPin, Link, NodeType}};

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

        ChipDescription::new(&circuit.name, id_type_map, links)
    }
    
    // Add nodes for each circuit ground, supply, input and output
    fn add_inputs_and_outputs(
        chip_pin_to_id_node: &mut HashMap<ChipAndPin, IDNode>,
        circuit: &CircuitDescription
    ) -> usize {
        let mut new_node_id: usize = 0;
        
        let type_order = [
            (ChipType::Ground, NodeType::Ground),
            (ChipType::Supply, NodeType::Supply),
            (ChipType::Input,  NodeType::Input),
            (ChipType::Output, NodeType::Output),
        ];
        
        for (target_chip_type, node_type) in type_order {            
            for (id, chip_type) in &circuit.chip_types {
                if chip_type == &target_chip_type {
                    chip_pin_to_id_node.insert(chip_pin!(*id, 0), (new_node_id, node_type));
                    new_node_id += 1;
                }
            }
        }

        new_node_id
    }
    
    // "explode" all chips: add nand nodes for each nand in each chip
    fn add_all_nands(chip_pin_to_id_node: &mut HashMap<ChipAndPin, IDNode>, circuit: &CircuitDescription, num_nodes: usize) -> usize {
        let mut new_node_id = num_nodes;

        for (chip_id, description) in &circuit.chip_descriptions {
            for (old_node_id, node_type) in &description.id_type_map {
                if node_type == &NodeType::NAnd {
                    let chip_and_pin = chip_pin!(*chip_id, *old_node_id);
                    chip_pin_to_id_node.insert(chip_and_pin, (new_node_id, NodeType::NAnd));
                    new_node_id += 1;
                }
            }
        }

        return new_node_id;
    }
    
    // add temporary "buffer" nodes for each ground, supply, input and output in each chip
    fn add_buffer_nodes(
        chip_pin_to_id_node: &mut HashMap<ChipAndPin, (usize, NodeType)>, 
        circuit: &CircuitDescription, 
        num_nodes: usize
    ) {
        let type_order = [ NodeType::Ground, NodeType::Supply, NodeType::Input, NodeType::Output ];
        let mut new_node_id = num_nodes;
        
        for target_chip_type in type_order {
            for (chip_id, description) in &circuit.chip_descriptions {
                let nodes_of_type: Vec<usize> = description.id_type_map.iter()
                    .filter(|(_, node_type)| node_type == &&target_chip_type)
                    .map(|(id, _)| *id)
                    .collect();

                for old_node_id in nodes_of_type {
                    let chip_and_pin = chip_pin!(*chip_id, old_node_id);
                    
                    if chip_pin_to_id_node.contains_key(&chip_and_pin) {
                        panic!("Duplicate chip and pin!?")
                    }

                    chip_pin_to_id_node.insert(chip_and_pin, (new_node_id, NodeType::Buffer));
                    new_node_id += 1;
                }
            }
        }
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
                let source_chip_pin = chip_pin!(*chip_id, *old_source_node_id);
                let (new_source_id, _) = chip_pin_to_id_node.get(&source_chip_pin).expect("Old node not found!");

                let new_target_ids: Vec<usize> = old_target_ids.iter()
                    .map(|old| chip_pin!(*chip_id, *old))
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
                let chip_pin = chip_pin!(*chip_id, *pin);
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
        id_types: &mut HashMap<usize, NodeType>
    ) {
        let unused_buffer_ids: HashSet<usize> = id_types
            .iter()
            .filter(|(_, node_type)| node_type == &&NodeType::Buffer) 
            .map(|(&id, _)| id)
            .filter(|&id| {
                let no_outgoing = forward_links.get(&id).map_or(true, |links| links.is_empty());
                let no_incoming = back_links.get(&id).map_or(true, |links| links.is_empty());
                no_outgoing || no_incoming
            })
            .collect();
        
        if unused_buffer_ids.is_empty() {
            return;
        }
        
        id_types.retain(|id, _| !unused_buffer_ids.contains(id));
        forward_links.retain(|id, _| !unused_buffer_ids.contains(id));
        back_links.retain(|id, _| !unused_buffer_ids.contains(id));
        
        for links in forward_links.values_mut() {
            links.retain(|target_id| !unused_buffer_ids.contains(target_id));
        }
        for links in back_links.values_mut() {
            links.retain(|source_id| !unused_buffer_ids.contains(source_id));
        }
        
        forward_links.retain(|_, links| !links.is_empty());
        back_links.retain(|_, links| !links.is_empty());
    }
    
    // "explode" the remaining buffer nodes: for each buffer source,
    // add a link from each of its ancestors to each of its targets, then delete itself
    // repeat until there are no more buffers
    fn explode_buffers(
        forward_links: &mut HashMap<usize, Vec<usize>>,
        back_links: &mut HashMap<usize, Vec<usize>>,
        id_types: &mut HashMap<usize, NodeType>,
    ) {
        let all_buffers_ids: Vec<usize> = id_types.iter()
            .filter(|(_, node_type)| node_type == &&NodeType::Buffer)
            .map(|(&id, _)| id)
            .collect();
        
        for buffer_id in all_buffers_ids {
            let sources = back_links.remove(&buffer_id).unwrap_or_default();
            let targets = forward_links.remove(&buffer_id).unwrap_or_default();
            
            for &source_id in &sources {
                if let Some(source_fwd_links) = forward_links.get_mut(&source_id) {
                    source_fwd_links.retain(|&id| id != buffer_id);
                    source_fwd_links.extend(targets.iter().filter(|&&t_id| id_types.contains_key(&t_id)));
                }
            }
    
            for &target_id in &targets {
                if let Some(target_back_links) = back_links.get_mut(&target_id) {
                    target_back_links.retain(|&id| id != buffer_id);
                    target_back_links.extend(sources.iter().filter(|&&s_id| id_types.contains_key(&s_id)));
                }
            }
            
            id_types.remove(&buffer_id);
        }
        
        forward_links.retain(|_, v| !v.is_empty());
        back_links.retain(|_, v| !v.is_empty());
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
