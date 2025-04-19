pub struct Input {}

impl Input {
    pub fn new() -> Self {
        Input {}
    }
}

pub struct Output {}

impl Output {
    pub fn new() -> Self {
        Output {}
    }
}

pub struct Nand {}

impl Nand {
    pub fn new() -> Self {
        Nand {}
    }
}

pub struct Link {}

impl Link {
    pub fn new(_input_idx: usize, _output_idx: usize) -> Self {
        Link {}
    }
}

pub struct Chip {
    last_input_value: i32,
}

impl Chip {
    pub fn new<const I: usize, const O: usize, const N: usize, const L: usize>(
        _inputs: [Input; I],
        _outputs: [Output; O],
        _nands: [Nand; N],
        _links: [Link; L],
    ) -> Self {
        Chip {
            last_input_value: 0,
        } // Default value, will be overwritten by update_input
    }

    pub fn update_input(&mut self, _index: usize, value: i32) {
        self.last_input_value = value;
    }

    pub fn get_output(&self, _index: usize, _unused_param: i32) -> i32 {
        self.last_input_value
    }
}
