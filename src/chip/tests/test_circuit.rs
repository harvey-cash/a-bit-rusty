use crate::{
    chip::{
        chip::{
            Chip, ChipType, CustomChip, GroundChip, InputChip, NAndChip, OutputChip, SupplyChip,
            Tickable,
        },
        chip_description::ChipDescription,
        circuit::Circuit,
        circuit_description::CircuitDescription,
        compiler::ChipCompiler,
        types::*,
    },
    chip_pin,
};

#[test]
fn given_description_then_description_is_equal() {
    let description = CircuitDescription::new("Test Circuit");
    let circuit = Circuit::load(description.clone());
    assert_eq!(circuit.get_description(), description);
}

#[test]
fn given_loaded_not_description_when_input_0_then_output_1() {
    let mut description = CircuitDescription::new("Test Circuit");
    let ground = description.add_chip(ChipType::Ground);
    let supply = description.add_chip(ChipType::Supply);
    let input = description.add_chip(ChipType::Input);
    let nand = description.add_custom_chip(NAndChip::new().get_description());
    let output = description.add_chip(ChipType::Output);

    description.add_forward_link(chip_pin!(ground, 0), chip_pin!(nand, 0));
    description.add_forward_link(chip_pin!(supply, 0), chip_pin!(nand, 1));
    description.add_forward_link(chip_pin!(input, 0), chip_pin!(nand, 2));
    description.add_forward_link(chip_pin!(input, 0), chip_pin!(nand, 3));
    description.add_forward_link(chip_pin!(nand, 4), chip_pin!(output, 0));

    let mut circuit = Circuit::load(description);
    circuit.set_supply(supply, 1);
    circuit.set_input(input, 0);
    circuit.tick();
    assert_eq!(circuit.get_output(output), 1);
}

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
    circuit.set_supply(supply_id, 1);
    circuit.create_link(chip_pin!(supply_id, 0), chip_pin!(output_id, 0));
    circuit.tick();
    assert_eq!(circuit.get_output(output_id), 1);
}

#[test]
fn given_supply_disconnected_then_output_is_0() {
    let mut circuit = Circuit::new();
    let supply_id = circuit.add_chip(SupplyChip::new());
    let output_id = circuit.add_chip(OutputChip::new());
    circuit.set_supply(supply_id, 1);
    circuit.create_link(chip_pin!(supply_id, 0), chip_pin!(output_id, 0));
    circuit.tick();
    circuit.delete_link(chip_pin!(supply_id, 0), chip_pin!(output_id, 0));
    circuit.tick();
    assert_eq!(circuit.get_output(output_id), 0);
}

#[test]
fn given_supply_off_then_output_is_0() {
    let mut circuit = Circuit::new();
    let supply_id = circuit.add_chip(SupplyChip::new());
    let output_id = circuit.add_chip(OutputChip::new());
    circuit.set_supply(supply_id, 0);
    circuit.create_link(chip_pin!(supply_id, 0), chip_pin!(output_id, 0));
    circuit.tick();
    assert_eq!(circuit.get_output(output_id), 0);
}

#[test]
fn given_ground_connected_then_output_is_0() {
    let mut circuit = Circuit::new();
    let ground_id = circuit.add_chip(GroundChip::new());
    let output_id = circuit.add_chip(OutputChip::new());
    circuit.create_link(chip_pin!(ground_id, 0), chip_pin!(output_id, 0));
    circuit.tick();
    assert_eq!(circuit.get_output(output_id), 0);
}

#[test]
fn given_input_connected_when_0_then_output_is_0() {
    let mut circuit = Circuit::new();
    let input_id = circuit.add_chip(InputChip::new());
    let output_id = circuit.add_chip(OutputChip::new());
    circuit.set_input(input_id, 0);
    circuit.create_link(chip_pin!(input_id, 0), chip_pin!(output_id, 0));
    circuit.tick();
    assert_eq!(circuit.get_output(output_id), 0);
}

