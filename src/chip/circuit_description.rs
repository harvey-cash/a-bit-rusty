use std::collections::HashMap;

use super::{
    chip::ChipType, 
    chip_description::{ChipAndPin, ChipDescription, Link}
};

pub struct CircuitDescription {
    pub num_chips: usize,
    pub chip_types: HashMap<usize, ChipType>,
    pub forward_links: HashMap<usize, HashMap<ChipAndPin, Vec<ChipAndPin>>>,
}

impl CircuitDescription {
    pub fn new() -> Self {
        Self {
            num_chips: 0,
            chip_types: HashMap::new(),
            forward_links: HashMap::new(),
        }
    }

    pub fn add_chip(&mut self, chip_type: ChipType) -> usize {
        let id = self.num_chips;
        self.num_chips += 1;
        self.chip_types.entry(id).or_insert(chip_type);
        id
    }

    pub fn is_valid(&self) -> bool {
        let chip_description = self.compile_to_chip();
        chip_description.is_valid()
    }

    pub fn compile_to_chip(&self) -> ChipDescription {
        let num_inputs = self.chip_types.iter().filter(|(_, chip_type)| chip_type == &&ChipType::Input).count();
        let num_nands = self.chip_types.iter().filter(|(_, chip_type)| chip_type == &&ChipType::Custom).count();
        let num_outputs = self.chip_types.iter().filter(|(_, chip_type)| chip_type == &&ChipType::Output).count();
        
        let mut links = Vec::new();
        for (source_chip_id, sources) in &self.forward_links {
            for (_, target_pins) in sources {
                for target_pin in target_pins {
                    links.push(Link::new(source_chip_id.clone(), target_pin.chip_id));
                }
            }
        }
        
        ChipDescription::new(num_inputs, num_nands, num_outputs, links)
    }
    
    pub fn add_forward_link(&mut self, source: ChipAndPin, target: ChipAndPin) {
        self.forward_links
            .entry(source.chip_id)
            .or_insert_with(HashMap::new)
            .entry(source)
            .or_insert_with(Vec::new)
            .push(target);
    }
    
    pub fn delete_link(&mut self, source: ChipAndPin, target: ChipAndPin) {
        let forward_links = self.forward_links.get_mut(&source.chip_id);
        if forward_links.is_none() {
            panic!("No forward links found for chip ID {}.", source.chip_id);
        }
        let forward_links = forward_links.unwrap();
        let targets = forward_links.get_mut(&source);
        if targets.is_none() {
            panic!("No targets found for source chip and pin.");
        }
        let targets = targets.unwrap();
        targets.retain(|t| t != &target);
    }
}
