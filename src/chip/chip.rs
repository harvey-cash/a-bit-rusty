use crate::chip::chip_description::{ChipDescription, NodeId, NodeType};

use super::Tickable;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChipType {
    Ground,
    Supply,
    Input,
    Output,
    Nand,
}

pub struct Chip {
    description: ChipDescription,
    values: Vec<u8>,
}

impl Chip {
    pub fn new(description: ChipDescription) -> Self {
        if !description.is_valid() {
            panic!("Chip can not be built from invalid description!");
        }

        let num_nodes = description.num_nodes;
        Self {
            description,
            values: vec![0; num_nodes],
        }
    }

    pub fn set_input(&mut self, index: NodeId, value: u8) {
        self.values[index] = value;
    }

    pub fn get_output(&self, output_index: NodeId) -> u8 {
        self.values[self.description.num_inputs + self.description.num_nands + output_index]
    }

    fn nand(&self, index: &NodeId) -> u8 {
        let a_idx = self.description.back_links[index][0];
        let b_idx = self.description.back_links[index][1];
        let a = self.values[a_idx];
        let b = self.values[b_idx];

        if a == 1 && b == 1 { 0 } else { 1 }
    }
}

impl Tickable for Chip {
    fn get_num_components(&self) -> usize {
        self.description.node_types.len()
    }

    fn get_input_ids(&self) -> Vec<usize> {
        (0..self.description.num_inputs).collect()
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

    fn get_forward_links_for(&mut self, index: &usize) -> Option<&Vec<usize>> {
        self.description.forward_links.get(&index)
    }
}
