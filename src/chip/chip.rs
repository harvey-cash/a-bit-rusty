use std::collections::{HashMap, VecDeque};

type NodeId = usize;

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
    forward_links: HashMap<NodeId, Vec<NodeId>>,
    back_links: HashMap<NodeId, Vec<NodeId>>,
    node_types: HashMap<NodeId, NodeType>,
    values: Vec<u8>,
}

impl Chip {
    pub fn new(num_inputs: usize, num_nands: usize, num_outputs: usize, links: Vec<Link>) -> Self {
        if num_inputs == 0 || num_outputs == 0 || links.len() == 0 {
            panic!("Chip must have at least one input, one output, and one link!")
        }

        let num_nodes: usize = num_inputs + num_nands + num_outputs;
        let max_node_index: NodeId = num_nodes - 1;

        let link_out_of_range =
            |link: &Link| -> bool { link.source > max_node_index || link.target > max_node_index };

        if links.iter().any(link_out_of_range) {
            panic!("Bad link!")
        }

        let node_types: HashMap<NodeId, NodeType> =
            Self::construct_node_types(num_inputs, num_nands, num_outputs);

        let forward_links = Self::construct_forward_links(&links);
        let back_links = Self::construct_back_links(&links, &node_types);

        let values = vec![0; num_nodes];

        Chip {
            num_inputs,
            num_nands,
            forward_links,
            back_links,
            node_types,
            values,
        }
    }

    pub fn set_input(&mut self, index: NodeId, value: u8) {
        self.values[index] = value;
    }

    pub fn update(&mut self) {
        let inputs: Vec<NodeId> = (0..self.num_inputs).collect();
        let mut queue: VecDeque<NodeId> = VecDeque::from(inputs);

        while let Some(index) = queue.pop_front() {
            self.update_node(&index);

            if let Some(targets) = self.forward_links.get(&index) {
                queue.extend(targets.iter().copied());
            }
        }
    }

    pub fn get_output(&self, output_index: NodeId) -> u8 {
        self.values[self.num_inputs + self.num_nands + output_index]
    }

    fn construct_forward_links(links: &Vec<Link>) -> HashMap<NodeId, Vec<NodeId>> {
        let mut forward_links: HashMap<NodeId, Vec<NodeId>> = HashMap::new();

        for link in links {
            forward_links
                .entry(link.source)
                .or_default()
                .push(link.target);
        }

        forward_links
    }

    fn construct_back_links(
        links: &Vec<Link>,
        node_types: &HashMap<NodeId, NodeType>,
    ) -> HashMap<NodeId, Vec<NodeId>> {
        let mut back_links: HashMap<NodeId, Vec<NodeId>> = HashMap::new();

        for link in links {
            let sources = back_links.entry(link.target).or_default();
            sources.push(link.source);

            if let Some(node_type) = node_types.get(&link.target) {
                if *node_type == NodeType::NAnd && sources.len() > 2 {
                    panic!(
                        "Node {} of type NAnd has more than two sources ({} found)",
                        link.target,
                        sources.len()
                    );
                }
            }
        }

        back_links
    }

    fn construct_node_types(
        num_inputs: usize,
        num_nands: usize,
        num_outputs: usize,
    ) -> HashMap<NodeId, NodeType> {
        let end_nands = num_inputs + num_nands;
        let end_outputs = end_nands + num_outputs;

        (0..=num_inputs - 1)
            .map(|i| (i, NodeType::Input))
            .chain((num_inputs..=end_nands - 1).map(|i| (i, NodeType::NAnd)))
            .chain((end_nands..=end_outputs - 1).map(|i| (i, NodeType::Output)))
            .collect()
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
}
