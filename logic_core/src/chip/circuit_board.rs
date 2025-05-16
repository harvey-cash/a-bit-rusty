use std::collections::HashMap;

use super::trace::Vector3;

pub struct CircuitBoard {
    chip_positions: HashMap<usize, Vector3>,
}

impl CircuitBoard {
    pub fn new() -> Self { Self { 
        chip_positions: HashMap::new(), 
    } }

    pub fn get_chip_positions(&self) -> &HashMap<usize, Vector3> {
        &self.chip_positions
    }
    
    pub(crate) fn add_chip_at_position(&mut self, id: usize, pos: Vector3) -> Result<(), String> {
        self.chip_positions.insert(id, pos);
        Ok(())
    }
}