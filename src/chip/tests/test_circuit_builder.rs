// ToDo:
// [ ] Can load circuits from the circuit list for editing
// [ ] Given a circuit already compiled a chip, can choose to overwrite on save
// [ ] Given a circuit already compiled a chip, can choose to save-as new
// [ ] Can save circuits to the circuit list

use crate::{
    chip::{
        chip::{
            ChipType, CustomChip, GroundChip, InputChip, NAndChip, OutputChip, SupplyChip, Tickable,
        }, chip_description::ChipDescription, circuit::Circuit, chip_database::{ChipKey, ChipValue, ChipDatabase}, circuit_description::CircuitDescription, compiler::ChipCompiler, types::*
    },
    chip_pin,
};

#[test]
fn given_new_then_chip_list_contains_fundamentals() {
    let database = ChipDatabase::new();
    let chips = database.get_chip_list();
    assert!(chips.contains(&ChipKey::Basic(ChipType::Ground)));
    assert!(chips.contains(&ChipKey::Basic(ChipType::Supply)));
    assert!(chips.contains(&ChipKey::Basic(ChipType::Input)));
    assert!(chips.contains(&ChipKey::Basic(ChipType::Output)));
    assert!(chips.contains(&ChipKey::Custom(String::from("NAnd"))));
}

#[test]
fn given_new_when_delete_basic_chip_then_err()
{
    let mut database = ChipDatabase::new();
    let success = database.delete_chip(&ChipKey::Basic(ChipType::Ground));
    assert_eq!(success, false);
}

#[test]
fn given_new_then_circuit_list_is_empty() {
    let database = ChipDatabase::new();
    let circuits = database.get_circuit_list();
    assert!(circuits.is_empty());
}

#[test]
fn given_nothing_saved_when_load_chip_then_is_none() {
    let database = ChipDatabase::new();
    let loaded = database.load_chip(ChipKey::Custom(String::from("Not")));
    assert_eq!(loaded, None);
}

#[test]
fn given_nothing_saved_when_load_circuit_then_is_none() {
    let database = ChipDatabase::new();
    let loaded = database.load_circuit("Not");
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
fn given_saved_chip_then_chip_list_contains_it() {
    let circuit_description = build_not();
    let chip_description: ChipDescription = ChipCompiler::compile(circuit_description.clone());
    let mut database = ChipDatabase::new();
    database.save_chip("Not", chip_description, circuit_description);

    let chips = database.get_chip_list();
    assert!(chips.contains(&ChipKey::Custom(String::from("Not"))));
}

#[test]
fn given_saved_chip_when_deleted_then_success() {
    let circuit_description = build_not();
    let chip_description: ChipDescription = ChipCompiler::compile(circuit_description.clone());
    let mut database = ChipDatabase::new();
    database.save_chip("Not", chip_description, circuit_description);

    let success = database.delete_chip(&ChipKey::Custom(String::from("Not")));
    assert!(success);
}

#[test]
fn given_saved_chip_when_deleted_then_chip_list_omits_it() {
    let circuit_description = build_not();
    let chip_description: ChipDescription = ChipCompiler::compile(circuit_description.clone());
    let mut database = ChipDatabase::new();
    database.save_chip("Not", chip_description, circuit_description);

    let key = ChipKey::Custom(String::from("Not"));
    database.delete_chip(&key);

    let chips = database.get_chip_list();
    assert_eq!(chips.contains(&key), false);
}

#[test]
fn given_saved_new_circuit_then_success() {
    let circuit = build_not();
    let mut database = ChipDatabase::new();
    let success = database.save_circuit(circuit, "Not");
    assert!(success);
}

#[test]
fn given_saved_circuit_then_circuit_list_contains_it() {
    let circuit = build_not();
    let mut database = ChipDatabase::new();
    database.save_circuit(circuit, "Not");
    
    let circuits = database.get_circuit_list();
    assert!(circuits.contains(&String::from("Not")));
}

#[test]
fn given_saved_circuit_then_can_load_it() {
    let description = build_not();
    let mut database = ChipDatabase::new();
    database.save_circuit(description.clone(), "Not");
    
    let loaded: Option<&CircuitDescription> = database.load_circuit("Not");
    assert_eq!(loaded, Some(&description));
}

#[test]
fn given_deleted_circuit_then_circuit_list_omits_it() {
    let circuit = build_not();
    let mut database = ChipDatabase::new();
    database.save_circuit(circuit, "Not");

    database.delete_circuit("Not");
    
    let circuits = database.get_circuit_list();
    assert_eq!(circuits.contains(&String::from("Not")), false);
}

#[test]
fn given_saved_chip_then_can_load_its_circuit() {
    let circuit_description = build_not();
    let chip_description: ChipDescription = ChipCompiler::compile(circuit_description.clone());
    let mut database = ChipDatabase::new();
    database.save_chip("Not", chip_description, circuit_description.clone());

    let circuit: Option<&CircuitDescription> = database.load_circuit("Not");
    assert_eq!(circuit, Some(&circuit_description));
}

#[test]
fn given_saved_chip_when_save_new_circuit_with_same_name_then_err() {
    let circuit_description = build_not();
    let chip_description: ChipDescription = ChipCompiler::compile(circuit_description.clone());
    let mut database = ChipDatabase::new();
    database.save_chip("Not", chip_description, circuit_description.clone());

    let success = database.save_circuit(CircuitDescription::new(), "Not");
    assert_eq!(success, false);
}

#[test]
fn given_saved_chip_when_save_new_circuit_with_same_name_then_not_overwritten() {
    let circuit_description = build_not();
    let chip_description: ChipDescription = ChipCompiler::compile(circuit_description.clone());
    let mut database = ChipDatabase::new();
    database.save_chip("Not", chip_description, circuit_description.clone());

    database.save_circuit(CircuitDescription::new(), "Not");

    let circuit = database.load_circuit("Not");
    assert_eq!(circuit, Some(&circuit_description));
}

#[test]
fn given_saved_not_when_loaded_then_behaves_as_not() {
    let circuit_description = build_not();
    let chip_description: ChipDescription = ChipCompiler::compile(circuit_description.clone());
    let mut database = ChipDatabase::new();
    database.save_chip("Not", chip_description, circuit_description.clone());

    let loaded = database.load_chip(ChipKey::Custom(String::from("Not")));
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
