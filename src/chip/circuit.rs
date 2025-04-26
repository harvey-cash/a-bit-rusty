use std::{collections::{HashMap, VecDeque}, vec};

use super::{chip_description::ChipAndPin, Chip, ChipDescription, ChipType, CircuitDescription, Tickable};

pub struct Circuit {
    description: CircuitDescription,
    chips: HashMap<usize, Box<dyn Chip>>,
    back_links: HashMap<ChipAndPin, ChipAndPin>,
}

impl Circuit {
    pub fn new(description: CircuitDescription) -> Self {
        Self {
            description,
            chips: HashMap::new(),
            back_links: HashMap::new(),
        }
    }

    pub fn add_chip<C: Chip + 'static>(&mut self, chip: C) -> usize {
        let chip_type = chip.get_type();        
        let id = self.description.add_chip(chip_type);
        self.chips.insert(id, Box::new(chip));
        return id;
    }

    pub fn get_description(&self) -> &CircuitDescription {
        &self.description
    }

    pub fn compile_to_chip(&self) -> ChipDescription {
        self.description.compile_to_chip()
    }

    pub fn set_input(&mut self, input_chip_id: usize, value: u8) {
        if self.description.chips.get(&input_chip_id) != Some(&ChipType::Input) {
            panic!("Invalid chip ID for input.");
        }
        self.chips.get_mut(&input_chip_id).unwrap().set_input(0, value);
    }

    pub fn set_supply(&mut self, supply_chip_id: usize, value: u8) {
        if self.description.chips.get(&supply_chip_id) != Some(&ChipType::Supply) {
            panic!("Invalid chip ID for supply.");
        }
        let supply_chip = self.chips.get_mut(&supply_chip_id).unwrap();
        supply_chip.set_input(0, value);
    }

    pub fn get_output(&self, output_index: usize) -> u8 {
        if self.description.chips.get(&output_index) != Some(&ChipType::Output) {
            panic!("Invalid chip ID for output.");
        }
        self.chips.get(&output_index).unwrap().get_output(0)
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

    fn get_input_ids(&self) -> Vec<usize> {
        self.description
            .chips
            .iter()
            .filter(|(_, chip_type)| chip_type == &&ChipType::Ground || chip_type == &&ChipType::Supply || chip_type == &&ChipType::Input)
            .map(|(&id, _)| id)
            .collect()
    }
    
    fn get_input_values_for_chip(&self, index: &usize) -> Vec<u8> {
        let num_inputs = self.chips.get(index).unwrap().get_num_inputs();
        let mut inputs = vec![0; num_inputs];
        
        for pin_idx in 0..num_inputs {
            let pin = ChipAndPin::new(*index, pin_idx);
            let back_link_option = self.back_links.get(&pin);

            if back_link_option.is_none() {
                inputs[pin_idx] = 0;
                continue;
            }
            
            let source_pin: ChipAndPin = *back_link_option.unwrap();
            inputs[pin_idx] = self.chips.get(&source_pin.chip_id).unwrap().get_output(source_pin.pin_index);
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

            let input_values = self.get_input_values_for_chip(&index);

            let chip_option = self.chips.get_mut(&index);
            if chip_option.is_none() {
                panic!("Chip with ID {} not found.", index);
            }

            let chip = chip_option.unwrap();
            for (i, value) in input_values.iter().enumerate() {
                chip.set_input(i, *value);
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
