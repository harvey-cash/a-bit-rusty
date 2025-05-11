
// Starts with an empty circuit
// Can add fundamental chips
// Can compile circuit to chip
// Can create new circuit
// Can add new chip to circuits going forwards
// Can read pin states and link states

use crate::{chip::{chip::ChipType, designer::{ChipPinLink, Designer}, types::*}, chip_pin};

#[test]
fn given_add_nand_chip_then_succeeds() {
    let mut designer = Designer::new();
    let result = designer.add_chip("NAnd");
    assert!(result.is_ok())
}

#[test]
fn given_no_chips_in_db_when_add_custom_chip_then_err() {
    let mut designer = Designer::new();
    let result = designer.add_chip("Test");
    assert!(result.is_err())
}

#[test]
fn given_nothing_loaded_then_state_has_no_chips() {
    let designer = Designer::new();
    let state = designer.get_state();
    assert_eq!(state.chip_pin_states.len(), 0);
}

#[test]
fn given_single_input_when_linked_to_self_then_err() {
    let mut designer = Designer::new();
    let input = designer.add_chip(&ChipType::Input.to_string()).unwrap();
    let result = designer.add_link(ChipPinLink { 
        source: chip_pin!(input, 0), 
        target: chip_pin!(input, 0) 
    });
    assert!(result.is_err());
}

#[test]
fn given_single_io_when_linked_then_succeeds() {
    let mut designer = Designer::new();
    let input = designer.add_chip(&ChipType::Input.to_string()).unwrap();
    let output = designer.add_chip(&ChipType::Output.to_string()).unwrap();
    let result = designer.add_link(ChipPinLink { 
        source: chip_pin!(input, 0), 
        target: chip_pin!(output, 0)
    });
    assert!(result.is_ok());
}

#[test]
fn given_single_io_when_input_1_then_output_1() {
    let mut designer = Designer::new();
    let input = designer.add_chip(&ChipType::Input.to_string()).unwrap();
    let output = designer.add_chip(&ChipType::Output.to_string()).unwrap();
    let _ = designer.add_link(ChipPinLink { 
        source: chip_pin!(input, 0), 
        target: chip_pin!(output, 0) 
    });
    let _ = designer.set_input_chip_value(input, 1);
    designer.tick();
    let state = designer.get_state();
    let output_state = state.chip_pin_states.get(&output).unwrap().get(&0).unwrap();
    assert_eq!(*output_state, 1);
}

#[test]
fn given_single_io_when_link_deleted_then_output_0() {
    let mut designer = Designer::new();
    let input = designer.add_chip(&ChipType::Input.to_string()).unwrap();
    let output = designer.add_chip(&ChipType::Output.to_string()).unwrap();
    let _ = designer.add_link(ChipPinLink { 
        source: chip_pin!(input, 0), 
        target: chip_pin!(output, 0) 
    });
    let _ = designer.set_input_chip_value(input, 1);
    let _ = designer.delete_link(chip_pin!(input, 0), chip_pin!(output, 0));
    designer.tick();
    let state = designer.get_state();
    let output_state = state.chip_pin_states.get(&output).unwrap().get(&0).unwrap();
    assert_eq!(*output_state, 0);
}

#[test]
fn given_single_io_when_input_0_then_output_0() {
    let mut designer = Designer::new();
    let input = designer.add_chip(&ChipType::Input.to_string()).unwrap();
    let output = designer.add_chip(&ChipType::Output.to_string()).unwrap();
    let _ = designer.add_link(ChipPinLink { 
        source: chip_pin!(input, 0), 
        target: chip_pin!(output, 0) 
    });
    let _ = designer.set_input_chip_value(input, 0);
    designer.tick();
    let state = designer.get_state();
    let output_state = state.chip_pin_states.get(&output).unwrap().get(&0).unwrap();
    assert_eq!(*output_state, 0);
}