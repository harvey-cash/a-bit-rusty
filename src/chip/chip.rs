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
    forward_links: HashMap<usize, Vec<usize>>,
    back_links: HashMap<usize, Vec<usize>>,
    node_types: HashMap<usize, NodeType>,
    values: Vec<u8>,
}

impl Chip {
    pub fn new(num_inputs: usize, num_outputs: usize, num_nands: usize, links: Vec<Link>) -> Self {
        if num_inputs == 0 || num_outputs == 0 || links.len() == 0 {
            panic!("Chip must have at least one input, one output, and one link!")
        }

        let num_nodes: usize = num_inputs + num_outputs + num_nands;
        let max_node_index: usize = num_nodes - 1;

        let link_out_of_range =
            |link: &Link| -> bool { link.source > max_node_index || link.target > max_node_index };

        if links.iter().any(link_out_of_range) {
            panic!("Bad link!")
        }

        let forward_links = Self::get_forward_links(&links);
        let back_links = Self::get_back_links(&links);

        let node_types: HashMap<usize, NodeType> =
            Self::get_node_types(num_inputs, num_outputs, num_nands);
        let values = vec![0; num_nodes];

        Chip {
            num_inputs,
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
        for (index, node_type) in self.node_types.iter() {
            if matches!(node_type, NodeType::Input) {
                for target in self.forward_links[index].iter() {
                    self.values[*target] = self.values[*index];
                }
            } else if matches!(node_type, NodeType::NAnd) {
                let target = self.forward_links[index][0];
                self.values[target] = self.nand(index);
            }
        }
    }

    pub fn get_output(&self, output_index: usize) -> u8 {
        self.values[self.num_inputs + output_index]
    }

    fn get_forward_links(links: &Vec<Link>) -> HashMap<usize, Vec<usize>> {
        let mut forward_links: HashMap<usize, Vec<usize>> = HashMap::new();

        for link in links {
            forward_links
                .entry(link.source)
                .or_default()
                .push(link.target);
        }

        forward_links
    }

    fn get_back_links(links: &Vec<Link>) -> HashMap<usize, Vec<usize>> {
        let mut back_links: HashMap<usize, Vec<usize>> = HashMap::new();

        for link in links {
            back_links.entry(link.target).or_default().push(link.source);
        }

        back_links
    }

    fn get_node_types(
        num_inputs: usize,
        num_outputs: usize,
        num_nands: usize,
    ) -> HashMap<usize, NodeType> {
        let end_outputs = num_inputs + num_outputs;
        let end_nands = end_outputs + num_nands;

        (0..=num_inputs - 1)
            .map(|i| (i, NodeType::Input))
            .chain((num_inputs..=end_outputs - 1).map(|i| (i, NodeType::Output)))
            .chain((end_outputs..=end_nands - 1).map(|i| (i, NodeType::NAnd)))
            .collect()
    }

    fn nand(&self, index: &usize) -> u8 {
        let a_idx = self.back_links[index][0];
        let b_idx = self.back_links[index][0];
        let a = self.values[a_idx];
        let b = self.values[b_idx];

        if a == 1 && b == 1 { 0 } else { 1 }
    }
}
