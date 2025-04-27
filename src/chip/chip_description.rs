
use std::{collections::HashMap, ops::Range};

use super::types::*;

fn link_maps_equal_ignore_vec_order(map1: &LinkMap, map2: &LinkMap) -> bool {
    if map1.len() != map2.len() {
        return false;
    }
    for (key, vec1) in map1 {
        match map2.get(key) {
            None => return false,
            Some(vec2) => {
                if vec1.len() != vec2.len() {
                    return false;
                }
                let mut sorted_vec1 = vec1.clone();
                let mut sorted_vec2 = vec2.clone();
                
                sorted_vec1.sort_unstable();
                sorted_vec2.sort_unstable();
                
                if sorted_vec1 != sorted_vec2 {
                    return false;
                }
            }
        }
    }
    true
}

#[derive(Debug, Clone)]
pub struct ChipDescription {
    pub num_nodes: usize,
    pub num_inputs: usize,
    pub num_nands: usize,
    pub num_outputs: usize,
    pub node_types: NodeTypeMap,
    pub forward_links: LinkMap,
    pub back_links: LinkMap,
    pub layout: PinLayout,

    is_valid: bool,
}

impl PartialEq for ChipDescription {
    fn eq(&self, other: &Self) -> bool {
        self.num_nodes == other.num_nodes &&
        self.num_inputs == other.num_inputs &&
        self.num_nands == other.num_nands &&
        self.num_outputs == other.num_outputs &&
        self.is_valid == other.is_valid &&
        self.node_types == other.node_types &&
        link_maps_equal_ignore_vec_order(&self.forward_links, &other.forward_links) &&
        link_maps_equal_ignore_vec_order(&self.back_links, &other.back_links)
    }
}
impl Eq for ChipDescription {}

impl ChipDescription {
    pub fn new(num_inputs: usize, num_nands: usize, num_outputs: usize, links: Vec<Link>) -> Self {

        let (input_iter, nand_iter, output_iter) =
            Self::create_node_iters(num_inputs, num_nands, num_outputs);
        let node_types: NodeTypeMap =
            Self::construct_node_types(&input_iter, &nand_iter, &output_iter);

        let forward_links = Self::construct_forward_links(&links);
        let back_links = Self::construct_back_links(&links);

        let num_nodes: usize = 2 + num_inputs + num_nands + num_outputs;

        let mut is_valid = !Self::has_insufficient_nodes(num_inputs, num_outputs, &links);
        is_valid &= !Self::any_link_out_of_range(&links, num_nodes);
        is_valid &= !Self::any_link_targets_input(&back_links, &node_types);
        is_valid &= !Self::any_link_sources_output(&forward_links, &node_types);
        is_valid &= !Self::any_output_targeted_more_than_once(&back_links, &output_iter);
        is_valid &= !Self::any_node_unconnected(num_nodes, &forward_links, &back_links);
        is_valid &= !Self::any_nand_has_bad_sources(&back_links, &nand_iter);
        is_valid &= !Self::any_nand_has_no_targets(&forward_links, &nand_iter);

        let layout = PinLayout::new(1, 1, num_inputs, num_outputs);

        Self { num_nodes, num_inputs, num_nands, num_outputs, node_types, forward_links, back_links, layout, is_valid }
    }

    pub fn is_valid(&self) -> bool {
        self.is_valid
    }
    
    pub fn get_layout(&self) -> PinLayout {
        self.layout.clone()
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
        nands: &Range<usize>,
        outputs: &Range<usize>,
    ) -> NodeTypeMap {
        inputs
            .clone()
            .map(|i| (i, NodeType::Input))
            .chain(nands.clone().map(|i| (i, NodeType::NAnd)))
            .chain(outputs.clone().map(|i| (i, NodeType::Output)))
            .collect()
    }

    fn create_node_iters(
        num_inputs: usize,
        num_nands: usize,
        num_outputs: usize,
    ) -> (Range<usize>, Range<usize>, Range<usize>) {
        let num_ground_and_supply: usize = 2;
        let end_inputs = num_ground_and_supply + num_inputs;
        let end_nands = end_inputs + num_nands;
        let end_outputs = end_nands + num_outputs;

        let input_iter = num_ground_and_supply..end_inputs;
        let nand_iter = end_inputs..end_nands;
        let output_iter = end_nands..end_outputs;

        return (input_iter, nand_iter, output_iter);
    }

    fn any_link_out_of_range(links: &Vec<Link>, num_nodes: usize) -> bool {
        let max_node_index: NodeId = if num_nodes == 0 { 0 } else { num_nodes - 1 };
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

    fn any_nand_has_bad_sources(back_links: &LinkMap, nand_iter: &Range<usize>) -> bool {
        for index in nand_iter.clone() {
            let empty: Vec<usize> = vec![];
            let sources = back_links.get(&index).unwrap_or(&empty);
            if sources.len() != 2 {
                eprintln!("NAnd with id {index} does not have two sources!");
                return true;
            }
            if sources[0] == index && sources[1] == index {
                eprintln!("NAnd with id {index} is unconnected and targeting only itself!");
                return true;
            }
        }
        return false;
    }

    fn any_nand_has_no_targets(forward_links: &LinkMap, nand_iter: &Range<usize>) -> bool {
        for index in nand_iter.clone() {
            if forward_links.get(&index).is_none() {
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
        output_iter: &Range<usize>,
    ) -> bool {
        for index in output_iter.clone() {
            if back_links.get(&index).is_none_or(|links| links.len() != 1) {
                eprintln!("Output must have exactly one source!");
                return true;
            }
        }
        return false;
    }
}