#[test]
fn given_input_connected_when_1_then_output_is_1() {
    let mut circuit = Circuit::new();
    let input_id = circuit.add_chip(InputChip::new());
    let output_id = circuit.add_chip(OutputChip::new());
    circuit.set_input(input_id, 1);
    circuit.create_link(chip_pin!(input_id, 0), chip_pin!(output_id, 0));
    circuit.tick();
    assert_eq!(circuit.get_output(output_id), 1);
}

#[test]
fn given_nand_when_supply_1_then_output_pin_1() {
    println!("{:?}", NAndChip::new().get_description());

    let mut circuit = Circuit::new();
    let ground = circuit.add_chip(GroundChip::new());
    let supply = circuit.add_chip(SupplyChip::new());
    let nand = circuit.add_custom_chip(NAndChip::new());
    circuit.create_link(chip_pin!(ground, 0), chip_pin!(nand, 0));
    circuit.create_link(chip_pin!(supply, 0), chip_pin!(nand, 1));
    circuit.set_supply(supply, 1);
    circuit.tick();
    let pin_states = circuit.get_chip_pin_states();
    let output_pin_value = pin_states.get(&chip_pin!(nand, 4)).unwrap();
    assert_eq!(*output_pin_value, 1);
}

fn build_not() -> (Circuit, usize, usize, usize) {
    let mut circuit = Circuit::new();
    let ground = circuit.add_chip(GroundChip::new());
    let supply = circuit.add_chip(SupplyChip::new());
    let input_id = circuit.add_chip(InputChip::new());
    let nand_id = circuit.add_custom_chip(NAndChip::new());
    let output_id = circuit.add_chip(OutputChip::new());

    circuit.create_link(chip_pin!(ground, 0), chip_pin!(nand_id, 0));
    circuit.create_link(chip_pin!(supply, 0), chip_pin!(nand_id, 1));
    circuit.create_link(chip_pin!(input_id, 0), chip_pin!(nand_id, 2));
    circuit.create_link(chip_pin!(input_id, 0), chip_pin!(nand_id, 3));
    circuit.create_link(chip_pin!(nand_id, 4), chip_pin!(output_id, 0));

    return (circuit, supply, input_id, output_id);
}

#[test]
fn given_not_gate_when_input_0_then_output_1() {
    let (mut circuit, supply, input_id, output_id) = build_not();
    circuit.set_supply(supply, 1);
    circuit.set_input(input_id, 0);
    circuit.tick();
    assert_eq!(circuit.get_output(output_id), 1);
}

#[test]
fn given_not_gate_when_input_1_then_output_0() {
    let (mut circuit, supply, input_id, output_id) = build_not();
    circuit.set_supply(supply, 1);
    circuit.set_input(input_id, 1);
    circuit.tick();
    assert_eq!(circuit.get_output(output_id), 0);
}

#[test]
fn given_compiled_not_circuit_then_functions_as_not_chip() {
    let (circuit, _, _, _) = build_not();
    let description: ChipDescription = ChipCompiler::compile(circuit.get_description());

    let mut chip = CustomChip::new(description);
    chip.write_pin(chip.get_supply_pin(), 1);

    chip.write_pin(2, 0);
    chip.tick();
    assert_eq!(chip.read_pin(3), 1);

    chip.write_pin(2, 1);
    chip.tick();
    assert_eq!(chip.read_pin(3), 0);
}

