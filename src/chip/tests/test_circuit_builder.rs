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
fn given_new_then_circuit_list_is_empty() {
    let builder = CircuitBuilder::new();
    let circuits = builder.get_circuit_list();
    assert!(circuits.is_empty());
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

fn build_not() -> CircuitDescription {
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

    circuit.get_description()
}

#[test]
fn given_saved_not_then_chip_list_contains_not() {
    let circuit_description = build_not();
    let chip_description: ChipDescription = ChipCompiler::compile(circuit_description.clone());
    let mut builder = CircuitBuilder::new();
    builder.save_chip("Not", chip_description, circuit_description);

    let chips = builder.get_chip_list();
    assert!(chips.contains(&ChipKey::Custom(String::from("Not"))));
}

#[test]
fn given_saved_new_circuit_then_success() {
    let circuit = build_not();
    let mut builder = CircuitBuilder::new();
    let success = builder.save_circuit(circuit, "Not");
    assert!(success);
}

#[test]
fn given_saved_circuit_then_circuit_list_contains_it() {
    let circuit = build_not();
    let mut builder = CircuitBuilder::new();
    builder.save_circuit(circuit, "Not");
    
    let circuits = builder.get_circuit_list();
    assert!(circuits.contains(&String::from("Not")));
}

#[test]
fn given_saved_circuit_then_can_load_it() {
    let description = build_not();
    let mut builder = CircuitBuilder::new();
    builder.save_circuit(description.clone(), "Not");
    
    let loaded: Option<&CircuitDescription> = builder.load_circuit("Not");
    assert_eq!(loaded, Some(&description));
}

#[test]
fn given_saved_chip_then_can_load_its_circuit() {
    let circuit_description = build_not();
    let chip_description: ChipDescription = ChipCompiler::compile(circuit_description.clone());
    let mut builder = CircuitBuilder::new();
    builder.save_chip("Not", chip_description, circuit_description.clone());

    let circuit: Option<&CircuitDescription> = builder.load_circuit("Not");
    assert_eq!(circuit, Some(&circuit_description));
}

#[test]
fn given_saved_chip_when_save_new_circuit_with_same_name_then_err() {
    let circuit_description = build_not();
    let chip_description: ChipDescription = ChipCompiler::compile(circuit_description.clone());
    let mut builder = CircuitBuilder::new();
    builder.save_chip("Not", chip_description, circuit_description.clone());

    let success = builder.save_circuit(CircuitDescription::new(), "Not");
    assert_eq!(success, false);
}

#[test]
fn given_saved_chip_when_save_new_circuit_with_same_name_then_not_overwritten() {
    let circuit_description = build_not();
    let chip_description: ChipDescription = ChipCompiler::compile(circuit_description.clone());
    let mut builder = CircuitBuilder::new();
    builder.save_chip("Not", chip_description, circuit_description.clone());

    builder.save_circuit(CircuitDescription::new(), "Not");

    let circuit = builder.load_circuit("Not");
    assert_eq!(circuit, Some(&circuit_description));
}

#[test]
fn given_saved_not_when_loaded_then_behaves_as_not() {
    let circuit_description = build_not();
    let chip_description: ChipDescription = ChipCompiler::compile(circuit_description.clone());
    let mut builder = CircuitBuilder::new();
    builder.save_chip("Not", chip_description, circuit_description.clone());

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
