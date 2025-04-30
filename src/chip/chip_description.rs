
use std::{collections::HashMap, ops::Range};

use super::types::*;

#[derive(Debug, Clone)]
pub struct ChipDescription {
    pub layout: PinLayout,
    pub id_type_map: NodeTypeMap,
    pub forward_links: LinkMap,
    pub back_links: LinkMap,
    num_nands: usize,
    is_valid: bool,
}

impl PartialEq for ChipDescription {
    fn eq(&self, other: &Self) -> bool {
        self.layout == other.layout &&
        self.num_nands == other.num_nands &&
        self.is_valid == other.is_valid &&
        self.id_type_map == other.id_type_map &&
        map_contents_eq_ignore_order(&self.forward_links, &other.forward_links) &&
        map_contents_eq_ignore_order(&self.back_links, &other.back_links)
    }
}
impl Eq for ChipDescription {}

impl ChipDescription {
    pub fn new(id_type_map: NodeTypeMap, links: Vec<Link>) -> Self {

        let forward_links = Self::construct_forward_links(&links);
        let back_links = Self::construct_back_links(&links);

        let num_nodes: usize = id_type_map.len();

        let num_inputs = &id_type_map.iter().filter(|(_, t)| t == &&NodeType::Input).count();
        let num_outputs = &id_type_map.iter().filter(|(_, t)| t == &&NodeType::Output).count();
        let num_nands = &id_type_map.iter().filter(|(_, t)| t == &&NodeType::NAnd).count();

        let mut is_valid = !Self::has_no_ground_nodes(&id_type_map);
        is_valid &= !Self::has_no_supply_nodes(&id_type_map);
        is_valid &= !Self::has_insufficient_nodes(*num_inputs, *num_outputs, &links);
        is_valid &= !Self::any_link_out_of_range(&links, num_nodes);
        is_valid &= !Self::any_link_targets_input(&back_links, &id_type_map);
        is_valid &= !Self::any_link_sources_output(&forward_links, &id_type_map);
        is_valid &= !Self::any_output_targeted_more_than_once(&back_links, &id_type_map);
        is_valid &= !Self::any_node_unconnected(num_nodes, &forward_links, &back_links);
        is_valid &= !Self::any_nand_has_bad_sources(&back_links, &id_type_map);
        is_valid &= !Self::any_nand_has_no_targets(&forward_links, &id_type_map);

        let layout = PinLayout::new(id_type_map.clone());

        Self { layout, id_type_map, forward_links, back_links, num_nands: *num_nands, is_valid }
    }

    pub fn is_valid(&self) -> bool {
        self.is_valid
    }
    
    pub fn get_layout(&self) -> PinLayout {
        self.layout.clone()
    }

    pub fn get_num_nodes(&self) -> usize {
        self.layout.num_pins + self.num_nands
    }

    fn construct_forward_links(links: &Vec<Link>) -> LinkMap {
        let mut forward_links: LinkMap = HashMap::new();

        for link in links {
            forward_links
                .entry(link.source)
                .or_default()
                .push(link.target);
        }

        forward_links
    }

    fn construct_back_links(links: &Vec<Link>) -> LinkMap {
        let mut back_links: LinkMap = HashMap::new();

        for link in links {
            let sources = back_links.entry(link.target).or_default();
            sources.push(link.source);
        }

        back_links
    }

    fn construct_node_types(
        inputs: &Range<usize>,
        outputs: &Range<usize>,
        nands: &Range<usize>,
    ) -> NodeTypeMap {
        inputs
            .clone()
            .map(|i| (i, NodeType::Input))
            .chain(outputs.clone().map(|i| (i, NodeType::Output)))
            .chain(nands.clone().map(|i| (i, NodeType::NAnd)))
            .collect()
    }

