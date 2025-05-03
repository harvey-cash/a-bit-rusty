// ToDo:
// [ ] Can delete chips from the chip list
// [ ] Can not delete the fundamental chips from the chip list
// [ ] Compiled chips have their circuit saved to a circuit list
// [ ] Can load circuits from the circuit list for editing
// [ ] Given a circuit already compiled a chip, can choose to overwrite on save
// [ ] Given a circuit already compiled a chip, can choose to save-as new
// [ ] Can save circuits to the circuit list

use crate::{
    chip::{
        chip::{
            ChipType, CustomChip, GroundChip, InputChip, NAndChip, OutputChip, SupplyChip, Tickable,
        }, chip_description::ChipDescription, circuit::Circuit, circuit_builder::{ChipKey, ChipValue, CircuitBuilder}, circuit_description::CircuitDescription, compiler::ChipCompiler, types::*
    },
    chip_pin,
};

#[test]
fn given_new_then_chip_list_contains_fundamentals() {
    let builder = CircuitBuilder::new();
    let chips = builder.get_chip_list();
    assert!(chips.contains(&ChipKey::Basic(ChipType::Ground)));
    assert!(chips.contains(&ChipKey::Basic(ChipType::Supply)));
    assert!(chips.contains(&ChipKey::Basic(ChipType::Input)));
    assert!(chips.contains(&ChipKey::Basic(ChipType::Output)));
    assert!(chips.contains(&ChipKey::Custom(String::from("NAnd"))));
}

#[test]
fn given_nothing_saved_when_load_chip_then_is_none() {
    let builder = CircuitBuilder::new();
    let loaded = builder.load_chip(ChipKey::Custom(String::from("Not")));
    assert_eq!(loaded, None);
}

#[test]
fn given_nothing_saved_when_load_circuit_then_is_none() {
    let builder = CircuitBuilder::new();
    let loaded = builder.load_circuit("Not");
    assert_eq!(loaded, None);
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
    assert!(chips.contains(&ChipKey::Custom(String::from("Not"))));
}

#[test]
fn given_saved_chip_then_can_load_its_circuit() {
    let chip_description: ChipDescription = build_not();
    let mut builder = CircuitBuilder::new();
    builder.save_chip(chip_description, "Not");

    let circuit: Option<CircuitDescription> = builder.load_circuit("Not");
    assert_ne!(circuit, None)
}

#[test]
fn given_saved_not_when_loaded_then_behaves_as_not() {
    let chip_description: ChipDescription = build_not();
    let mut builder = CircuitBuilder::new();
    builder.save_chip(chip_description, "Not");

    let loaded = builder.load_chip(ChipKey::Custom(String::from("Not")));
    let chip_description = match loaded {
        None => panic!("Should not be None!"),
        Some(ChipValue::Basic(_)) => panic!("Expected a custom chip"),
        Some(ChipValue::Custom(description)) => description,
    };

    let mut circuit = Circuit::new();
    let ground = circuit.add_chip(GroundChip::new());
    let supply = circuit.add_chip(SupplyChip::new());
    let input = circuit.add_chip(InputChip::new());
    let output = circuit.add_chip(OutputChip::new());
    let not = circuit.add_custom_chip(CustomChip::new(chip_description.clone()));
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
