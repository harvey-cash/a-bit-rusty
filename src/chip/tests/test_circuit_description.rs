

// [ ] CircuitDescription contains position and rotation of all Chips.
// [ ] CircuitDescription contains position of all Traces.
// [ ] CircuitDescription contains state of all Chips, Pins, and Traces.

use crate::chip::{
    chip::{ChipType, CustomChip, NAndChip}, circuit_description::CircuitDescription, types::*
};

#[test]
fn given_empty_then_invalid() {
    let description = CircuitDescription::new();
    assert_eq!(description.is_valid(), false);
}

#[test]
fn given_output_without_source_then_invalid() {
    let mut description = CircuitDescription::new();
    description.add_chip(ChipType::Output);
    assert_eq!(description.is_valid(), false);
}

// #[test]
// fn given_input_linked_to_output_then_valid() {
//     let mut description = CircuitDescription::new();
//     let input_id = description.add_chip(ChipType::Input);
//     let output_id = description.add_chip(ChipType::Output);
//     description.add_forward_link(ChipAndPin::new(input_id, 0), ChipAndPin::new(output_id, 0));
//     assert_eq!(description.is_valid(), true);
// }

#[test]
#[should_panic]
fn given_invalid_circuit_can_not_compile_chip() {
    let circuit_description = CircuitDescription::new();
    circuit_description.compile_to_chip();
}

#[test]
fn given_supply_not_added_then_invalid() {
    let mut circuit = CircuitDescription::new();
    circuit.add_chip(ChipType::Ground);
    let a = circuit.add_chip(ChipType::Input);
    let b = circuit.add_chip(ChipType::Input);
    let n = circuit.add_custom_chip(NAndChip::new().get_description());
    let y = circuit.add_chip(ChipType::Output);
    circuit.add_forward_link(ChipAndPin::new(a, 0), ChipAndPin::new(n, 2));
    circuit.add_forward_link(ChipAndPin::new(b, 0), ChipAndPin::new(n, 3));
    circuit.add_forward_link(ChipAndPin::new(n, 4), ChipAndPin::new(y, 0));
    
    assert_eq!(circuit.is_valid(), false);
}

#[test]
fn given_ground_not_added_then_invalid() {
    let mut circuit = CircuitDescription::new();
    circuit.add_chip(ChipType::Supply);
    let a = circuit.add_chip(ChipType::Input);
    let b = circuit.add_chip(ChipType::Input);
    let n = circuit.add_custom_chip(NAndChip::new().get_description());
    let y = circuit.add_chip(ChipType::Output);
    circuit.add_forward_link(ChipAndPin::new(a, 0), ChipAndPin::new(n, 2));
    circuit.add_forward_link(ChipAndPin::new(b, 0), ChipAndPin::new(n, 3));
    circuit.add_forward_link(ChipAndPin::new(n, 4), ChipAndPin::new(y, 0));
    
    assert_eq!(circuit.is_valid(), false);
}

// #[test]
// fn given_ground_and_supply_both_connected_then_valid()
// {
//     let mut circuit = CircuitDescription::new();
//     let ground = circuit.add_chip(ChipType::Ground);
//     let supply = circuit.add_chip(ChipType::Supply);
//     let a = circuit.add_chip(ChipType::Input);
//     let b = circuit.add_chip(ChipType::Input);
//     let n = circuit.add_custom_chip(NAndChip::new().get_description());
//     let y = circuit.add_chip(ChipType::Output);
//     circuit.add_forward_link(ChipAndPin::new(ground, 0), ChipAndPin::new(n, CustomChip::GROUND_PIN));
//     circuit.add_forward_link(ChipAndPin::new(supply, 0), ChipAndPin::new(n, CustomChip::SUPPLY_PIN));
//     circuit.add_forward_link(ChipAndPin::new(a, 0), ChipAndPin::new(n, 2));
//     circuit.add_forward_link(ChipAndPin::new(b, 0), ChipAndPin::new(n, 3));
//     circuit.add_forward_link(ChipAndPin::new(n, 4), ChipAndPin::new(y, 0));
    
//     assert_eq!(circuit.is_valid(), true);
// }

// #[test]
// fn given_single_custom_chip_then_compiles_to_identical_description() {
//     let nand_description = NAndChip::new().get_description();

//     let mut circuit = CircuitDescription::new();
//     circuit.add_chip(ChipType::Supply);
//     let a = circuit.add_chip(ChipType::Input);
//     let b = circuit.add_chip(ChipType::Input);
//     let n = circuit.add_custom_chip(nand_description.clone());
//     let y = circuit.add_chip(ChipType::Output);
//     circuit.add_forward_link(ChipAndPin::new(a, 0), ChipAndPin::new(n, 2));
//     circuit.add_forward_link(ChipAndPin::new(b, 0), ChipAndPin::new(n, 3));
//     circuit.add_forward_link(ChipAndPin::new(n, 4), ChipAndPin::new(y, 0));

//     let chip_description = circuit.compile_to_chip();
//     assert_eq!(chip_description, nand_description);
// }