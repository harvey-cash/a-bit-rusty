

// [ ] CircuitDescription contains position and rotation of all Chips.
// [ ] CircuitDescription contains position of all Traces.
// [ ] CircuitDescription contains state of all Chips, Pins, and Traces.

use crate::chip::{
    chip::{ChipType, CustomChip, NAndChip}, 
    chip_description::{ChipAndPin, ChipDescription}, 
    circuit_description::CircuitDescription
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

#[test]
fn given_input_linked_to_output_then_valid() {
    let mut description = CircuitDescription::new();
    let input_id = description.add_chip(ChipType::Input);
    let output_id = description.add_chip(ChipType::Output);
    description.add_forward_link(ChipAndPin::new(input_id, 0), ChipAndPin::new(output_id, 0));
    assert_eq!(description.is_valid(), true);
}

#[test]
#[should_panic]
fn given_invalid_circuit_can_not_compile_chip() {
    let circuit_description = CircuitDescription::new();
    let chip_description: ChipDescription = circuit_description.compile_to_chip();
    CustomChip::new(chip_description);
}

#[test]
fn given_single_custom_chip_then_compiles_to_identical_description() {
    let nand_description = NAndChip::new().get_description();

    let mut circuit = CircuitDescription::new();
    let a = circuit.add_chip(ChipType::Input);
    let b = circuit.add_chip(ChipType::Input);
    let n = circuit.add_custom_chip(nand_description.clone());
    let y = circuit.add_chip(ChipType::Output);
    circuit.add_forward_link(ChipAndPin::new(a, 0), ChipAndPin::new(n, 0));
    circuit.add_forward_link(ChipAndPin::new(b, 0), ChipAndPin::new(n, 1));
    circuit.add_forward_link(ChipAndPin::new(n, 2), ChipAndPin::new(y, 0));

    let chip_description = circuit.compile_to_chip();
    assert_eq!(chip_description, nand_description);
}