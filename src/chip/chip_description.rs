
use std::{collections::HashMap, ops::Range};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChipAndPin {
    pub chip_id: usize,
    pub pin_index: usize,
}
impl ChipAndPin {
    pub fn new(chip_id: usize, pin_index: usize) -> Self {
        Self { chip_id, pin_index }
    }
}

#[derive(PartialEq, Debug)]
pub enum NodeType {
    Input,
    Output,
    NAnd,
}

pub type NodeId = usize;
pub type LinkMap = HashMap<NodeId, Vec<NodeId>>;
pub type NodeTypeMap = HashMap<NodeId, NodeType>;

pub struct Link {
    pub source: NodeId,
    pub target: NodeId,
}


impl Link {
    pub fn new(source: NodeId, target: NodeId) -> Self {
        Link { source, target }
    }
}

pub struct ChipDescription {
    pub num_nodes: usize,
    pub num_inputs: usize,
    pub num_nands: usize,
    pub num_outputs: usize,
    pub node_types: NodeTypeMap,
    pub forward_links: LinkMap,
    pub back_links: LinkMap,

    is_valid: bool,
}

impl ChipDescription {
    pub fn new(num_inputs: usize, num_nands: usize, num_outputs: usize, links: Vec<Link>) -> Self {

        let (input_iter, nand_iter, output_iter) =
            Self::create_node_iters(num_inputs, num_nands, num_outputs);
        let node_types: NodeTypeMap =
            Self::construct_node_types(&input_iter, &nand_iter, &output_iter);

        let forward_links = Self::construct_forward_links(&links);
        let back_links = Self::construct_back_links(&links);

        let num_nodes: usize = num_inputs + num_nands + num_outputs;

        let mut is_valid = !Self::has_insufficient_nodes(num_inputs, num_outputs, &links);
        is_valid &= !Self::any_link_out_of_range(&links, num_nodes);
        is_valid &= !Self::any_link_targets_input(&back_links, &node_types);
        is_valid &= !Self::any_link_sources_output(&forward_links, &node_types);
        is_valid &= !Self::any_output_targeted_more_than_once(&back_links, &output_iter);
        is_valid &= !Self::any_node_unconnected(num_nodes, &forward_links, &back_links);
        is_valid &= !Self::any_nand_has_bad_sources(&back_links, &nand_iter);
        is_valid &= !Self::any_nand_has_no_targets(&forward_links, &nand_iter);

        Self { num_nodes, num_inputs, num_nands, num_outputs, node_types, forward_links, back_links, is_valid }
    }

    pub fn is_valid(&self) -> bool {
        self.is_valid
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
        let end_nands = num_inputs + num_nands;
        let end_outputs = end_nands + num_outputs;

        let input_iter = 0..num_inputs;
        let nand_iter = num_inputs..end_nands;
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
        for index in 0..num_nodes {
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
        back_links: &HashMap<usize, Vec<usize>>,
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