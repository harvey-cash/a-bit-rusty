use std::collections::HashMap;

use serde::Serialize;

use super::{chip::{ChipType, CustomChip, GroundChip, InputChip, OutputChip, SupplyChip, Tickable}, chip_database::{ChipDatabase, ChipKey, ChipValue}, circuit::{self, Circuit}, types::ChipAndPin};

#[derive(Serialize)]
pub struct CircuitState {
    pub tick_counter: u64,
    pub outputs: HashMap<usize, u8>
}

impl CircuitState {
    pub fn new() -> Self { Self { tick_counter: 0, outputs: HashMap::new() } }
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
            database: ChipDatabase::new() 
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

    pub fn add_link(&mut self, source: ChipAndPin, target: ChipAndPin) -> Result<usize, String> {
        if source == target {
            return Err("Can not link a pin to itself!".to_string());
        }
        self.circuit.create_link(source, target);
        Ok(0)
    }

    pub fn delete_link(&mut self, id: usize) -> Result<(), String> {
        Ok(())
    }

    pub fn set_input_chip_value(&mut self, id: usize, value: u8) -> Result<(), String> {
        self.circuit.set_input(id, value);
        Ok(())
    }

    pub fn tick(&mut self) -> Result<(), String> {
        self.tick_counter += 1;
        self.circuit.tick();
        Ok(())
    }

    pub fn get_state(&self, ) -> CircuitState {
        let mut outputs: HashMap<usize, u8> = HashMap::new();
        let value = self.circuit.get_output(1);
        outputs.insert(1, value);
        CircuitState { tick_counter: self.tick_counter, outputs: outputs }
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
}