use std::collections::HashMap;

use serde::Serialize;

use super::{chip::{ChipType, CustomChip, GroundChip, InputChip, OutputChip, SupplyChip, Tickable}, chip_database::{ChipDatabase, ChipKey, ChipValue}, circuit::{self, Circuit}, types::{ChipAndPin, PinLayout}};

#[derive(Serialize)]
pub struct DesignerState {
    pub tick_counter: u64,
    pub chip_pin_states: HashMap<usize, HashMap<usize, u8>>,
    pub links: HashMap<usize, HashMap<usize, Vec<ChipAndPin>>>,
}

pub struct Designer {    
    tick_counter: u64,
    circuit: Circuit,
    database: ChipDatabase,
}

impl Designer {
    pub fn new() -> Self { 
        Self { 
            tick_counter: 0,
            circuit: Circuit::new(), 
            database: ChipDatabase::new(),
        }
    }

    pub fn add_chip(&mut self, key: ChipKey) -> Result<usize, String> {        
        let chip = self.database.load_chip(key.clone());
        
        match chip {
            None => Err(format!("No chip with key {:?} in database!", key)),
            Some(ChipValue::Custom(description)) => Ok(self.circuit.add_custom_chip(CustomChip::new(description.clone()))),
            Some(ChipValue::Basic(ChipType::Ground)) => Ok(self.circuit.add_chip(GroundChip::new())),
            Some(ChipValue::Basic(ChipType::Supply)) => Ok(self.circuit.add_chip(SupplyChip::new())),
            Some(ChipValue::Basic(ChipType::Input)) => Ok(self.circuit.add_chip(InputChip::new())),
            Some(ChipValue::Basic(ChipType::Output)) => Ok(self.circuit.add_chip(OutputChip::new())),
            Some(ChipValue::Basic(ChipType::Custom)) => Err(format!("database should not contain a basic chip value of Custom type!")),
        }
    }

    pub fn remove_chip(&mut self, id: usize) -> Result<(), String> {
        Err("Not implemented".to_string())
    }

    pub fn add_link(&mut self, source: ChipAndPin, target: ChipAndPin) -> Result<(), String> {
        if source == target {
            return Err("Can not link a pin to itself!".to_string());
        }
        self.circuit.create_link(source, target);
        Ok(())
    }

    pub fn delete_link(&mut self, source: ChipAndPin, target: ChipAndPin) -> Result<(), String> {
        self.circuit.delete_link(source, target);
        Ok(())
    }

    pub fn set_input_chip_value(&mut self, id: usize, value: u8) -> Result<(), String> {
        self.circuit.set_input(id, value);
        Ok(())
    }

    pub fn tick(&mut self) {
        self.tick_counter += 1;
        self.circuit.tick();
    }

    pub fn get_state(&self) -> DesignerState {
        DesignerState { 
            tick_counter: self.tick_counter,
            chip_pin_states: self.get_pin_states_map(),
            links: self.circuit.get_description().forward_links,
        }
    }

    pub fn get_chips_from_db(&self) -> Result<Vec<ChipKey>, String> {
        Ok(vec![])
    }

    pub fn save_chip_to_db(&mut self, name: String) -> Result<ChipKey, String> {
        Ok(ChipKey::Custom(name))
    }

    pub fn delete_chip_from_db(&mut self, key: ChipKey) -> Result<(), String> {
        Ok(())
    }

    pub fn get_circuits_from_db(&self) -> Result<Vec<String>, String> {
        Ok(vec![])
    }

    pub fn save_circuit_to_db(&mut self, name: String) -> Result<(), String> {
        Ok(())
    }

    pub fn delete_circuit_from_db(&mut self, name: String) -> Result<(), String> {
        Ok(())
    }

    pub fn load_circuit_from_db(&mut self, name: String) -> Result<(), String> {
        Ok(())
    }

    pub fn new_circuit(&mut self) -> Result<(), String> {
        Ok(())
    }
    
    fn get_pin_states_map(&self) -> HashMap<usize, HashMap<usize, u8>> {
        let mut pin_states: HashMap<usize, HashMap<usize, u8>> = HashMap::new();
        for (chip_pin, value) in self.circuit.get_chip_pin_states() {
            pin_states.entry(chip_pin.chip_id).or_default().insert(chip_pin.pin_index, value);
        }
        return pin_states;
    }
}