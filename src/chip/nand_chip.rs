pub struct NANDChip {
    pub input_a: u8,
    pub input_b: u8,
    pub vcc: u8,
    pub gnd: u8,
}

impl NANDChip {
    pub fn new() -> Self {
        Self {
            input_a: 0,
            input_b: 0,
            vcc: 0,
            gnd: 0,
        }
    }

    pub fn set_vcc(&mut self, vcc: u8) {
        self.vcc = vcc;
    }

    pub fn set_gnd(&mut self, gnd: u8) {
        self.gnd = gnd;
    }

    pub fn set_inputs(&mut self, input_a: u8, input_b: u8) {
        self.input_a = input_a;
        self.input_b = input_b;
    }

    pub fn output(&self) -> u8 {
        if self.vcc == 0 || self.gnd == 1 {
            return 0;
        }

        if self.input_a == 1 && self.input_b == 1 {
            0
        } else {
            1
        }
    }
}
