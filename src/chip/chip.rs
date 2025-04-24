use crate::chip::chip_description::{ChipDescription, NodeId, NodeType};

use super::Tickable;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChipType {
    Ground,
    Supply,
    Input,
    Output,
    Nand,
    Custom,
}

pub struct GroundChip {}
impl GroundChip {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_output(&self, _index: u8) -> u8 {
        0
    }
}

pub struct SupplyChip {
    value: u8,
}
impl SupplyChip {
    pub fn new() -> Self {
        Self { value: 1 }
    }

    pub fn turn_on(&mut self) {
        self.value = 1;
    }

    pub fn turn_off(&mut self) {
        self.value = 0;
    }

    pub fn get_output(&self, _index: u8) -> u8 {
        self.value
    }
}

pub struct NAndChip {
    values: Vec<u8>,
}

impl NAndChip {
    pub fn new() -> Self {
        Self { values: vec![0; 3] }
    }

    pub fn set_input(&mut self, index: NodeId, value: u8) {
        self.values[index] = value;
    }

    pub fn get_output(&self, _output_index: NodeId) -> u8 {
        self.values[2]
    }
}

impl Tickable for NAndChip {
    fn get_num_components(&self) -> usize {
        0
    }

    fn get_input_ids(&self) -> Vec<usize> {
        vec![]
    }

    fn tick(&mut self) {
        if self.values[0] == 1 && self.values[1] == 1 {
            self.values[2] = 0;
        } else {
            self.values[2] = 1;
        }
    }

    fn get_forward_links_for(&mut self, _index: &usize) -> Option<&Vec<usize>> {
        Option::None
    }

    fn update_node(&mut self, _index: &usize) {}
}

pub struct Chip {
    ground: u8,
    supply: u8,
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
            ground: 0,
            supply: 1,
            description,
            values: vec![0; num_nodes],
        }
    }

    pub fn set_ground(&mut self, value: u8) {
        self.ground = value;
    }

    pub fn set_supply(&mut self, value: u8) {
        self.supply = value;
    }

    pub fn set_input(&mut self, index: NodeId, value: u8) {
        self.values[index] = value;
    }

    pub fn get_output(&self, output_index: NodeId) -> u8 {
        if self.supply != 1 || self.ground != 0 {
            return 0;
        }
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
