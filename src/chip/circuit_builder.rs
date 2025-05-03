use std::collections::HashMap;

use super::{
    chip::{ChipType, NAndChip},
    chip_description::ChipDescription, circuit_description::CircuitDescription,
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
    saved_circuits: HashMap<String, CircuitDescription>
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
            saved_circuits: HashMap::new()
        }
    }

    pub fn get_chip_list(&self) -> Vec<ChipKey> {
        self.saved_chips.keys().map(|k| k.clone()).collect()
    }
    
    pub fn get_circuit_list(&self) -> Vec<String> {
        self.saved_circuits.keys().map(|k| k.clone()).collect()
    }

    pub fn save_chip(&mut self, name: &str, chip: ChipDescription, circuit: CircuitDescription) {
        self.saved_chips.insert(ChipKey::Custom(name.to_string()), ChipValue::Custom(chip));
        self.saved_circuits.insert(name.to_string(), circuit);
    }

    pub fn load_chip(&self, key: ChipKey) -> Option<&ChipValue> {
        self.saved_chips.get(&key)
    }

    pub fn save_circuit(&mut self, description: CircuitDescription, name: &str) -> bool {
        let key: String = name.to_string();

        if self.saved_circuits.contains_key(&key) {
            return false;
        }

        self.saved_circuits.insert(key, description);
        return true;
    }
    
    pub fn load_circuit(&self, name: &str) -> Option<&CircuitDescription> {
        self.saved_circuits.get(name)
    }
}
