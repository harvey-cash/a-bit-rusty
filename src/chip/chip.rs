use crate::chip::chip_description::{ChipDescription, NodeId, NodeType};
use std::collections::VecDeque;

use super::chip_description::Link;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChipType {
    Ground,
    Supply,
    Input,
    Output,
    Custom,
}

pub trait Tickable {
    fn tick(&mut self) {}
}

pub trait Chip: Tickable {
    fn get_type(&self) -> ChipType;
    fn get_num_inputs(&self) -> usize;
    fn get_num_outputs(&self) -> usize;
    fn set_input(&mut self, index: usize, value: u8);
    fn get_output(&self, index: usize) -> u8;
}

pub struct GroundChip {}
impl GroundChip {
    pub fn new() -> Self { Self {} }
}
impl Chip for GroundChip {
    fn get_type(&self) -> ChipType { ChipType::Ground }
    fn get_num_inputs(&self) -> usize { 0 }
    fn set_input(&mut self, _index: usize, _value: u8) {}
    fn get_num_outputs(&self) -> usize { 1 }
    fn get_output(&self, _index: usize) -> u8 { 0 }
}
impl Tickable for GroundChip { }

pub struct SupplyChip {
    value: u8,
}
impl SupplyChip {
    pub fn new() -> Self { Self { value: 1 } }
    pub fn turn_on(&mut self) { self.value = 1; }
    pub fn turn_off(&mut self) { self.value = 0; }
}
impl Chip for SupplyChip {
    fn get_type(&self) -> ChipType { ChipType::Supply }
    fn get_num_inputs(&self) -> usize { 0 }
    fn set_input(&mut self, _index: usize, value: u8) {
        self.value = value;
    }
    fn get_num_outputs(&self) -> usize { 1 }
    fn get_output(&self, _index: usize) -> u8 { self.value }
}
impl Tickable for SupplyChip { }

pub struct InputChip {
    value: u8,
}
impl InputChip {
    pub fn new() -> Self { Self { value: 0 } }
}
impl Chip for InputChip {
    fn get_type(&self) -> ChipType { ChipType::Input }
    fn get_num_inputs(&self) -> usize { 0 }
    fn set_input(&mut self, _index: usize, value: u8) { self.value = value; }
    fn get_num_outputs(&self) -> usize { 1 }
    fn get_output(&self, _index: usize) -> u8 { self.value }
}
impl Tickable for InputChip { }

pub struct OutputChip {
    input_value: u8,
    output_value: u8,
}
impl OutputChip {
    pub fn new() -> Self { Self { input_value: 0, output_value: 0 } }
}
impl Chip for OutputChip {
    fn get_type(&self) -> ChipType { ChipType::Output }
    fn get_num_inputs(&self) -> usize { 1 }
    fn set_input(&mut self, _index: usize, value: u8) {
        self.input_value = value;
    }
    fn get_num_outputs(&self) -> usize { 1 }
    fn get_output(&self, _index: usize) -> u8 { self.output_value }
}
impl Tickable for OutputChip { 
    fn tick(&mut self) {
        self.output_value = self.input_value;
        self.input_value = 0;
    }
}

pub struct NAndChip {}
impl NAndChip {
    pub fn new() -> CustomChip {
        let links = vec![Link::new(0, 2), Link::new(1, 2), Link::new(2, 3)];
        let description = ChipDescription::new(2, 1, 1, links);
        CustomChip::new(description)
    }
}

pub struct CustomChip {
    ground: u8,
    supply: u8,
    description: ChipDescription,
    values: Vec<u8>,
}

impl CustomChip {
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
    
    pub fn get_description(&self) -> ChipDescription {
        self.description.clone()
    }

    pub fn set_ground(&mut self, value: u8) {
        self.ground = value;
    }

    pub fn set_supply(&mut self, value: u8) {
        self.supply = value;
    }    

    fn nand(&self, index: &NodeId) -> u8 {
        let a_idx = self.description.back_links[index][0];
        let b_idx = self.description.back_links[index][1];
        let a = self.values[a_idx];
        let b = self.values[b_idx];

        if a == 1 && b == 1 { 0 } else { 1 }
    }
    
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

impl Tickable for CustomChip {
    fn tick(&mut self) {
        let mut updated_this_tick = vec![false; self.get_num_components()];

        let inputs: Vec<usize> = self.get_input_ids();
        let mut queue: VecDeque<usize> = VecDeque::from(inputs);

        while let Some(index) = queue.pop_front() {
            if updated_this_tick[index] == true {
                continue;
            }

            self.update_node(&index);
            updated_this_tick[index] = true;

            if let Some(targets) = self.get_forward_links_for(&index) {
                queue.extend(targets.iter().copied());
            }
        }
    }
}
impl Chip for CustomChip {
    fn get_type(&self) -> ChipType { ChipType::Custom }
    
    fn get_num_inputs(&self) -> usize {
        self.description.num_inputs
    }

    fn set_input(&mut self, index: NodeId, value: u8) {
        self.values[index] = value;
    }

    fn get_num_outputs(&self) -> usize {
        self.description.num_outputs
    }

    fn get_output(&self, output_index: NodeId) -> u8 {
        if self.supply != 1 || self.ground != 0 {
            return 0;
        }
        self.values[self.description.num_inputs + self.description.num_nands + output_index]
    }
}
