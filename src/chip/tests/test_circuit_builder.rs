// ToDo:
// [ ] Chip list begins with the fundamental chips: Ground, Supply, Input, Output, and NAnd
// [ ] Compiled chips get added to the chip list
// [ ] Can delete chips from the chip list
// [ ] Can not delete the fundamental chips from the chip list
// [ ] Compiled chips have their circuit saved to a circuit list
// [ ] Can load circuits from the circuit list for editing
// [ ] Given a circuit already compiled a chip, can choose to overwrite on save
// [ ] Given a circuit already compiled a chip, can choose to save-as new
// [ ] Can save circuits to the circuit list
// [ ] Can add chips from the chip list to a circuit
// [ ] Turning the circuit on or off sets the value of all Supply chips
// [ ] Can add links between chip pins on a circuit
// [ ] Links added between two source pins are not saved
// [ ] Link state is set by the source pin state
// [ ] Can delete links
// [ ] Can manually tick a circuit
// [ ] Can set an automatic tick rate for the circuit

// [ ] Can specify a truth table of Input and Output states over a number of ticks
// [ ] Truth table gets saved along with the circuit
// [ ] Can run a circuit against a truth table and compare the actual output against the expected

use crate::chip::{chip::ChipType, circuit_builder::{CircuitBuilder, LoadableChip}};

#[test]
fn given_new_then_chip_list_contains_fundamentals() {
    let builder = CircuitBuilder::new();
    let chips = builder.get_chip_list();
    assert!(chips.contains(&LoadableChip::Basic(ChipType::Ground)));
    assert!(chips.contains(&LoadableChip::Basic(ChipType::Supply)));
    assert!(chips.contains(&LoadableChip::Basic(ChipType::Input)));
    assert!(chips.contains(&LoadableChip::Basic(ChipType::Output)));
    assert!(chips.contains(&LoadableChip::Custom(String::from("NAnd"))));
}

#[test]
fn given_load_nand_then_added_to_circuit() {
    let mut builder = CircuitBuilder::new();
    let id = builder.load_chip(LoadableChip::Custom(String::from("NAnd")));
    let circuit = builder.get_circuit_description();
    assert_eq!(circuit.chip_types.get(&id).unwrap(), &ChipType::Custom);
}