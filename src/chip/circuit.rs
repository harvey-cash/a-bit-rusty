use std::{collections::{HashMap, VecDeque}, hash::Hash, vec};

use super::{Chip, ChipType, Tickable};

pub struct CircuitDescription {
    pub num_chips: usize,
    pub chips: HashMap<usize, ChipType>,
}

impl CircuitDescription {
    pub fn new() -> Self {
        Self {
            num_chips: 0,
            chips: HashMap::new(),
        }
    }

    pub fn add_chip(&mut self, chip_type: ChipType) -> usize {
        let id = self.num_chips;
        self.num_chips += 1;
        self.chips.entry(id).or_insert(chip_type);
        id
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChipAndPin {
    chip_id: usize,
    pin_index: usize,
}
impl ChipAndPin {
    pub fn new(chip_id: usize, pin_index: usize) -> Self {
        Self { chip_id, pin_index }
    }
}

pub struct Circuit {
    description: CircuitDescription,
    chips: HashMap<usize, Box<dyn Chip>>,
    forward_links: HashMap<usize, HashMap<ChipAndPin, Vec<ChipAndPin>>>,
    back_links: HashMap<ChipAndPin, ChipAndPin>,
}

impl Circuit {
    pub fn new(description: CircuitDescription) -> Self {
        Self {
            description,
            chips: HashMap::new(),
            forward_links: HashMap::new(),
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

    pub fn set_input(&mut self, input_chip_id: usize, value: u8) {
        if self.description.chips.get(&input_chip_id) != Some(&ChipType::Input) {
            panic!("Invalid chip ID for input.");
        }
        self.chips.get_mut(&input_chip_id).unwrap().set_input(0, value);
    }

    pub fn get_output(&self, output_index: usize) -> u8 {
        if self.description.chips.get(&output_index) != Some(&ChipType::Output) {
            panic!("Invalid chip ID for output.");
        }
        self.chips.get(&output_index).unwrap().get_output(0)
    }

    pub fn create_link(&mut self, source: ChipAndPin, target: ChipAndPin) {
        let chip_id = source.chip_id;
        self.forward_links
            .entry(chip_id)
            .or_insert_with(HashMap::new)
            .entry(source)
            .or_insert_with(Vec::new)
            .push(target);

        self.back_links.insert(target, source);
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

            let forward_links = self.forward_links.get(&index);
            if forward_links.is_none() {
                continue;
            }

            for (_, targets) in forward_links.unwrap() {
                for target in targets {
                    queue.push_back(target.chip_id);
                }
            }
        }
    }
}
