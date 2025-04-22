pub type ChipID = usize;

pub struct CircuitDescription {
    pub ground_chips: Vec<ChipID>,
    pub supply_chips: Vec<ChipID>,
    pub input_chips: Vec<ChipID>,
    pub output_chips: Vec<ChipID>,
}

impl CircuitDescription {
    pub fn new() -> Self {
        Self {
            ground_chips: vec![1],
            supply_chips: vec![1],
            input_chips: vec![1],
            output_chips: vec![1],
        }
    }
}

pub struct Circuit {
    value: u8,
}

impl Circuit {
    pub fn new() -> Self {
        Self { value: 0 }
    }

    pub fn get_description(&self) -> CircuitDescription {
        CircuitDescription::new()
    }

    pub fn get_output(&self, _output_index: usize) -> u8 {
        self.value
    }

    pub fn set_supply(&mut self, value: u8) {
        self.value = value;
    }
}
