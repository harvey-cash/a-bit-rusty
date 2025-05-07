use serde::Serialize;

use super::{chip_database::{ChipDatabase, ChipKey}, circuit::Circuit, types::ChipAndPin};

#[derive(Serialize)]
pub struct CircuitState {
    tick_counter: u64,
}

impl CircuitState {
    pub fn new() -> Self { Self { tick_counter: 0 } }
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
        Ok(0)
    }

    pub fn remove_chip(&mut self, id: usize) -> Result<(), String> {
        Ok(())
    }

    pub fn add_link(&mut self, source: ChipAndPin, target: ChipAndPin) -> Result<usize, String> {
        Ok(0)
    }

    pub fn delete_link(&mut self, id: usize) -> Result<(), String> {
        Ok(())
    }

    pub fn set_input_chip_value(&mut self, id: usize, value: u8) -> Result<(), String> {
        Ok(())
    }

    pub fn tick(&mut self) -> Result<(), String> {
        self.tick_counter += 1;
        Ok(())
    }

    pub fn get_state(&self, ) -> CircuitState {
        CircuitState { tick_counter: self.tick_counter }
    }

    pub fn get_chips_from_db(&self) -> Result<Vec<ChipKey>, String> {
        Ok(vec![])
    }

    pub fn save_to_chip_to_db(&mut self, name: String) -> Result<ChipKey, String> {
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