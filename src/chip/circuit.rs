use std::collections::HashMap;

pub type ChipID = usize;

pub struct CircuitDescription {
    pub ground_chips: Vec<ChipID>,
    pub supply_chips: Vec<ChipID>,
    pub input_chips: Vec<ChipID>,
    pub output_chips: Vec<ChipID>,
    pub pins: HashMap<usize, Vec<usize>>,
}

impl CircuitDescription {
    pub fn new() -> Self {
        let mut pins: HashMap<usize, Vec<usize>> = HashMap::new();
        pins.insert(0, vec![0]);
        pins.insert(1, vec![1]);
        pins.insert(2, vec![0]);
        pins.insert(3, vec![0]);

        Self {
            ground_chips: vec![0],
            supply_chips: vec![1],
            input_chips: vec![2],
            output_chips: vec![3],
            pins,
        }
    }
}

pub struct Circuit {
    description: CircuitDescription,
    value: u8,
}

impl Circuit {
    pub fn new() -> Self {
        Self {
            description: CircuitDescription::new(),
            value: 0,
        }
    }

    pub fn get_description(&self) -> &CircuitDescription {
        &self.description
    }

    pub fn get_output(&self, _output_index: usize) -> u8 {
        self.value
    }

    pub fn set_supply(&mut self, value: u8) {
        self.value = value;
    }

    pub fn create_link(&mut self, source_pin_id: usize, target_pin_id: usize) {
        if source_pin_id == 0 {
            self.value = 0;
        } else {
            self.value = 1;
        }
    }
}
