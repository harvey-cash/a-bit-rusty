use super::{
    chip::{ChipType, GroundChip, InputChip, NAndChip, OutputChip, SupplyChip},
    chip_description::ChipDescription,
    circuit::Circuit,
    circuit_description::CircuitDescription,
};

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum LoadableChip {
    Basic(ChipType),
    Custom(String),
}

pub struct CircuitBuilder {
    circuit: Circuit,
    chip_list: Vec<LoadableChip>,
}

impl CircuitBuilder {
    pub fn new() -> Self {
        Self {
            circuit: Circuit::new(),
            chip_list: vec![
                LoadableChip::Basic(ChipType::Ground),
                LoadableChip::Basic(ChipType::Supply),
                LoadableChip::Basic(ChipType::Input),
                LoadableChip::Basic(ChipType::Output),
                LoadableChip::Custom(String::from("NAnd")),
            ],
        }
    }

    pub fn get_chip_list(&self) -> Vec<LoadableChip> {
        self.chip_list.clone()
    }

    pub fn load_chip(&mut self, from: LoadableChip) -> usize {
        match from {
            LoadableChip::Basic(ChipType::Ground) => self.circuit.add_chip(GroundChip::new()),
            LoadableChip::Basic(ChipType::Supply) => self.circuit.add_chip(SupplyChip::new()),
            LoadableChip::Basic(ChipType::Input) => self.circuit.add_chip(InputChip::new()),
            LoadableChip::Basic(ChipType::Output) => self.circuit.add_chip(OutputChip::new()),
            LoadableChip::Custom(name) => self.circuit.add_custom_chip(NAndChip::new()),
            _ => {
                panic!("Unknown chip type!")
            }
        }
    }

    pub fn get_circuit(&mut self) -> &mut Circuit {
        &mut self.circuit
    }

    pub fn save_chip(&mut self, chip: ChipDescription, name: &str) {
        self.chip_list.push(LoadableChip::Custom(name.to_string()));
    }
}
