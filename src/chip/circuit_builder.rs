use std::collections::HashMap;

use super::{
    chip::{ChipType, NAndChip},
    chip_description::ChipDescription, circuit::Circuit, circuit_description::CircuitDescription,
};

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum ChipKey {
    Basic(ChipType),
    Custom(String),
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ChipValue {
    Basic(ChipType),
    Custom(ChipDescription),
}

pub struct CircuitBuilder {
    saved_chips: HashMap<ChipKey, ChipValue>,
}

impl CircuitBuilder {
    pub fn new() -> Self {
        let mut saved_chips = HashMap::new();

        let basic_types = [ChipType::Ground, ChipType::Supply, ChipType::Input, ChipType::Output];
        basic_types.iter().for_each(|chip_type| { 
            saved_chips.insert(ChipKey::Basic(*chip_type), ChipValue::Basic(*chip_type)); 
        });
        saved_chips.insert(ChipKey::Custom(String::from("NAnd")), ChipValue::Custom(NAndChip::new().get_description()));

        Self {
            saved_chips,
        }
    }

    pub fn get_chip_list(&self) -> Vec<ChipKey> {
        self.saved_chips.keys().map(|k| k.clone()).collect()
    }

    pub fn load_chip(&self, key: ChipKey) -> Option<&ChipValue> {
        self.saved_chips.get(&key)
    }

    pub fn save_chip(&mut self, description: ChipDescription, name: &str) {
        let key: ChipKey = ChipKey::Custom(name.to_string());
        let value: ChipValue = ChipValue::Custom(description);
        self.saved_chips.insert(key, value);
    }
    
    pub fn load_circuit(&self, chip_name: &str) -> Option<CircuitDescription> {
        let key = &ChipKey::Custom(chip_name.to_string());
        match self.saved_chips.get(key) {
            None => None,
            Some(_) => Some(CircuitDescription::new())
        }
    }
}
