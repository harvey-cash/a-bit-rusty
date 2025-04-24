use std::collections::HashMap;

use super::{ChipType, Tickable};

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

pub struct Circuit {
    description: CircuitDescription,
    values: Vec<u8>,
    forward_links: HashMap<usize, Vec<usize>>,
    back_links: HashMap<usize, Vec<usize>>,
}

impl Circuit {
    pub fn new() -> Self {
        Self {
            description: CircuitDescription::new(),
            values: vec![],
            forward_links: HashMap::new(),
            back_links: HashMap::new(),
        }
    }

    pub fn add_chip(&mut self, chip_type: ChipType) -> usize {
        let id = self.description.add_chip(chip_type);
        let value = match chip_type {
            ChipType::Ground => 0,
            ChipType::Supply => 1,
            ChipType::Input => 0,
            ChipType::Output => 0,
            ChipType::Nand => 0,
        };
        self.values.push(value);
        id
    }

    pub fn get_description(&self) -> &CircuitDescription {
        &self.description
    }

    pub fn set_input(&mut self, input_chip_id: usize, value: u8) {
        if self.description.chips.get(&input_chip_id) == Some(&ChipType::Input) {
            self.values[input_chip_id] = value;
        } else {
            panic!("Invalid chip ID for input.");
        }
    }

    pub fn get_output(&self, output_index: usize) -> u8 {
        self.values[output_index]
    }

    pub fn create_link(&mut self, source_chip_id: usize, target_chip_id: usize) {
        self.forward_links
            .entry(source_chip_id)
            .or_insert_with(Vec::new)
            .push(target_chip_id);
        self.back_links
            .entry(target_chip_id)
            .or_insert_with(Vec::new)
            .push(source_chip_id);
    }

    fn nand(&self, index: &usize) -> u8 {
        let a_idx = self.back_links[index][0];
        let b_idx = self.back_links[index][1];
        let a = self.values[a_idx];
        let b = self.values[b_idx];

        if a == 1 && b == 1 { 0 } else { 1 }
    }
}

impl Tickable for Circuit {
    fn get_num_components(&self) -> usize {
        self.description.num_chips
    }

    fn get_input_ids(&self) -> Vec<usize> {
        self.description
            .chips
            .iter()
            .filter(|(_, chip_type)| chip_type == &&ChipType::Input)
            .map(|(&id, _)| id)
            .collect()
    }

    fn update_node(&mut self, index: &usize) {
        let chip_type: &ChipType = self.description.chips.get(index).unwrap();
        if *chip_type == ChipType::Nand {
            self.values[*index] = self.nand(&index);
        } else if *chip_type == ChipType::Output {
            let source = self.back_links[&index][0];
            self.values[*index] = self.values[source];
        }
    }

    fn get_forward_links_for(&mut self, index: &usize) -> Option<&Vec<usize>> {
        self.forward_links.get(&index)
    }
}
