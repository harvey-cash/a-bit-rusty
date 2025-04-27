use std::collections::HashMap;


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PinLayout {
    pub ground_pins: Vec<usize>,
    pub supply_pins: Vec<usize>,
    pub input_pins: Vec<usize>,
    pub output_pins: Vec<usize>,
    pub num_pins: usize,
}
impl PinLayout {
    pub fn new(num_ground: usize, num_supply: usize, num_inputs: usize, num_outputs: usize) -> Self
    {
        let ground_pins = Vec::from_iter(0.. num_ground);
        let mut max = num_ground;
        let supply_pins = Vec::from_iter(max..max+num_supply);
        max += num_supply;
        let input_pins = Vec::from_iter(max..max+num_inputs);
        max += num_inputs;
        let output_pins = Vec::from_iter(max..max+num_outputs);
        
        let num_pins = max+num_outputs;

        Self { ground_pins, supply_pins, input_pins, output_pins, num_pins }
    }
    
    pub fn get_num_inputs(&self) -> usize {
        self.ground_pins.len() + self.supply_pins.len() + self.input_pins.len()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChipAndPin {
    pub chip_id: usize,
    pub pin_index: usize,
}
impl ChipAndPin {
    pub fn new(chip_id: usize, pin_index: usize) -> Self {
        Self { chip_id, pin_index }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NodeType {
    Input,
    Output,
    NAnd,
}

pub type NodeId = usize;
pub type LinkMap = HashMap<NodeId, Vec<NodeId>>;
pub type NodeTypeMap = HashMap<NodeId, NodeType>;

pub struct Link {
    pub source: NodeId,
    pub target: NodeId,
}


impl Link {
    pub fn new(source: NodeId, target: NodeId) -> Self {
        Link { source, target }
    }
}