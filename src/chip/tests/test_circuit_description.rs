

// [ ] CircuitDescription contains position and rotation of all Chips.
// [ ] CircuitDescription contains position of all Traces.
// [ ] CircuitDescription contains state of all Chips, Pins, and Traces.

use crate::chip::{
    chip::{ChipType, CustomChip, NAndChip}, circuit_description::CircuitDescription, compiler::ChipCompiler, types::*
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

#[test]
fn given_single_nand_chip_then_compiles_to_identical_description() {
    let nand = NAndChip::new().get_description();
    let layout = nand.get_layout();
    let ground = layout.get_pin_for(layout.ground_pins[0]);
    let supply = layout.get_pin_for(layout.supply_pins[0]);

    let mut circuit = CircuitDescription::new();
    let g = circuit.add_chip(ChipType::Ground);
    let s = circuit.add_chip(ChipType::Supply);
    let a = circuit.add_chip(ChipType::Input);
    let b = circuit.add_chip(ChipType::Input);
    let n = circuit.add_custom_chip(nand.clone());
    let y = circuit.add_chip(ChipType::Output);

    circuit.add_forward_link(ChipAndPin::new(g, 0), ChipAndPin::new(n, ground));
    circuit.add_forward_link(ChipAndPin::new(s, 0), ChipAndPin::new(n, supply));
    circuit.add_forward_link(ChipAndPin::new(a, 0), ChipAndPin::new(n, layout.input_pins[0]));
    circuit.add_forward_link(ChipAndPin::new(b, 0), ChipAndPin::new(n, layout.input_pins[1]));
    circuit.add_forward_link(ChipAndPin::new(n, layout.output_pins[0]), ChipAndPin::new(y, 0));

    let chip_description = ChipCompiler::compile(circuit);
    assert_eq!(chip_description, nand);
}