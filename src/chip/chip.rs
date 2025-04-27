use crate::{chip::chip_description::ChipDescription, link, node_type_map};
use std::collections::{HashMap, VecDeque};

use super::types::*;

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
    fn get_layout(&self) -> PinLayout;
    fn get_num_inputs(&self) -> usize { self.get_layout().input_pins.len() }
    fn get_num_outputs(&self) -> usize { self.get_layout().output_pins.len() }
    fn write_pin(&mut self, index: usize, value: u8);
    fn read_pin(&self, index: usize) -> u8;
}

pub struct GroundChip {}
impl GroundChip {
    pub fn new() -> Self { Self {} }
}
impl Chip for GroundChip {
    fn get_type(&self) -> ChipType { ChipType::Ground }
    fn get_layout(&self) -> PinLayout { 
        PinLayout::new(0, 0, node_type_map!{0 => NodeType::Output}) 
    }
    fn write_pin(&mut self, _index: usize, _value: u8) {}
    fn read_pin(&self, _index: usize) -> u8 { 0 }
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
    fn get_layout(&self) -> PinLayout {
        PinLayout::new(0, 0, node_type_map!{0 => NodeType::Output}) 
    }
    fn write_pin(&mut self, _index: usize, value: u8) {
        self.value = value;
    }
    fn read_pin(&self, _index: usize) -> u8 { self.value }
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
    fn get_layout(&self) -> PinLayout { 
        PinLayout::new(0, 0, node_type_map!{0 => NodeType::Output}) 
    }
    fn write_pin(&mut self, _index: usize, value: u8) { self.value = value; }
    fn read_pin(&self, _index: usize) -> u8 { self.value }
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
    fn get_layout(&self) -> PinLayout { 
        PinLayout::new(0, 0, node_type_map!{
            0 => NodeType::Input,
            1 => NodeType::Output
        })
    }
    fn write_pin(&mut self, _index: usize, value: u8) {
        self.input_value = value;
    }
    fn read_pin(&self, _index: usize) -> u8 { self.output_value }
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
        let id_types = node_type_map!{
            2 => NodeType::Input,
            3 => NodeType::Input,
            4 => NodeType::Output,
            5 => NodeType::NAnd
        };
        let links = vec![link!(2 => 5), link!(3 => 5), link!(5 => 4)];
        let description = ChipDescription::new(id_types, links);
        CustomChip::new(description)
    }
}

pub struct CustomChip {
    description: ChipDescription,
    values: Vec<u8>,
}

impl CustomChip {
    pub const GROUND_PIN: usize = 0;
    pub const SUPPLY_PIN: usize = 1;

    pub fn new(description: ChipDescription) -> Self {
        if !description.is_valid() {
            panic!("Chip can not be built from invalid description!");
        }

        let num_nodes = description.get_num_nodes();
        Self {
            description,
            values: vec![0; num_nodes],
        }
    }
    
    pub fn get_description(&self) -> ChipDescription {
        self.description.clone()
    }

    fn nand(&self, index: &usize) -> u8 {
        let a_idx = self.description.back_links[index][0];
        let b_idx = self.description.back_links[index][1];
        let a = self.values[a_idx];
        let b = self.values[b_idx];

        if a == 1 && b == 1 { 0 } else { 1 }
    }

    fn get_input_ids(&self) -> Vec<usize> {
        let layout = self.description.layout.clone();
        [layout.ground_pins, layout.supply_pins, layout.input_pins].concat()
    }

    fn update_node(&mut self, index: &usize) {
        let layout = &self.description.layout;
        if *index < layout.ground_pins.len() + layout.supply_pins.len() {
            return;
        }

        let node_type: &NodeType = self.description.id_type_map.get(index).unwrap();
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
    
    fn clear_internal_state(&mut self) {
        for (id, node_type) in &self.description.id_type_map {
            if node_type == &NodeType::NAnd || node_type == &NodeType::Output {
                self.values[*id] = 0;
            }
        }
    }
}

impl Tickable for CustomChip {
    fn tick(&mut self) {
        
        if self.values[CustomChip::GROUND_PIN] != 0 || self.values[CustomChip::SUPPLY_PIN] != 1 {
            self.clear_internal_state();
            return;
        }

        let mut updated_this_tick = vec![false; self.description.get_num_nodes()];

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

    fn get_layout(&self) -> PinLayout {
        self.description.get_layout()
    }

    fn write_pin(&mut self, pin_idx: usize, value: u8) {
        let num_inputs = self.description.layout.get_num_inputs();
        if pin_idx < num_inputs {
            self.values[pin_idx] = value;
        } else {
            panic!("Can't set pin with index {pin_idx}!");
        }
    }

    fn read_pin(&self, pin_idx: usize) -> u8 {
        let layout = &self.description.layout;
        let num_pins = layout.get_num_inputs() + layout.output_pins.len();
        if pin_idx >= num_pins {
            panic!("Can't read an internal (NAnd) node with index {pin_idx}!")
        }

        self.values[pin_idx]
    }
}
