use serde::{Deserialize, Serialize};

use crate::{chip::chip_description::ChipDescription, link, node_type_map};
use std::collections::{HashMap, HashSet, VecDeque};

use super::types::*;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChipType {
    Ground,
    Supply,
    Input,
    Output,
    Custom,
}

pub trait Tickable: Send + Sync {
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
        PinLayout::new(node_type_map!{0 => NodeType::Output}) 
    }
    fn write_pin(&mut self, _index: usize, _value: u8) {}
    fn read_pin(&self, _index: usize) -> u8 { 0 }
}
impl Tickable for GroundChip { }

pub struct SupplyChip {
    value: u8,
}
impl SupplyChip {
    pub fn new() -> Self { Self { value: 0 } }
    pub fn turn_on(&mut self) { self.value = 1; }
    pub fn turn_off(&mut self) { self.value = 0; }
}
impl Chip for SupplyChip {
    fn get_type(&self) -> ChipType { ChipType::Supply }
    fn get_layout(&self) -> PinLayout {
        PinLayout::new(node_type_map!{0 => NodeType::Output}) 
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
        PinLayout::new(node_type_map!{0 => NodeType::Output}) 
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
        PinLayout::new(node_type_map!{
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
            0 => NodeType::Ground,
            1 => NodeType::Supply,
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
    values: HashMap<usize, u8>,
    changed_since_last_tick: HashSet<usize>
}

impl CustomChip {
    pub fn new(description: ChipDescription) -> Self {
        if !description.is_valid() {
            panic!("Chip can not be built from invalid description!");
        }

        let values = Self::create_id_value_map(&description);

        Self {
            description,
            values,
            changed_since_last_tick: HashSet::new()
        }
    }
    
    pub fn get_description(&self) -> ChipDescription {
        self.description.clone()
    }

    pub fn get_ground_pin(&self) -> usize {
        let ground_ids: Vec<&usize> = self.description.id_type_map.iter()
            .filter_map(|(id, node_type)| { if node_type == &NodeType::Ground { Some(id) } else { None } })
            .collect();
        *ground_ids[0]
    }

    pub fn get_supply_pin(&self) -> usize {
        let supply_ids: Vec<&usize> = self.description.id_type_map.iter()
            .filter_map(|(id, node_type)| { if node_type == &NodeType::Supply { Some(id) } else { None } })
            .collect();
        *supply_ids[0]
    }
    
    fn create_id_value_map(description: &ChipDescription) -> HashMap<usize, u8> {
        let num_nodes = description.get_num_nodes();
        let mut values: HashMap<usize, u8> = HashMap::with_capacity(num_nodes);
        for (id, _) in &description.id_type_map {
            values.insert(*id, 0);
        }
        values
    }

    fn nand(&self, index: &usize) -> u8 {
        let a_idx = self.description.back_links[index][0];
        let b_idx = self.description.back_links[index][1];
        let a = self.values.get(&a_idx).unwrap();
        let b = self.values.get(&b_idx).unwrap();

        if *a == 1 && *b == 1 { 0 } else { 1 }
    }

    fn get_input_ids(&self) -> Vec<usize> {
        let layout = self.description.layout.clone();
        [layout.ground_pins, layout.supply_pins, layout.input_pins].concat()
    }
    
    fn get_input_queue(&self) -> VecDeque<usize> {
        let input_ids: Vec<usize> = self.get_input_ids();
        let mut queue: VecDeque<usize> = VecDeque::with_capacity(input_ids.len());
        for id in input_ids {
            match self.changed_since_last_tick.contains(&id) {
                true => queue.push_front(id),
                false => queue.push_back(id),
            }
        }
        queue
    }

    fn update_node(&mut self, index: &usize) {
        let layout = &self.description.layout;
        if *index < layout.ground_pins.len() + layout.supply_pins.len() {
            return;
        }

        let node_type: &NodeType = self.description.id_type_map.get(index).unwrap();
        if *node_type == NodeType::NAnd {
            let value =  self.nand(&index);
            self.values.insert(*index, value);
        } else if *node_type == NodeType::Output {
            let source = self.description.back_links[&index][0];
            let value = self.values.get(&source).unwrap();
            self.values.insert(*index, *value);
        }
    }

    fn get_forward_links_for(&mut self, index: &usize) -> Option<&Vec<usize>> {
        self.description.forward_links.get(&index)
    }
    
    fn clear_internal_state(&mut self) {
        for (id, node_type) in &self.description.id_type_map {
            if node_type == &NodeType::NAnd || node_type == &NodeType::Output {
                self.values.insert(*id, 0);
            }
        }
    }
    
    fn get_power_input_values(&self) -> (u8, u8) {
        let ground_pin = &self.get_ground_pin();
        let ground = self.values.get(ground_pin).unwrap();

        let supply_pin = &self.get_supply_pin();
        let supply = self.values.get(supply_pin).unwrap();
        
        (*ground, *supply)
    }
}

impl Tickable for CustomChip {
    fn tick(&mut self) {
        let (ground, supply) = self.get_power_input_values();
        if ground != 0 || supply != 1 {
            self.clear_internal_state();
            return;
        }

        let mut updated_this_tick = vec![false; self.description.get_num_nodes()];
        let mut queue: VecDeque<usize> = self.get_input_queue();

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

        self.changed_since_last_tick.clear();
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
            self.values.insert(pin_idx, value);
            self.changed_since_last_tick.insert(pin_idx);
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

        *self.values.get(&pin_idx).unwrap()
    }
}
