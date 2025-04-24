mod chip_description;
pub use chip_description::ChipDescription;

mod chip;
pub use chip::{Chip, ChipType};

mod tickable;
pub use tickable::Tickable;

mod trace;
pub use trace::TraceMap;

mod circuit;
pub use circuit::{Circuit, CircuitDescription};

#[cfg(test)]
mod tests;
