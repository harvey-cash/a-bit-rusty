use std::collections::HashMap;

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

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum LoadedChip {
    Basic(ChipType),
    Custom(ChipDescription),
}

pub struct CircuitBuilder {
    circuit: Circuit,
    chip_list: Vec<LoadableChip>,
    saved_chips: HashMap<String, ChipDescription>,
}

impl CircuitBuilder {
    pub fn new() -> Self {
        let mut saved_chips = HashMap::new();
        saved_chips.insert(String::from("NAnd"), NAndChip::new().get_description());

        Self {
            circuit: Circuit::new(),
            chip_list: vec![
                LoadableChip::Basic(ChipType::Ground),
                LoadableChip::Basic(ChipType::Supply),
                LoadableChip::Basic(ChipType::Input),
                LoadableChip::Basic(ChipType::Output),
                LoadableChip::Custom(String::from("NAnd")),
            ],
            saved_chips,
        }
    }

    pub fn get_chip_list(&self) -> Vec<LoadableChip> {
        self.chip_list.clone()
    }

    pub fn load_chip(&mut self, from: LoadableChip) -> LoadedChip {
        match from {
            LoadableChip::Basic(ChipType::Ground) => LoadedChip::Basic(ChipType::Ground),
            LoadableChip::Basic(ChipType::Supply) => LoadedChip::Basic(ChipType::Supply),
            LoadableChip::Basic(ChipType::Input) => LoadedChip::Basic(ChipType::Input),
            LoadableChip::Basic(ChipType::Output) => LoadedChip::Basic(ChipType::Output),
            LoadableChip::Custom(name) => LoadedChip::Custom(
                self.saved_chips
                    .get(&name)
                    .unwrap_or_else(|| panic!("Chip {} not found!", name))
                    .clone(),
            ),
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
        self.saved_chips.insert(name.to_string(), chip);
    }
}
