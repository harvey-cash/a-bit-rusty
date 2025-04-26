// CIRCUIT REQUIREMENTS:
// [ ] Circuits have an arrangement of Traces and Chips.
// [ ] Circuits have a 3D integer coordinate space. Z=0 is the "front layer".
// [ ] Circuits are created with one Ground and one Supply Chip.
// [ ] Circuits are created with one Input Chip and one Output Chip.
// [ ] The Supply value on a Circuit can be set to 0 or 1.
// [ ] The Input Chip values on a Circuit can be set to 0 or 1.
// [ ] The Output Chip values can be read from a Circuit.
// [ ] CircuitDescription can be read from a Circuit.
// [ ] CircuitDescription contains position and rotation of all Chips.
// [ ] CircuitDescription contains position of all Traces.
// [ ] CircuitDescription contains state of all Chips, Pins, and Traces.
// [ ] Chips can be added to a Circuit.
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
// [ ] If a Circuit is valid, it can be compiled to a ChipDescription.
// [ ] Circuits are invalid if any Trace is invalid.
// [ ] Compilation turns Traces into Links in the ChipDescription.
// [ ] Circuits can be ticked, even if invalid.
// [ ] Ticking a Circuit calls tick on all Chips with no Input pins connected to an invalid Trace.
// [ ] Tick is called on Chips in topological order, starting with the Inputs.
// [ ] Before a Chip is ticked, its Inputs are set to the values of the connected Traces.
// [ ] After a Chip is ticked, Traces connected to its Outputs have their value set.
// [ ] After a Circuit is ticked, all output Chips have their values set to the value of the connected Traces.
// [ ] Trace states can be read from a Circuit.
// [ ] All Chip Pin states can be read from a Circuit.

use crate::chip::{chip::{InputChip, OutputChip}, circuit::ChipAndPin, Circuit, CircuitDescription, GroundChip, NAndChip, SupplyChip, Tickable};

#[test]
fn given_just_output_then_output_is_0() {
    let mut circuit = Circuit::new(CircuitDescription::new());
    let output_id = circuit.add_chip(OutputChip::new());
    circuit.tick();
    assert_eq!(circuit.get_output(output_id), 0);
}

#[test]
fn given_supply_connected_then_output_is_1() {
    let mut circuit = Circuit::new(CircuitDescription::new());
    let supply_id = circuit.add_chip(SupplyChip::new());
    let output_id = circuit.add_chip(OutputChip::new());
    circuit.create_link(ChipAndPin::new(supply_id, 0), ChipAndPin::new(output_id, 0));
    circuit.tick();
    assert_eq!(circuit.get_output(output_id), 1);
}

#[test]
fn given_ground_connected_then_output_is_0() {
    let mut circuit = Circuit::new(CircuitDescription::new());
    let ground_id = circuit.add_chip(GroundChip::new());
    let output_id = circuit.add_chip(OutputChip::new());
    circuit.create_link(ChipAndPin::new(ground_id, 0), ChipAndPin::new(output_id, 0));
    circuit.tick();
    assert_eq!(circuit.get_output(output_id), 0);
}

#[test]
fn given_input_connected_when_0_then_output_is_0() {
    let mut circuit = Circuit::new(CircuitDescription::new());
    let input_id = circuit.add_chip(InputChip::new());
    let output_id = circuit.add_chip(OutputChip::new());
    circuit.set_input(input_id, 0);
    circuit.create_link(ChipAndPin::new(input_id, 0), ChipAndPin::new(output_id, 0));
    circuit.tick();
    assert_eq!(circuit.get_output(output_id), 0);
}

#[test]
fn given_input_connected_when_1_then_output_is_1() {
    let mut circuit = Circuit::new(CircuitDescription::new());
    let input_id = circuit.add_chip(InputChip::new());
    let output_id = circuit.add_chip(OutputChip::new());
    circuit.set_input(input_id, 1);
    circuit.create_link(ChipAndPin::new(input_id, 0), ChipAndPin::new(output_id, 0));
    circuit.tick();
    assert_eq!(circuit.get_output(output_id), 1);
}

#[test]
fn given_not_gate_when_input_1_then_output_0() {
    let mut circuit = Circuit::new(CircuitDescription::new());
    let input_id = circuit.add_chip(InputChip::new());
    let nand_id = circuit.add_chip(NAndChip::new());
    let output_id = circuit.add_chip(OutputChip::new());
    circuit.set_input(input_id, 0);
    circuit.create_link(ChipAndPin::new(input_id, 0), ChipAndPin::new(nand_id, 0));
    circuit.create_link(ChipAndPin::new(input_id, 0), ChipAndPin::new(nand_id, 1));
    circuit.create_link(ChipAndPin::new(nand_id, 0), ChipAndPin::new(output_id, 0));
    circuit.tick();
    assert_eq!(circuit.get_output(output_id), 1);
}
