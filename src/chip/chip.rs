use std::{collections::{HashMap, VecDeque}, ops::Range};

type NodeId = usize;
type LinkMap = HashMap<NodeId, Vec<NodeId>>;
type NodeTypeMap = HashMap<NodeId, NodeType>;

#[derive(PartialEq, Debug)]
enum NodeType {
    Input,
    Output,
    NAnd,
}

pub struct Link {
    pub source: NodeId,
    pub target: NodeId,
}

impl Link {
    pub fn new(source: NodeId, target: NodeId) -> Self {
        Link { source, target }
    }
}

pub struct Chip {
    num_inputs: usize,
    num_nands: usize,
    forward_links: LinkMap,
    back_links: LinkMap,
    node_types: NodeTypeMap,
    values: Vec<u8>,
    updated_this_tick: Vec<bool>
}

impl Chip {
    pub fn new(num_inputs: usize, num_nands: usize, num_outputs: usize, links: Vec<Link>) -> Self {
        Self::panic_if_insufficient_nodes(num_inputs, num_outputs, &links);

        let (input_iter, nand_iter, output_iter) = 
            Self::create_node_iters(num_inputs, num_nands, num_outputs);
        let node_types: NodeTypeMap = 
            Self::construct_node_types(&input_iter, &nand_iter, &output_iter);

        let forward_links = Self::construct_forward_links(&links);
        let back_links = Self::construct_back_links(&links);

        let num_nodes: usize = num_inputs + num_nands + num_outputs;
        
        Self::panic_if_any_link_out_of_range(&links, num_nodes);
        Self::panic_if_any_link_targets_input(&back_links, &node_types);
        Self::panic_if_any_link_sources_output(&forward_links, &node_types);
        Self::panic_if_any_output_targeted_more_than_once(&back_links, &output_iter);
        Self::panic_if_any_node_unconnected(num_nodes, &forward_links, &back_links);
        Self::panic_if_any_nand_has_bad_sources(&back_links, &nand_iter);
        Self::panic_if_any_nand_has_no_targets(&forward_links, &nand_iter);

        let values = vec![0; num_nodes];
        let updated_this_tick: Vec<bool> = vec![false; num_nodes];

        Chip {
            num_inputs,
            num_nands,
            forward_links,
            back_links,
            node_types,
            values,
            updated_this_tick
        }
    }

    pub fn set_input(&mut self, index: NodeId, value: u8) {
        self.values[index] = value;
    }

    pub fn update(&mut self) {
        let inputs: Vec<NodeId> = (0..self.num_inputs).collect();
        let mut queue: VecDeque<NodeId> = VecDeque::from(inputs);

        while let Some(index) = queue.pop_front() {
            if self.updated_this_tick[index] == true {
                continue;
            }

            self.update_node(&index);
            self.updated_this_tick[index] = true;

            if let Some(targets) = self.forward_links.get(&index) {
                queue.extend(targets.iter().copied());
            }
        }
    }

    pub fn get_output(&self, output_index: NodeId) -> u8 {
        self.values[self.num_inputs + self.num_nands + output_index]
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

    fn construct_node_types(inputs: &Range<usize>, nands: &Range<usize>, outputs: &Range<usize>) -> NodeTypeMap {
        inputs.clone().map(|i| (i, NodeType::Input))
            .chain(nands.clone().map(|i| (i, NodeType::NAnd)))
            .chain(outputs.clone().map(|i| (i, NodeType::Output)))
            .collect()
    }

    fn create_node_iters(
        num_inputs: usize,
        num_nands: usize,
        num_outputs: usize
    ) -> (Range<usize>, Range<usize>, Range<usize>) {
        let end_nands = num_inputs + num_nands;
        let end_outputs = end_nands + num_outputs;

        let input_iter = 0..num_inputs;
        let nand_iter = num_inputs..end_nands;
        let output_iter = end_nands..end_outputs;

        return (input_iter, nand_iter, output_iter)
    }

    fn update_node(&mut self, index: &NodeId) {
        let node_type: &NodeType = self.node_types.get(&index).unwrap();
        if *node_type == NodeType::NAnd {
            self.values[*index] = self.nand(&index);
        } else if *node_type == NodeType::Output {
            let source = self.back_links[&index][0];
            self.values[*index] = self.values[source];
        }
    }

    fn nand(&self, index: &NodeId) -> u8 {
        let a_idx = self.back_links[index][0];
        let b_idx = self.back_links[index][1];
        let a = self.values[a_idx];
        let b = self.values[b_idx];

        if a == 1 && b == 1 { 0 } else { 1 }
    }

    fn panic_if_any_link_out_of_range(links: &Vec<Link>, num_nodes: usize) {
        let max_node_index: NodeId = num_nodes - 1;
        
        for link in links {
            if link.source > max_node_index || link.target > max_node_index { 
                panic!("Link {} -> {} out of range!", link.source, link.target) 
            }
        }
    }

    fn panic_if_any_link_targets_input(back_links: &LinkMap, node_types: &NodeTypeMap) {
        for (index, _) in back_links {
            if node_types.get(&index).is_none_or(|t| t == &NodeType::Input) {
                panic!("Link targets input with id {}!", index)
            }            
        }
    }

    fn panic_if_any_link_sources_output(forward_links: &LinkMap, node_types: &NodeTypeMap) {
        for (index, _) in forward_links {
            if node_types.get(&index).is_none_or(|t| t == &NodeType::Output) {
                panic!("Link sources output with id {}!", index)
            }            
        }
    }

    fn panic_if_any_node_unconnected(num_nodes: usize, forward_links: &LinkMap, back_links: &LinkMap) {
        for index in 0..num_nodes {
            if !forward_links.contains_key(&index) && !back_links.contains_key(&index) {
                panic!("Node with id {} not connected!", index)
            }
        }
    }

    fn panic_if_any_nand_has_bad_sources(back_links: &LinkMap, nand_iter: &Range<usize>) {
        for index in nand_iter.clone() {
            if back_links.get(&index).unwrap_or(&vec![]).len() != 2 {
                panic!("NAnd with id {} does not have two sources!", index)
            }
        }        
    }

    fn panic_if_any_nand_has_no_targets(forward_links: &LinkMap, nand_iter: &Range<usize>) {
        for index in nand_iter.clone() {
            if forward_links.get(&index).is_none() {
                panic!("NAnd with id {} does not have any targets!", index)
            }
        }
    }
    
    fn panic_if_insufficient_nodes(num_inputs: usize, num_outputs: usize, links: &Vec<Link>) {
        if num_inputs == 0 || num_outputs == 0 || links.len() == 0 {
            panic!("Chip must have at least one input, one output, and one link!")
        }
    }
    
    fn panic_if_any_output_targeted_more_than_once(back_links: &HashMap<usize, Vec<usize>>, output_iter: &Range<usize>) {
        for index in output_iter.clone() {
            if back_links.get(&index).is_none_or(|links| links.len() != 1) {
                panic!("Output with id {} must be targeted exactly once!", index)
            }
        }
    }
}
