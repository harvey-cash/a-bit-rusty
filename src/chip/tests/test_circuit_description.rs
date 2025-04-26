

// [ ] CircuitDescription contains position and rotation of all Chips.
// [ ] CircuitDescription contains position of all Traces.
// [ ] CircuitDescription contains state of all Chips, Pins, and Traces.

use crate::chip::{
    chip::{ChipType, CustomChip}, 
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
