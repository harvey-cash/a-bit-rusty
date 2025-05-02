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
        chip::{
            ChipType, CustomChip, GroundChip, InputChip, NAndChip, OutputChip, SupplyChip, Tickable,
        },
        chip_description::ChipDescription,
        circuit::Circuit,
        circuit_builder::{CircuitBuilder, LoadableChip, LoadedChip},
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

fn build_not() -> ChipDescription {
    let mut circuit = Circuit::new();
    let ground = circuit.add_chip(GroundChip::new());
    let supply = circuit.add_chip(SupplyChip::new());
    let input = circuit.add_chip(InputChip::new());
    let output = circuit.add_chip(OutputChip::new());
    let nand = circuit.add_custom_chip(NAndChip::new());

    circuit.create_link(chip_pin!(ground, 0), chip_pin!(nand, 0));
    circuit.create_link(chip_pin!(supply, 0), chip_pin!(nand, 1));
    circuit.create_link(chip_pin!(input, 0), chip_pin!(nand, 2));
    circuit.create_link(chip_pin!(input, 0), chip_pin!(nand, 3));
    circuit.create_link(chip_pin!(nand, 4), chip_pin!(output, 0));

    ChipCompiler::compile(circuit.get_description())
}

#[test]
fn given_saved_not_then_chip_list_contains_not() {
    let chip_description: ChipDescription = build_not();
    let mut builder = CircuitBuilder::new();
    builder.save_chip(chip_description, "Not");

    let chips = builder.get_chip_list();
    assert!(chips.contains(&LoadableChip::Custom(String::from("Not"))));
}

#[test]
fn given_saved_not_when_loaded_then_behaves_as_not() {
    let chip_description: ChipDescription = build_not();
    let mut builder = CircuitBuilder::new();
    builder.save_chip(chip_description, "Not");

    let loaded: LoadedChip = builder.load_chip(LoadableChip::Custom(String::from("Not")));
    let chip_description = match loaded {
        LoadedChip::Basic(_) => panic!("Expected a custom chip"),
        LoadedChip::Custom(description) => description,
    };

    let mut circuit = Circuit::new();
    let ground = circuit.add_chip(GroundChip::new());
    let supply = circuit.add_chip(SupplyChip::new());
    let input = circuit.add_chip(InputChip::new());
    let output = circuit.add_chip(OutputChip::new());
    let not = circuit.add_custom_chip(CustomChip::new(chip_description));
    circuit.create_link(chip_pin!(ground, 0), chip_pin!(not, 0));
    circuit.create_link(chip_pin!(supply, 0), chip_pin!(not, 1));
    circuit.create_link(chip_pin!(input, 0), chip_pin!(not, 2));
    circuit.create_link(chip_pin!(not, 3), chip_pin!(output, 0));

    circuit.set_supply(supply, 1);

    circuit.set_input(input, 0);
    circuit.tick();
    assert_eq!(circuit.get_output(output), 1);

    circuit.set_input(input, 1);
    circuit.tick();
    assert_eq!(circuit.get_output(output), 0);
}
