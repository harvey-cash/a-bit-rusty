mod chip_description;
pub use chip_description::ChipDescription;

mod chip;
pub use chip::Chip;

mod trace;
pub use trace::TraceMap;

#[cfg(test)]
mod tests;
