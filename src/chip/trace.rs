use nalgebra::{self, Matrix};

pub type Vector3 = nalgebra::Vector3<i32>;

pub struct Trace {}

impl Trace {
    pub fn new(a: Vector3, b: Vector3) -> Self {
        Self::panic_if_ends_too_far_apart(&a, &b);
        Self {}
    }

    fn panic_if_ends_too_far_apart(a: &Vector3, b: &Vector3) {
        let delta = a - b;
        let abs_delta = Matrix::abs(&delta);
        if abs_delta.x > 1 || abs_delta.y > 1 || abs_delta.z > 1 {
            panic!("Trace ends too far apart! {} - {}", a, b);
        }
    }
}

pub struct TraceMap {
    traces: Vec<Trace>,
}

impl TraceMap {
    pub fn new() -> Self {
        let traces = vec![];
        Self { traces }
    }

    pub fn add(&mut self, trace: Trace) {
        self.traces.push(trace);
    }

    pub fn get_graphs(&self) -> &Vec<Trace> {
        &self.traces
    }
}
