use std::collections::HashMap;

enum NodeType {
    Input,
    Output,
    NAnd,
}

pub struct Link {
    pub source: usize,
    pub target: usize,
}

impl Link {
    pub fn new(source: usize, target: usize) -> Self {
        Link { source, target }
    }
}

pub struct Chip {
    num_inputs: usize,
    num_nands: usize,
    num_nodes: usize,
    forward_links: HashMap<usize, Vec<usize>>,
    back_links: HashMap<usize, Vec<usize>>,
    node_types: HashMap<usize, NodeType>,
    values: Vec<u8>,
}

impl Chip {
    pub fn new(num_inputs: usize, num_nands: usize, num_outputs: usize, links: Vec<Link>) -> Self {
        if num_inputs == 0 || num_outputs == 0 || links.len() == 0 {
            panic!("Chip must have at least one input, one output, and one link!")
        }

        let num_nodes: usize = num_inputs + num_nands + num_outputs;
        let max_node_index: usize = num_nodes - 1;

        let link_out_of_range =
            |link: &Link| -> bool { link.source > max_node_index || link.target > max_node_index };

        if links.iter().any(link_out_of_range) {
            panic!("Bad link!")
        }

        let forward_links = Self::construct_forward_links(&links);
        let back_links = Self::construct_back_links(&links);

        let node_types: HashMap<usize, NodeType> =
            Self::construct_node_types(num_inputs, num_nands, num_outputs);
        let values = vec![0; num_nodes];

        Chip {
            num_inputs,
            num_nands,
            num_nodes,
            forward_links,
            back_links,
            node_types,
            values,
        }
    }

    pub fn set_input(&mut self, index: usize, value: u8) {
        self.values[index] = value;
    }

    pub fn update(&mut self) {
        for index in 0..self.num_nodes {      
            let node_type: &NodeType = self.node_types.get(&index).unwrap();

            if matches!(node_type, NodeType::NAnd) {
                self.values[index] = self.nand(&index);
            }
            else if matches!(node_type, NodeType::Output) {
                let source = self.back_links[&index][0];
                self.values[index] = self.values[source];
            }
        }
    }

    pub fn get_output(&self, output_index: usize) -> u8 {
        self.values[self.num_inputs + self.num_nands + output_index]
    }

    fn construct_forward_links(links: &Vec<Link>) -> HashMap<usize, Vec<usize>> {
        let mut forward_links: HashMap<usize, Vec<usize>> = HashMap::new();

        for link in links {
            forward_links
                .entry(link.source)
                .or_default()
                .push(link.target);
        }

        forward_links
    }

    fn construct_back_links(links: &Vec<Link>) -> HashMap<usize, Vec<usize>> {
        let mut back_links: HashMap<usize, Vec<usize>> = HashMap::new();

        for link in links {
            back_links.entry(link.target).or_default().push(link.source);
        }

        back_links
    }

    fn construct_node_types(
        num_inputs: usize,
        num_nands: usize,
        num_outputs: usize,
    ) -> HashMap<usize, NodeType> {
        let end_nands = num_inputs + num_nands;
        let end_outputs = end_nands + num_outputs;

        (0..=num_inputs - 1)
            .map(|i| (i, NodeType::Input))
            .chain((num_inputs..=end_nands - 1).map(|i| (i, NodeType::NAnd)))
            .chain((end_nands..=end_outputs - 1).map(|i| (i, NodeType::Output)))
            .collect()
    }

    fn nand(&self, index: &usize) -> u8 {
        let a_idx = self.back_links[index][0];
        let b_idx = self.back_links[index][1];
        let a = self.values[a_idx];
        let b = self.values[b_idx];

        if a == 1 && b == 1 { 0 } else { 1 }
    }
}
