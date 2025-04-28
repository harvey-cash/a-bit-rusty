// CIRCUIT REQUIREMENTS:
// [ ] Circuits have an arrangement of Traces and Chips.
// [ ] Circuits have a 3D integer coordinate space. Z=0 is the "front layer".
// [ ] Circuits are created with one Ground and one Supply Chip.
// [ ] Circuits are created with one Input Chip and one Output Chip.
// [ ] Chips are placed on the front layer.
// [ ] Chips occupy a non-zero 2D area of points on the board.
// [ ] Chips can be rotated in 90 degree increments.
// [ ] Chips may not overlap other Chips.
// [ ] Chips can be removed from a Circuit.
// [ ] Pins exist at points adjacent to their Chip's surface area on the front layer.
// [ ] Pins are two dimensional lines in Z, occupying Z = [0, 1] at a single XY co-ordinate.
// [ ] Pins may not be coincident with other Pins (of other Chips).
// [ ] TraceSegments can be added to a Circuit.
// [ ] TraceSegments can not overlap any point covered by a Chip (on the front layer).
// [ ] TraceSegments can intersect Pins.
// [ ] A Trace Via can not be coincident with a Pin's XY coordinates.
// [ ] TraceSegments can be deleted from a Circuit.
// [ ] When multiple Pins intersect TraceSegments belonging to the same Trace, they are connected.
// [ ] Traces have a state value which defaults to 0.
// [ ] If a Trace intersects one Output Pin, its value equals the state of the Output Pin.
// [ ] If a Trace intersects multiple Output Pins it is invalid.
// [ ] Circuits are invalid if any Trace is invalid.
// [ ] Compilation turns Traces into Links in the ChipDescription.
// [ ] Circuits can be ticked, even if invalid.
// [ ] Ticking a Circuit calls tick on all Chips with no Input pins connected to an invalid Trace.
// [ ] Before a Chip is ticked, its Inputs are set to the values of the connected Traces.
// [ ] After a Chip is ticked, Traces connected to its Outputs have their value set.
// [ ] After a Circuit is ticked, all output Chips have their values set to the value of the connected Traces.
// [ ] Trace states can be read from a Circuit.
// [ ] All Chip Pin states can be read from a Circuit.
// [ ] CircuitDescription can be read from a Circuit.
// [ ] Circuit can be constructed from a ChipDescription.

use crate::chip::{
    types::*,
    chip::{Chip, CustomChip, GroundChip, InputChip, NAndChip, OutputChip, SupplyChip, Tickable}, 
    chip_description::ChipDescription, 
    circuit::Circuit
};


#[test]
fn given_just_output_then_output_is_0() {
    let mut circuit = Circuit::new();
    let output_id = circuit.add_chip(OutputChip::new());
    circuit.tick();
    assert_eq!(circuit.get_output(output_id), 0);
}

#[test]
fn given_supply_connected_then_output_is_1() {
    let mut circuit = Circuit::new();
    let supply_id = circuit.add_chip(SupplyChip::new());
    let output_id = circuit.add_chip(OutputChip::new());
    circuit.create_link(ChipAndPin::new(supply_id, 0), ChipAndPin::new(output_id, 0));
    circuit.tick();
    assert_eq!(circuit.get_output(output_id), 1);
}

#[test]
fn given_supply_disconnected_then_output_is_0() {
    let mut circuit = Circuit::new();
    let supply_id = circuit.add_chip(SupplyChip::new());
    let output_id = circuit.add_chip(OutputChip::new());
    circuit.create_link(ChipAndPin::new(supply_id, 0), ChipAndPin::new(output_id, 0));
    circuit.tick();
    circuit.delete_link(ChipAndPin::new(supply_id, 0), ChipAndPin::new(output_id, 0));
    circuit.tick();
    assert_eq!(circuit.get_output(output_id), 0);
}

#[test]
fn given_supply_off_then_output_is_0() {
    let mut circuit = Circuit::new();
    let supply_id = circuit.add_chip(SupplyChip::new());
    let output_id = circuit.add_chip(OutputChip::new());
    circuit.set_supply(supply_id, 0);
    circuit.create_link(ChipAndPin::new(supply_id, 0), ChipAndPin::new(output_id, 0));
    circuit.tick();
    assert_eq!(circuit.get_output(output_id), 0);
}

#[test]
fn given_ground_connected_then_output_is_0() {
    let mut circuit = Circuit::new();
    let ground_id = circuit.add_chip(GroundChip::new());
    let output_id = circuit.add_chip(OutputChip::new());
    circuit.create_link(ChipAndPin::new(ground_id, 0), ChipAndPin::new(output_id, 0));
    circuit.tick();
    assert_eq!(circuit.get_output(output_id), 0);
}

#[test]
fn given_input_connected_when_0_then_output_is_0() {
    let mut circuit = Circuit::new();
    let input_id = circuit.add_chip(InputChip::new());
    let output_id = circuit.add_chip(OutputChip::new());
    circuit.set_input(input_id, 0);
    circuit.create_link(ChipAndPin::new(input_id, 0), ChipAndPin::new(output_id, 0));
    circuit.tick();
    assert_eq!(circuit.get_output(output_id), 0);
}

