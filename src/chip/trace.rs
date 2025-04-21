use nalgebra::{self, Matrix};

pub type Vector3 = nalgebra::Vector3<i32>;

struct TraceSegment {}

impl TraceSegment {
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
    traces: Vec<TraceSegment>,
}

impl TraceMap {
    pub fn new() -> Self {
        let traces = vec![];
        Self { traces }
    }

    pub fn add(&mut self, a: Vector3, b: Vector3) {
        self.traces.push(TraceSegment::new(a, b));
    }

    pub fn get_graphs(&self) -> Vec<i32> {
        let list: Vec<i32> = self.traces.iter().map(|_f| 1).collect();
        list
    }
}
