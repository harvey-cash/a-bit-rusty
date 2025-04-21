use crate::chip::chip_description::{NodeId, NodeType, ChipDescription};

use std::collections::VecDeque;

pub struct Chip {
    description: ChipDescription,
    values: Vec<u8>,
    updated_this_tick: Vec<bool>,
}

impl Chip {
    pub fn new(description: ChipDescription) -> Self {
        let num_nodes = description.num_nodes;
        Self {
            description,
            values: vec![0; num_nodes],
            updated_this_tick: vec![false; num_nodes],
        }
    }

    pub fn set_input(&mut self, index: NodeId, value: u8) {
        self.values[index] = value;
    }

    pub fn tick(&mut self) {
        let num_nodes = self.description.node_types.len();
        self.updated_this_tick = vec![false; num_nodes];

        let inputs: Vec<NodeId> = (0..self.description.num_inputs).collect();
        let mut queue: VecDeque<NodeId> = VecDeque::from(inputs);

        while let Some(index) = queue.pop_front() {
            if self.updated_this_tick[index] == true {
                continue;
            }

            self.update_node(&index);
            self.updated_this_tick[index] = true;

            if let Some(targets) = self.description.forward_links.get(&index) {
                queue.extend(targets.iter().copied());
            }
        }
    }

    pub fn get_output(&self, output_index: NodeId) -> u8 {
        self.values[self.description.num_inputs + self.description.num_nands + output_index]
    }

    fn update_node(&mut self, index: &NodeId) {
        let node_type: &NodeType = self.description.node_types.get(&index).unwrap();
        if *node_type == NodeType::NAnd {
            self.values[*index] = self.nand(&index);
        } else if *node_type == NodeType::Output {
            let source = self.description.back_links[&index][0];
            self.values[*index] = self.values[source];
        }
    }

    fn nand(&self, index: &NodeId) -> u8 {
        let a_idx = self.description.back_links[index][0];
        let b_idx = self.description.back_links[index][1];
        let a = self.values[a_idx];
        let b = self.values[b_idx];

        if a == 1 && b == 1 { 0 } else { 1 }
    }
}
