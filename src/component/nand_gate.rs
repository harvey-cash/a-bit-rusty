
pub struct NANDGate {
    pub input_a: u8,
    pub input_b: u8,
    pub vcc: u8,
}

impl NANDGate {
    pub fn new() -> Self {
        Self {
            input_a: 0,
            input_b: 0,
            vcc: 0,
        }
    }

    pub fn set_vcc(&mut self, vcc: u8) {
        self.vcc = vcc;
    }

    pub fn set_inputs(&mut self, input_a: u8, input_b: u8) {
        self.input_a = input_a;
        self.input_b = input_b;
    }

    pub fn output(&self) -> u8 {
        if self.vcc == 0 {
            return 0;
        }

        if self.input_a == 1 && self.input_b == 1 {
            0
        } else {
            1
        }
    }
}
