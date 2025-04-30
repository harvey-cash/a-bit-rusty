use std::collections::HashMap;

pub fn vec_contents_eq_ignore_order(vec1: &Vec<usize>, vec2: &Vec<usize>) -> bool {
    if vec1.len() != vec2.len() {
        return false;
    }

    let mut sorted1 = vec1.clone();
    sorted1.sort_unstable();
    let mut sorted2 = vec2.clone();
    sorted2.sort_unstable();    
    sorted1 == sorted2
}

pub fn map_contents_eq_ignore_order(map1: &LinkMap, map2: &LinkMap) -> bool {
    if map1.len() != map2.len() {
        return false;
    }
    for (key, vec1) in map1 {
        match map2.get(key) {
            None => return false,
            Some(vec2) => {
                if !vec_contents_eq_ignore_order(vec1, vec2) {
                    return false;
                }
            }
        }
    }
    true
}

#[derive(Debug, Clone)]
pub struct PinLayout {
    pub ground_pins: Vec<usize>,
    pub supply_pins: Vec<usize>,
    pub input_pins: Vec<usize>,
    pub output_pins: Vec<usize>,
    pub num_pins: usize,

    id_pin_map: HashMap<usize, usize>,
    pin_id_map: HashMap<usize, usize>,
}

impl PartialEq for PinLayout {
    fn eq(&self, other: &Self) -> bool {
        vec_contents_eq_ignore_order(&self.ground_pins, &other.ground_pins) &&
        vec_contents_eq_ignore_order(&self.supply_pins, &other.supply_pins) &&
        vec_contents_eq_ignore_order(&self.input_pins, &other.input_pins) &&
        vec_contents_eq_ignore_order(&self.output_pins, &other.output_pins) &&
        self.num_pins == other.num_pins
    }
}
impl Eq for PinLayout {}

impl PinLayout {
    pub fn new(id_type_map: NodeTypeMap) -> Self
    {        
        let mut num_pins = 0;
        let mut ground_pins = vec![];
        let mut supply_pins = vec![];
        let mut input_pins = vec![];
        let mut output_pins = vec![];

        let mut id_pin_map: HashMap<usize, usize> = HashMap::new();
        let mut pin_id_map: HashMap<usize, usize> = HashMap::new();

        for (id, _) in id_type_map.iter().filter(|(_, node_type)| node_type == &&NodeType::Ground) {
            id_pin_map.insert(*id, num_pins);
            pin_id_map.insert(num_pins, *id);
            ground_pins.push(*id);
            num_pins += 1;
        }
        for (id, _) in id_type_map.iter().filter(|(_, node_type)| node_type == &&NodeType::Supply) {
            id_pin_map.insert(*id, num_pins);
            pin_id_map.insert(num_pins, *id);
            supply_pins.push(*id);
            num_pins += 1;
        }
        for (id, _) in id_type_map.iter().filter(|(_, node_type)| node_type == &&NodeType::Input) {
            id_pin_map.insert(*id, num_pins);
            pin_id_map.insert(num_pins, *id);
            input_pins.push(*id);
            num_pins += 1;
        }
        for (id, _) in id_type_map.iter().filter(|(_, node_type)| node_type == &&NodeType::Output) {
            id_pin_map.insert(*id, num_pins);
            pin_id_map.insert(num_pins, *id);
            output_pins.push(*id);
            num_pins += 1;
        }

        Self { ground_pins, supply_pins, input_pins, output_pins, num_pins, id_pin_map, pin_id_map }
    }
    
    pub fn get_num_inputs(&self) -> usize {
        self.ground_pins.len() + self.supply_pins.len() + self.input_pins.len()
    }

    pub fn get_pin_for(&self, id: usize) -> usize {
        let extern_pin = self.id_pin_map.get(&id).expect("ID not found in pin map!");
        *extern_pin
    }

    pub fn get_id_for(&self, pin: usize) -> usize {
        let id = self.pin_id_map.get(&pin).expect("Pin not found!");
        *id
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

#[macro_export]
macro_rules! cap {
    ( $id:expr => $pin:expr ) => {
        ChipAndPin::new($id, $pin)
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NodeType {
    Ground,
    Supply,
    Input,
    Output,
    NAnd,
    Buffer,
}

pub type NodeTypeMap = HashMap<usize, NodeType>;

#[macro_export]
macro_rules! node_type_map {
    ( $( $key:expr => $value:expr ),* $(,)? ) => {
        {            
            let mut map = std::collections::HashMap::new();
            $(
                map.insert($key, $value);
            )*
            map
        }
    };
}

pub type LinkMap = HashMap<usize, Vec<usize>>;

#[derive(Debug)]
pub struct Link {
    pub source: usize,
    pub target: usize,
}
impl Link {
    pub fn new(source: usize, target: usize) -> Self {
        Link { source, target }
    }
}

#[macro_export]
macro_rules! link {
    ( $source:expr => $target:expr ) => {
        Link::new($source, $target)
    };
}
