use std::collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};
use super::{
    chip::{ChipType, NAndChip},
    chip_description::ChipDescription, circuit_description::CircuitDescription,
};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug, Hash)]
pub enum ChipKey {
    Basic(ChipType),
    Custom(String),
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ChipValue {
    Basic(ChipType),
    Custom(ChipDescription),
}

#[derive(Debug)]
pub struct ChipDatabase {
    fundamental_chips: HashSet<ChipKey>,
    saved_chips: HashMap<ChipKey, ChipValue>,
    saved_circuits: HashMap<String, CircuitDescription>
}

impl ChipDatabase {
    pub fn new() -> Self {
        let mut saved_chips = HashMap::new();

        let basic_types = [ChipType::Ground, ChipType::Supply, ChipType::Input, ChipType::Output];
        basic_types.iter().for_each(|chip_type| { 
            saved_chips.insert(ChipKey::Basic(*chip_type), ChipValue::Basic(*chip_type)); 
        });

        let nand_key = ChipKey::Custom(String::from("NAnd"));
        saved_chips.insert(nand_key.clone(), ChipValue::Custom(NAndChip::new().get_description()));

        let mut fundamental_chips: HashSet<ChipKey> = basic_types.iter().map(|chip_type| ChipKey::Basic(*chip_type)).collect();
        fundamental_chips.insert(nand_key);

        Self {
            fundamental_chips,
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

    pub fn save_chip(&mut self, name: &str, chip: ChipDescription, circuit: CircuitDescription) -> bool {
        self.saved_chips.insert(ChipKey::Custom(name.to_string()), ChipValue::Custom(chip));
        self.saved_circuits.insert(name.to_string(), circuit);
        return true;
    }

    pub fn save_circuit(&mut self, description: CircuitDescription, name: &str) -> bool {
        let key: String = name.to_string();

        if self.saved_circuits.contains_key(&key) {
            return false;
        }

        self.saved_circuits.insert(key, description);
        return true;
    }

    pub fn load_chip(&self, key: ChipKey) -> Option<&ChipValue> {
        self.saved_chips.get(&key)
    }
    
    pub fn load_circuit(&self, name: &str) -> Option<&CircuitDescription> {
        self.saved_circuits.get(name)
    }
    
    pub fn delete_chip(&mut self, key: &ChipKey) -> bool {
        if self.fundamental_chips.contains(key) {
            return false;
        }
        let value = self.saved_chips.remove(key);
        return if value == None { false } else { true };
    }

    pub fn delete_circuit(&mut self, key: &str) -> bool {
        let value = self.saved_circuits.remove(key);
        return if value == None { false } else { true };
    }
}
