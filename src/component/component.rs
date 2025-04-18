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

pub struct Link {}

impl Link {
    pub fn new(_input_idx: usize, _output_idx: usize) -> Self {
        Link {}
    }
}

pub struct Component {
    last_input_value: i32,
}

impl Component {
    pub fn new<const N: usize, const M: usize, const L: usize>(
        _inputs: [Input; N],
        _outputs: [Output; M],
        _links: [Link; L],
    ) -> Self {
        Component { last_input_value: 0 } // Default value, will be overwritten by update_input
    }

    pub fn update_input(&mut self, _index: usize, value: i32) {
        self.last_input_value = value;
    }

    pub fn get_output(&self, _index: usize, _unused_param: i32) -> i32 {
        self.last_input_value
    }
}