fn build_xor() -> (Circuit, usize, usize, usize) {
    let mut circuit = Circuit::new();
    let ground = circuit.add_chip(GroundChip::new());
    let supply = circuit.add_chip(SupplyChip::new());
    let in_a = circuit.add_chip(InputChip::new());
    let in_b = circuit.add_chip(InputChip::new());
    let nand_1 = circuit.add_custom_chip(NAndChip::new());
    let nand_2 = circuit.add_custom_chip(NAndChip::new());
    let nand_3 = circuit.add_custom_chip(NAndChip::new());
    let nand_4 = circuit.add_custom_chip(NAndChip::new());
    let output = circuit.add_chip(OutputChip::new());

    circuit.create_link(chip_pin!(ground, 0), chip_pin!(nand_1, 0));
    circuit.create_link(chip_pin!(ground, 0), chip_pin!(nand_2, 0));
    circuit.create_link(chip_pin!(ground, 0), chip_pin!(nand_3, 0));
    circuit.create_link(chip_pin!(ground, 0), chip_pin!(nand_4, 0));
    circuit.create_link(chip_pin!(supply, 0), chip_pin!(nand_1, 1));
    circuit.create_link(chip_pin!(supply, 0), chip_pin!(nand_2, 1));
    circuit.create_link(chip_pin!(supply, 0), chip_pin!(nand_3, 1));
    circuit.create_link(chip_pin!(supply, 0), chip_pin!(nand_4, 1));

    circuit.create_link(chip_pin!(in_a, 0), chip_pin!(nand_1, 2));
    circuit.create_link(chip_pin!(in_a, 0), chip_pin!(nand_2, 2));

    circuit.create_link(chip_pin!(in_b, 0), chip_pin!(nand_1, 3));
    circuit.create_link(chip_pin!(in_b, 0), chip_pin!(nand_3, 2));

    circuit.create_link(chip_pin!(nand_1, 4), chip_pin!(nand_2, 3));
    circuit.create_link(chip_pin!(nand_1, 4), chip_pin!(nand_3, 3));

    circuit.create_link(chip_pin!(nand_2, 4), chip_pin!(nand_4, 2));
    circuit.create_link(chip_pin!(nand_3, 4), chip_pin!(nand_4, 3));

    circuit.create_link(chip_pin!(nand_4, 4), chip_pin!(output, 0));

    return (circuit, in_a, in_b, output);
}

#[test]
fn given_xor_when_both_inputs_0_then_output_0() {
    let (mut circuit, in_a, in_b, output) = build_xor();
    circuit.set_supply(circuit.get_supply_ids()[0], 1);

    circuit.set_input(in_a, 0);
    circuit.set_input(in_b, 0);
    circuit.tick();
    circuit.tick();
    assert_eq!(circuit.get_output(output), 0);
}

#[test]
fn given_xor_when_inputs_differ_then_output_1() {
    let (mut circuit, in_a, in_b, output) = build_xor();
    circuit.set_supply(circuit.get_supply_ids()[0], 1);

    circuit.set_input(in_a, 1);
    circuit.set_input(in_b, 0);
    circuit.tick();
    circuit.tick();
    assert_eq!(circuit.get_output(output), 1);

    circuit.set_input(in_a, 1);
    circuit.set_input(in_b, 0);
    circuit.tick();
    circuit.tick();
    assert_eq!(circuit.get_output(output), 1);
}

#[test]
fn given_xor_when_both_inputs_1_then_output_0() {
    let (mut circuit, in_a, in_b, output) = build_xor();
    circuit.set_supply(circuit.get_supply_ids()[0], 1);

    circuit.set_input(in_a, 1);
    circuit.set_input(in_b, 1);
    circuit.tick();
    circuit.tick();
    assert_eq!(circuit.get_output(output), 0);
}

#[test]
fn given_compiled_xor_when_both_inputs_0_then_output_0() {
    let (circuit, _, _, _) = build_xor();
    let description: ChipDescription = ChipCompiler::compile(circuit.get_description());
    let mut xor = CustomChip::new(description);

    let layout = xor.get_description().get_layout();
    xor.write_pin(xor.get_supply_pin(), 1);

    xor.write_pin(layout.input_pins[0], 0);
    xor.write_pin(layout.input_pins[1], 0);
    xor.tick();
    xor.tick();
    assert_eq!(xor.read_pin(layout.output_pins[0]), 0);
}

#[test]
fn given_compiled_xor_when_inputs_differ_then_output_1() {
    let (circuit, _, _, _) = build_xor();
    let description: ChipDescription = ChipCompiler::compile(circuit.get_description());
    let mut xor = CustomChip::new(description);

    let layout = xor.get_description().get_layout();
    xor.write_pin(xor.get_supply_pin(), 1);

    xor.write_pin(layout.input_pins[0], 0);
    xor.write_pin(layout.input_pins[1], 1);
    xor.tick();
    xor.tick();
    assert_eq!(xor.read_pin(layout.output_pins[0]), 1);

    xor.write_pin(layout.input_pins[0], 1);
    xor.write_pin(layout.input_pins[1], 0);
    xor.tick();
    xor.tick();
    assert_eq!(xor.read_pin(layout.output_pins[0]), 1);
}