#[test]
fn given_input_connected_when_1_then_output_is_1() {
    let mut circuit = Circuit::new();
    let input_id = circuit.add_chip(InputChip::new());
    let output_id = circuit.add_chip(OutputChip::new());
    circuit.set_input(input_id, 1);
    circuit.create_link(ChipAndPin::new(input_id, 0), ChipAndPin::new(output_id, 0));
    circuit.tick();
    assert_eq!(circuit.get_output(output_id), 1);
}

#[test]
fn given_not_gate_when_input_0_then_output_1() {
    let mut circuit = Circuit::new();
    let input_id = circuit.add_chip(InputChip::new());

    let mut chip = NAndChip::new();
    chip.write_pin(chip.get_supply_pin(), 1);
    let nand_id = circuit.add_chip(chip);
    
    let output_id = circuit.add_chip(OutputChip::new());
    circuit.set_input(input_id, 0);
    circuit.create_link(ChipAndPin::new(input_id, 0), ChipAndPin::new(nand_id, 2));
    circuit.create_link(ChipAndPin::new(input_id, 0), ChipAndPin::new(nand_id, 3));
    circuit.create_link(ChipAndPin::new(nand_id, 4), ChipAndPin::new(output_id, 0));
    circuit.tick();
    assert_eq!(circuit.get_output(output_id), 1);
}

#[test]
fn given_not_gate_when_input_1_then_output_0() {
    let mut circuit = Circuit::new();
    let input_id = circuit.add_chip(InputChip::new());

    let mut chip = NAndChip::new();
    chip.write_pin(chip.get_supply_pin(), 1);
    let nand_id = circuit.add_chip(chip);
    
    let output_id = circuit.add_chip(OutputChip::new());
    circuit.set_input(input_id, 1);
    circuit.create_link(ChipAndPin::new(input_id, 0), ChipAndPin::new(nand_id, 2));
    circuit.create_link(ChipAndPin::new(input_id, 0), ChipAndPin::new(nand_id, 3));
    circuit.create_link(ChipAndPin::new(nand_id, 4), ChipAndPin::new(output_id, 0));
    circuit.tick();
    assert_eq!(circuit.get_output(output_id), 0);
}

// #[test]
// fn given_valid_not_gate_can_compile_to_valid_chip_description() {
//     let mut circuit = Circuit::new();
//     let ground_id = circuit.add_chip(GroundChip::new());
//     let supply_id = circuit.add_chip(SupplyChip::new());
//     let input_id = circuit.add_chip(InputChip::new());
//     let nand_id = circuit.add_chip(NAndChip::new());
//     let output_id = circuit.add_chip(OutputChip::new());
//     circuit.create_link(ChipAndPin::new(ground_id, 0), ChipAndPin::new(nand_id, CustomChip::GROUND_PIN));
//     circuit.create_link(ChipAndPin::new(supply_id, 0), ChipAndPin::new(nand_id, chip.get_supply_pin()));
//     circuit.create_link(ChipAndPin::new(input_id, 0), ChipAndPin::new(nand_id, 2));
//     circuit.create_link(ChipAndPin::new(input_id, 0), ChipAndPin::new(nand_id, 3));
//     circuit.create_link(ChipAndPin::new(nand_id, 4), ChipAndPin::new(output_id, 0));
//     let description: ChipDescription = circuit.compile_to_chip();
//     assert_eq!(description.is_valid(), true);
// }

// #[test]
// fn given_valid_not_gate_compiled_chip_is_a_not_gate() {
//     let mut circuit = Circuit::new();
//     let ground_id = circuit.add_chip(GroundChip::new());
//     let supply_id = circuit.add_chip(SupplyChip::new());
//     let input_id = circuit.add_chip(InputChip::new());
//     let nand_id = circuit.add_chip(NAndChip::new());
//     let output_id = circuit.add_chip(OutputChip::new());
//     circuit.create_link(ChipAndPin::new(ground_id, 0), ChipAndPin::new(nand_id, CustomChip::GROUND_PIN));
//     circuit.create_link(ChipAndPin::new(supply_id, 0), ChipAndPin::new(nand_id, chip.get_supply_pin()));
//     circuit.create_link(ChipAndPin::new(input_id, 0), ChipAndPin::new(nand_id, 2));
//     circuit.create_link(ChipAndPin::new(input_id, 0), ChipAndPin::new(nand_id, 3));
//     circuit.create_link(ChipAndPin::new(nand_id, 4), ChipAndPin::new(output_id, 0));
//     let description: ChipDescription = circuit.compile_to_chip();

//     let mut chip = CustomChip::new(description);
//     chip.write_pin(chip.get_supply_pin(), 1);

//     chip.write_pin(2, 0);
//     chip.tick();
//     assert_eq!(chip.read_pin(3), 1);

//     chip.write_pin(2, 1);
//     chip.tick();
//     assert_eq!(chip.read_pin(3), 0);
// }
