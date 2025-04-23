use std::collections::{HashMap, VecDeque};

pub type ChipID = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChipType {
    Ground,
    Supply,
    Input,
    Output,
    Nand,
}

pub struct ChipLayout {
    pub input_pins: Vec<usize>,
    pub output_pins: Vec<usize>,
}

pub struct CircuitDescription {
    pub num_chips: usize,
    pub chips: HashMap<ChipID, ChipType>
}

impl CircuitDescription {
    pub fn new() -> Self {
        Self { num_chips: 0, chips: HashMap::new() }
    }

    pub fn add_chip(&mut self, chip_type: ChipType) -> ChipID {
        let id = self.num_chips;
        self.num_chips += 1;
        self.chips.entry(id).or_insert(chip_type);
        id
    }
}

pub struct Circuit {
    description: CircuitDescription,
    values: Vec<u8>,
    forward_links: HashMap<ChipID, Vec<ChipID>>,
    back_links: HashMap<ChipID, Vec<ChipID>>,
    updated_this_tick: Vec<bool>,
}

impl Circuit {
    pub fn new() -> Self {
        Self { 
            description: CircuitDescription::new(), 
            values: vec![], 
            forward_links: HashMap::new(),
            back_links: HashMap::new(),
            updated_this_tick: vec![],
        }
    }
    
    pub fn add_chip(&mut self, chip_type: ChipType) -> ChipID {
        let id = self.description.add_chip(chip_type);
        let value = match chip_type {
            ChipType::Ground => 0,
            ChipType::Supply => 1,
            ChipType::Input => 0,
            ChipType::Output => 0,
            ChipType::Nand => 0,
        };
        self.values.push(value);
        self.updated_this_tick.push(false);
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

    pub fn tick(&mut self) {
        let num_nodes = self.description.num_chips;
        self.updated_this_tick = vec![false; num_nodes];

        let inputs: Vec<ChipID> = self.get_input_chip_ids();
        let mut queue: VecDeque<ChipID> = VecDeque::from(inputs);

        while let Some(index) = queue.pop_front() {
            if self.updated_this_tick[index] == true {
                continue;
            }

            self.update_node(&index);
            self.updated_this_tick[index] = true;

            if let Some(targets) = self.forward_links.get(&index) {
                queue.extend(targets.iter().copied());
            }
        }
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

    fn get_input_chip_ids(&self) -> Vec<ChipID> {
        self.description.chips.iter()
            .filter(|(_, chip_type)| chip_type == &&ChipType::Input)
            .map(|(&id, _)| id)
            .collect()
    }

    fn update_node(&mut self, index: &ChipID) {
        let chip_type: &ChipType = self.description.chips.get(index).unwrap();
        if *chip_type == ChipType::Nand {
            self.values[*index] = self.nand(&index);
        } else if *chip_type == ChipType::Output {
            let source = self.back_links[&index][0];
            self.values[*index] = self.values[source];
        }
    }

    fn nand(&self, index: &ChipID) -> u8 {
        let a_idx = self.back_links[index][0];
        let b_idx = self.back_links[index][1];
        let a = self.values[a_idx];
        let b = self.values[b_idx];

        if a == 1 && b == 1 { 0 } else { 1 }
    }
}
