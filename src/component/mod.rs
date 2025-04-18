mod nand_gate;

pub use nand_gate::NANDGate;

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod test_nand_gate;