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

use crate::{
    chip::{
        chip::ChipType,
        circuit,
        circuit_builder::{CircuitBuilder, LoadableChip},
        compiler::ChipCompiler,
        types::*,
    },
    chip_pin,
};

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
    let circuit = builder.get_circuit();
    assert_eq!(circuit.get_description().chip_types.contains_key(&id), true);
}

fn build_not(builder: &mut CircuitBuilder) {
    let ground = builder.load_chip(LoadableChip::Basic(ChipType::Ground));
    let supply = builder.load_chip(LoadableChip::Basic(ChipType::Supply));
    let input = builder.load_chip(LoadableChip::Basic(ChipType::Input));
    let output = builder.load_chip(LoadableChip::Basic(ChipType::Output));
    let nand = builder.load_chip(LoadableChip::Custom(String::from("NAnd")));

    let circuit = builder.get_circuit();
    circuit.create_link(chip_pin!(ground, 0), chip_pin!(nand, 0));
    circuit.create_link(chip_pin!(supply, 0), chip_pin!(nand, 1));
    circuit.create_link(chip_pin!(input, 0), chip_pin!(nand, 2));
    circuit.create_link(chip_pin!(input, 0), chip_pin!(nand, 3));
    circuit.create_link(chip_pin!(nand, 4), chip_pin!(output, 0));
}

#[test]
fn given_saved_not_then_chip_list_contains_not() {
    let mut builder = CircuitBuilder::new();
    build_not(&mut builder);
    let circuit = builder.get_circuit().get_description();
    let chip = ChipCompiler::compile(circuit);
    builder.save_chip(chip, "Not");

    let chips = builder.get_chip_list();
    assert!(chips.contains(&LoadableChip::Custom(String::from("Not"))));
}

// #[test]
// fn given_saved_not_then_can_load() {
//     let mut builder = CircuitBuilder::new();
//     build_not(&mut builder);
//     let circuit = builder.get_circuit().get_description();
//     let chip = ChipCompiler::compile(circuit);
//     builder.save_chip(chip, "Not");

//     let id = builder.load_chip(LoadableChip::Custom(String::from("Not")));
//     let circuit = builder.get_circuit();
//     assert_eq!(circuit.get_description().chip_types.contains_key(&id), true);
// }
