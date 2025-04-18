mod nand_gate;
pub use nand_gate::NANDGate;

mod component;
pub use component::{Component, Input, Output, Link };

#[cfg(test)]
mod tests;