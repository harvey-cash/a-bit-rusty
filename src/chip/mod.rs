mod chip_description;
pub use chip_description::ChipDescription;

mod chip;
pub use chip::{Chip, Tickable, CustomChip, ChipType, GroundChip, NAndChip, SupplyChip};

mod trace;
pub use trace::TraceMap;

mod circuit_description;
pub use circuit_description::CircuitDescription;

mod circuit;
pub use circuit::Circuit;

#[cfg(test)]
mod tests;
