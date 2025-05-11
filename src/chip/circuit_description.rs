use std::collections::HashMap;

use super::{chip::ChipType, chip_description::ChipDescription, types::*};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CircuitDescription {
    pub name: String,
    pub num_chips: usize,
    pub chip_types: HashMap<usize, ChipType>,
    pub chip_descriptions: HashMap<usize, ChipDescription>,
    pub forward_links: HashMap<usize, HashMap<usize, Vec<ChipAndPin>>>,

    back_links: HashMap<ChipAndPin, ChipAndPin>,
}

impl CircuitDescription {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            num_chips: 0,
            chip_types: HashMap::new(),
            chip_descriptions: HashMap::new(),
            forward_links: HashMap::new(),
            back_links: HashMap::new(),
        }
    }

    pub fn add_chip(&mut self, chip_type: ChipType) -> usize {
        let id = self.num_chips;
        self.num_chips += 1;
        self.chip_types.entry(id).or_insert(chip_type);
        id
    }

    pub fn add_custom_chip(&mut self, description: ChipDescription) -> usize {
        let id = self.num_chips;
        self.num_chips += 1;
        self.chip_types.entry(id).or_insert(ChipType::Custom);
        self.chip_descriptions.entry(id).or_insert(description);
        id
    }

    pub fn is_valid(&self) -> bool {
        if self.has_custom_chips_but_no_supply() {
            return false;
        }
        return true;
    }

    pub fn add_forward_link(&mut self, source: ChipAndPin, target: ChipAndPin) {
        assert!(source != target, "Can not link a pin to itself!");

        self.forward_links
            .entry(source.chip_id)
            .or_insert_with(HashMap::new)
            .entry(source.pin_index)
            .or_insert_with(Vec::new)
            .push(target);

        self.back_links.insert(target, source);
    }

    pub fn get_back_links(&self) -> HashMap<ChipAndPin, ChipAndPin> {
        self.back_links.clone()
    }

    pub fn delete_link(&mut self, source: ChipAndPin, target: ChipAndPin) {
        let forward_links = self.forward_links.get_mut(&source.chip_id);
        if forward_links.is_none() {
            panic!("No forward links found for chip ID {}.", source.chip_id);
        }
        let forward_links = forward_links.unwrap();
        let targets = forward_links.get_mut(&source.pin_index);
        if targets.is_none() {
            panic!("No targets found for source chip and pin.");
        }
        let targets = targets.unwrap();
        targets.retain(|t| t != &target);
    }

    pub fn get_description_of_chip(&self, id: usize) -> ChipDescription {
        if let Some(description) = self.chip_descriptions.get(&id) {
            return description.clone();
        } else {
            panic!("No chip description found for ID {}.", id);
        }
    }

    fn has_custom_chips_but_no_supply(&self) -> bool {
        let num_custom = self
            .chip_types
            .iter()
            .filter(|(_, chip_type)| chip_type == &&ChipType::Custom)
            .count();
        let num_supply = self
            .chip_types
            .iter()
            .filter(|(_, chip_type)| chip_type == &&ChipType::Supply)
            .count();
        return num_custom > 0 && num_supply == 0;
    }
}