    fn create_node_iters(
        num_inputs: usize,
        num_outputs: usize,
        num_nands: usize,
    ) -> (Range<usize>, Range<usize>, Range<usize>) {
        let num_ground_and_supply: usize = 2;
        let end_inputs = num_ground_and_supply + num_inputs;
        let end_outputs = end_inputs + num_outputs;
        let end_nands = end_outputs + num_nands;

        let input_iter = num_ground_and_supply..end_inputs;
        let output_iter = end_inputs..end_outputs;
        let nand_iter = end_outputs..end_nands;

        return (input_iter, output_iter, nand_iter);
    }
    
    fn has_no_ground_nodes(id_type_map: &HashMap<usize, NodeType>) -> bool {
        for (_, node_type) in id_type_map {
            if node_type == &NodeType::Ground {
                return false;
            }
        }
        eprintln!("Has no ground nodes!");
        return true;
    }
    
    fn has_no_supply_nodes(id_type_map: &HashMap<usize, NodeType>) -> bool {
        for (_, node_type) in id_type_map {
            if node_type == &NodeType::Supply {
                return false;
            }
        }
        eprintln!("Has no supply nodes!");
        return true;
    }

    fn any_link_out_of_range(links: &Vec<Link>, num_nodes: usize) -> bool {
        let max_node_index: usize = if num_nodes == 0 { 0 } else { num_nodes - 1 };
        for link in links {
            if link.source > max_node_index || link.target > max_node_index {
                eprintln!("Link {} -> {} out of range!", link.source, link.target);
                return true;
            }
        }
        return false;
    }

    fn any_link_targets_input(back_links: &LinkMap, node_types: &NodeTypeMap) -> bool {
        for (index, _) in back_links {
            if node_types.get(&index).is_none_or(|t| t == &NodeType::Input) {
                eprintln!("Link targets input with id {index}!");
                return true;
            }
        }
        return false;
    }

    fn any_link_sources_output(forward_links: &LinkMap, node_types: &NodeTypeMap) -> bool {
        for (index, _) in forward_links {
            if node_types
                .get(&index)
                .is_none_or(|t| t == &NodeType::Output)
            {
                eprintln!("Link sources output with id {index}!");
                return true;
            }
        }
        return false;
    }

    fn any_node_unconnected(
        num_nodes: usize,
        forward_links: &LinkMap,
        back_links: &LinkMap,
    ) -> bool {
        for index in 2..num_nodes {
            if !forward_links.contains_key(&index) && !back_links.contains_key(&index) {
                eprintln!("Node with id {index} not connected!");
                return true;
            }
        }
        return false;
    }

    fn any_nand_has_bad_sources(back_links: &LinkMap, node_types: &NodeTypeMap) -> bool {
        for (index, node_type) in node_types {
            if node_type != &NodeType::NAnd {
                continue;
            }

            let empty: Vec<usize> = vec![];
            let sources = back_links.get(&index).unwrap_or(&empty);
            if sources.len() != 2 {
                eprintln!("NAnd with id {index} does not have two sources!");
                return true;
            }
            if sources[0] == *index && sources[1] == *index {
                eprintln!("NAnd with id {index} is unconnected and targeting only itself!");
                return true;
            }
        }
        return false;
    }

    fn any_nand_has_no_targets(forward_links: &LinkMap, node_types: &NodeTypeMap) -> bool {
        for (index, node_type) in node_types {
            if node_type != &NodeType::NAnd {
                continue;
            }
            if forward_links.get(index).is_none() {
                eprintln!("NAnd with id {} does not have any targets!", index);
                return true;
            }
        }
        return false;
    }

    fn has_insufficient_nodes(num_inputs: usize, num_outputs: usize, links: &Vec<Link>) -> bool {
        if num_inputs == 0 || num_outputs == 0 || links.len() == 0 {
            eprintln!("Chip must have at least one input, one output, and one link!");
            return true;
        }
        return false;
    }

    fn any_output_targeted_more_than_once(
        back_links: &LinkMap,
        node_types: &NodeTypeMap,
    ) -> bool {
        for (index, node_type) in node_types {
            if node_type != &NodeType::Output {
                continue;
            }

            if back_links.get(&index).is_none_or(|links| links.len() != 1) {
                eprintln!("Output must have exactly one source!");
                return true;
            }
        }
        return false;
    }
}
