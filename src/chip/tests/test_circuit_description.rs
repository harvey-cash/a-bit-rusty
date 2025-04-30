

// [ ] CircuitDescription contains position and rotation of all Chips.
// [ ] CircuitDescription contains position of all Traces.
// [ ] CircuitDescription contains state of all Chips, Pins, and Traces.

use crate::{chip::{
    chip::{ChipType, CustomChip, NAndChip}, circuit_description::CircuitDescription, compiler::ChipCompiler, types::*
}, chip_pin};

#[test]
fn given_wrapped_nand_then_compiled_description_is_valid() {
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

    circuit.add_forward_link(chip_pin!(g, 0), chip_pin!(n, ground));
    circuit.add_forward_link(chip_pin!(s, 0), chip_pin!(n, supply));
    circuit.add_forward_link(chip_pin!(a, 0), chip_pin!(n, layout.input_pins[0]));
    circuit.add_forward_link(chip_pin!(b, 0), chip_pin!(n, layout.input_pins[1]));
    circuit.add_forward_link(chip_pin!(n, layout.output_pins[0]), chip_pin!(y, 0));

    let chip_description = ChipCompiler::compile(circuit);
    assert_eq!(chip_description.is_valid(), true);
}

#[test]
fn given_wrapped_nand_then_compiled_description_is_identical() {
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

    circuit.add_forward_link(chip_pin!(g, 0), chip_pin!(n, ground));
    circuit.add_forward_link(chip_pin!(s, 0), chip_pin!(n, supply));
    circuit.add_forward_link(chip_pin!(a, 0), chip_pin!(n, layout.input_pins[0]));
    circuit.add_forward_link(chip_pin!(b, 0), chip_pin!(n, layout.input_pins[1]));
    circuit.add_forward_link(chip_pin!(n, layout.output_pins[0]), chip_pin!(y, 0));

    let chip_description = ChipCompiler::compile(circuit);
    assert_eq!(chip_description, nand);
}

#[test]
fn given_not_chip_then_compiled_description_is_valid() {
    let nand = NAndChip::new().get_description();
    let layout = nand.get_layout();
    let ground = layout.get_pin_for(layout.ground_pins[0]);
    let supply = layout.get_pin_for(layout.supply_pins[0]);

    let mut circuit = CircuitDescription::new();
    let g = circuit.add_chip(ChipType::Ground);
    let s = circuit.add_chip(ChipType::Supply);
    let i = circuit.add_chip(ChipType::Input);
    let n = circuit.add_custom_chip(nand.clone());
    let y = circuit.add_chip(ChipType::Output);

    circuit.add_forward_link(chip_pin!(g, 0), chip_pin!(n, ground));
    circuit.add_forward_link(chip_pin!(s, 0), chip_pin!(n, supply));
    circuit.add_forward_link(chip_pin!(i, 0), chip_pin!(n, layout.input_pins[0]));
    circuit.add_forward_link(chip_pin!(i, 0), chip_pin!(n, layout.input_pins[1]));
    circuit.add_forward_link(chip_pin!(n, layout.output_pins[0]), chip_pin!(y, 0));

    let not_description = ChipCompiler::compile(circuit);
    assert_eq!(not_description.is_valid(), true);
}

#[test]
fn given_wrapped_not_then_compiled_description_is_identical() {
    let nand = NAndChip::new().get_description();
    let layout = nand.get_layout();
    let ground = layout.get_pin_for(layout.ground_pins[0]);
    let supply = layout.get_pin_for(layout.supply_pins[0]);

    let mut circuit = CircuitDescription::new();
    let g = circuit.add_chip(ChipType::Ground);
    let s = circuit.add_chip(ChipType::Supply);
    let i = circuit.add_chip(ChipType::Input);
    let n = circuit.add_custom_chip(nand.clone());
    let y = circuit.add_chip(ChipType::Output);

    circuit.add_forward_link(chip_pin!(g, 0), chip_pin!(n, ground));
    circuit.add_forward_link(chip_pin!(s, 0), chip_pin!(n, supply));
    circuit.add_forward_link(chip_pin!(i, 0), chip_pin!(n, layout.input_pins[0]));
    circuit.add_forward_link(chip_pin!(i, 0), chip_pin!(n, layout.input_pins[1]));
    circuit.add_forward_link(chip_pin!(n, layout.output_pins[0]), chip_pin!(y, 0));

    let not_description = ChipCompiler::compile(circuit);
    let layout2 = not_description.get_layout();

    let mut circuit2 = CircuitDescription::new();
    let g = circuit2.add_chip(ChipType::Ground);
    let s = circuit2.add_chip(ChipType::Supply);
    let i = circuit2.add_chip(ChipType::Input);
    let n = circuit2.add_custom_chip(not_description.clone());
    let y = circuit2.add_chip(ChipType::Output);

    circuit2.add_forward_link(chip_pin!(g, 0), chip_pin!(n, ground));
    circuit2.add_forward_link(chip_pin!(s, 0), chip_pin!(n, supply));
    circuit2.add_forward_link(chip_pin!(i, 0), chip_pin!(n, layout2.input_pins[0]));
    circuit2.add_forward_link(chip_pin!(n, layout2.output_pins[0]), chip_pin!(y, 0));

    let wrapped_not_description = ChipCompiler::compile(circuit2);
    assert_eq!(wrapped_not_description, not_description);
}