#[test]
fn given_compiled_xor_when_both_inputs_1_then_output_0() {
    let (circuit, _, _, _) = build_xor();
    let description: ChipDescription = ChipCompiler::compile(circuit.get_description());
    let mut xor = CustomChip::new(description);

    let layout = xor.get_description().get_layout();
    xor.write_pin(xor.get_supply_pin(), 1);

    xor.write_pin(layout.input_pins[0], 1);
    xor.write_pin(layout.input_pins[1], 1);
    xor.tick();
    xor.tick();
    assert_eq!(xor.read_pin(layout.output_pins[0]), 0);
}

#[test]
fn given_xor_when_not_yet_ticked_then_all_pin_states_low() {
    let (circuit, _, _, _) = build_xor();
    let pin_states = circuit.get_chip_pin_states();
    let all_low = pin_states.iter().all(|(_, value)| *value == 0);
    assert!(all_low);
}

#[test]
fn given_xor_when_supply_off_then_all_pin_states_low() {
    let (mut circuit, _, _, _) = build_xor();
    circuit.tick();
    let pin_states = circuit.get_chip_pin_states();
    let all_low = pin_states.iter().all(|(_, value)| *value == 0);
    assert!(all_low);
}

#[test]
fn given_xor_when_supply_on_then_pin_states_correct() {
    let (mut circuit, _, _, _) = build_xor();
    let supply_id = circuit.get_supply_ids()[0];
    circuit.set_supply(supply_id, 1);
    circuit.tick();
    let states = circuit.get_chip_pin_states();

    // Ground and Supply
    assert_eq!(*states.get(&chip_pin!(0, 0)).unwrap(), 0);
    assert_eq!(*states.get(&chip_pin!(1, 0)).unwrap(), 1);

    // Inputs
    assert_eq!(*states.get(&chip_pin!(2, 0)).unwrap(), 0);
    assert_eq!(*states.get(&chip_pin!(3, 0)).unwrap(), 0);

    // NAnd 1
    assert_eq!(*states.get(&chip_pin!(4, 0)).unwrap(), 0);
    assert_eq!(*states.get(&chip_pin!(4, 1)).unwrap(), 1);
    assert_eq!(*states.get(&chip_pin!(4, 2)).unwrap(), 0);
    assert_eq!(*states.get(&chip_pin!(4, 3)).unwrap(), 0);
    assert_eq!(*states.get(&chip_pin!(4, 4)).unwrap(), 1);

    // NAnd 2
    assert_eq!(*states.get(&chip_pin!(5, 0)).unwrap(), 0);
    assert_eq!(*states.get(&chip_pin!(5, 1)).unwrap(), 1);
    assert_eq!(*states.get(&chip_pin!(5, 2)).unwrap(), 0);
    assert_eq!(*states.get(&chip_pin!(5, 3)).unwrap(), 1);
    assert_eq!(*states.get(&chip_pin!(5, 4)).unwrap(), 1);

    // NAnd 3
    assert_eq!(*states.get(&chip_pin!(6, 0)).unwrap(), 0);
    assert_eq!(*states.get(&chip_pin!(6, 1)).unwrap(), 1);
    assert_eq!(*states.get(&chip_pin!(6, 2)).unwrap(), 0);
    assert_eq!(*states.get(&chip_pin!(6, 3)).unwrap(), 1);
    assert_eq!(*states.get(&chip_pin!(6, 4)).unwrap(), 1);

    // NAnd 4
    assert_eq!(*states.get(&chip_pin!(7, 0)).unwrap(), 0);
    assert_eq!(*states.get(&chip_pin!(7, 1)).unwrap(), 1);
    assert_eq!(*states.get(&chip_pin!(7, 2)).unwrap(), 1);
    assert_eq!(*states.get(&chip_pin!(7, 3)).unwrap(), 1);
    assert_eq!(*states.get(&chip_pin!(7, 4)).unwrap(), 0);

    // Output
    assert_eq!(*states.get(&chip_pin!(8, 0)).unwrap(), 0);
}
