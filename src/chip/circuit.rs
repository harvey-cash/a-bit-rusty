use std::{collections::{HashMap, VecDeque}, vec};

use crate::chip_pin;

use super::{
    chip::{Chip, ChipType, CustomChip, Tickable}, circuit_description::CircuitDescription, types::*
};

pub struct Circuit {
    description: CircuitDescription,
    chips: HashMap<usize, Box<dyn Chip>>,
    back_links: HashMap<ChipAndPin, ChipAndPin>,
}

impl Circuit {
    pub fn new() -> Self {
        Self {
            description: CircuitDescription::new(),
            chips: HashMap::new(),
            back_links: HashMap::new(),
        }
    }
    
    pub fn get_description(&self) -> CircuitDescription {
        self.description.clone()
    }

    pub fn add_chip<C: Chip + 'static>(&mut self, chip: C) -> usize {
        let chip_type = chip.get_type();
        if chip_type == ChipType::Custom {
            panic!("Can not add custom chip here - use add_custom_chip!");
        }
        let id = self.description.add_chip(chip_type);
        self.chips.insert(id, Box::new(chip));
        return id;
    }

    pub fn add_custom_chip(&mut self, chip: CustomChip) -> usize {
        let id = self.description.add_custom_chip(chip.get_description());
        self.chips.insert(id, Box::new(chip));
        return id;
    }

    pub fn set_input(&mut self, input_chip_id: usize, value: u8) {
        if self.description.chip_types.get(&input_chip_id) != Some(&ChipType::Input) {
            panic!("Invalid chip ID for input.");
        }
        self.chips.get_mut(&input_chip_id).unwrap().write_pin(0, value);
    }
    
    pub fn get_supply_ids(&self) -> Vec<usize> {
        self.description.chip_types.iter()
            .filter(|(_, chip_type)| chip_type == &&ChipType::Supply)
            .map(|(id, _)| *id)
            .collect()
    }

    pub fn set_supply(&mut self, supply_chip_id: usize, value: u8) {
        if self.description.chip_types.get(&supply_chip_id) != Some(&ChipType::Supply) {
            panic!("Invalid chip ID for supply.");
        }
        let supply_chip = self.chips.get_mut(&supply_chip_id).unwrap();
        supply_chip.write_pin(0, value);
    }

    pub fn get_output(&self, output_index: usize) -> u8 {
        if self.description.chip_types.get(&output_index) != Some(&ChipType::Output) {
            panic!("Invalid chip ID for output.");
        }
        self.chips.get(&output_index).unwrap().read_pin(0)
    }

    pub fn create_link(&mut self, source: ChipAndPin, target: ChipAndPin) {
        self.description.add_forward_link(source, target);
        self.back_links.insert(target, source);
    }

    pub fn delete_link(&mut self, source: ChipAndPin, target: ChipAndPin) {
        self.description.delete_link(source, target);
        
        if !self.back_links.contains_key(&target) {
            panic!("No back link found for target chip and pin.");
        }
        self.back_links.remove(&target);
    }
    
    pub fn get_chip_pin_states(&self) -> HashMap<ChipAndPin, u8> {
        let mut pin_states: HashMap<ChipAndPin, u8> = HashMap::new();
        for (id, chip) in &self.chips {
            for pin in chip.get_layout().ground_pins {
                pin_states.insert(chip_pin!(*id, pin), chip.read_pin(pin));
            }
            for pin in chip.get_layout().supply_pins {
                pin_states.insert(chip_pin!(*id, pin), chip.read_pin(pin));
            }
            for pin in chip.get_layout().input_pins {
                pin_states.insert(chip_pin!(*id, pin), chip.read_pin(pin));
            }
            for pin in chip.get_layout().output_pins {
                pin_states.insert(chip_pin!(*id, pin), chip.read_pin(pin));
            }
        }
        pin_states
    }

    fn get_input_ids(&self) -> Vec<usize> {
        self.description
            .chip_types
            .iter()
            .filter(|(_, chip_type)| chip_type == &&ChipType::Ground || chip_type == &&ChipType::Supply || chip_type == &&ChipType::Input)
            .map(|(&id, _)| id)
            .collect()
    }
    
    fn get_input_values_for_chip(&self, index: &usize) -> HashMap<usize, u8> {
        let layout = self.chips.get(index).unwrap().get_layout();
        let all_inputs: Vec<Vec<usize>> = vec![layout.ground_pins, layout.supply_pins, layout.input_pins];
        let all_pins: Vec<usize> = all_inputs.iter()
            .flat_map(|v| v.iter())
            .map(|v| *v)
            .collect();

        let mut inputs = HashMap::new();
        
        for pin_idx in all_pins {
            let pin = chip_pin!(*index, pin_idx);
            let back_link_option = self.back_links.get(&pin);

            if back_link_option.is_none() {
                inputs.insert(pin_idx, 0);
                continue;
            }
            
            let source_pin: ChipAndPin = *back_link_option.unwrap();
            let value = self.chips.get(&source_pin.chip_id).unwrap().read_pin(source_pin.pin_index);
            inputs.insert(pin_idx, value);
        }
        inputs
        
    }
}

impl Tickable for Circuit {
    fn tick(&mut self) {
        let mut updated_this_tick = vec![false; self.description.num_chips];

        let inputs: Vec<usize> = self.get_input_ids();
        let mut queue: VecDeque<usize> = VecDeque::from(inputs);

        while let Some(index) = queue.pop_front() {
            if updated_this_tick[index] == true {
                continue;
            }

            let input_values: HashMap<usize, u8> = self.get_input_values_for_chip(&index);

            let chip_option = self.chips.get_mut(&index);
            if chip_option.is_none() {
                panic!("Chip with ID {} not found.", index);
            }

            let chip = chip_option.unwrap();
            for (pin_idx, value) in input_values {
                chip.write_pin(pin_idx, value);
            }
            chip.tick();
            updated_this_tick[index] = true;

            let forward_links = self.description.forward_links.get(&index);
            if forward_links.is_none() {
                continue;
            }

            for (_, targets) in forward_links.unwrap() {
                for target in targets {
                    queue.push_back(target.chip_id);
                }
            }
        }

        for (chip_id, chip) in self.chips.iter_mut() {
            if updated_this_tick[*chip_id] == false {
                chip.tick();
            }
        }
    }
}
