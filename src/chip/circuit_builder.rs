use super::{chip::{ChipType, NAndChip}, circuit_description::CircuitDescription};

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum LoadableChip {
    Basic(ChipType),
    Custom(String),
}

pub struct CircuitBuilder {}

impl CircuitBuilder {
    pub fn new() -> Self { Self {} }

    pub fn get_chip_list(&self) -> Vec<LoadableChip> {
        vec![ 
            LoadableChip::Basic(ChipType::Ground), 
            LoadableChip::Basic(ChipType::Supply), 
            LoadableChip::Basic(ChipType::Input), 
            LoadableChip::Basic(ChipType::Output),
            LoadableChip::Custom(String::from("NAnd")),
        ]
    }
    
    pub fn load_chip(&mut self, from: LoadableChip) -> usize {
        0
    }

    pub fn get_circuit_description(&self) -> CircuitDescription {
        let mut circuit = CircuitDescription::new();
        circuit.add_custom_chip(NAndChip::new().get_description());
        circuit
    }
}
