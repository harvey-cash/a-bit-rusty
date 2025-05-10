
// Starts with an empty circuit
// Can add fundamental chips
// Can add links between pins
// Can compile circuit to chip
// Can create new circuit
// Can add new chip to circuits going forwards
// Can read pin states and link states

use crate::{chip::{types::*, chip::ChipType, chip_database::ChipKey, designer::Designer}, chip_pin};

#[test]
fn given_add_nand_chip_then_succeeds() {
    let mut designer = Designer::new();
    let key = ChipKey::Custom("NAnd".to_string());
    let result = designer.add_chip(key);
    assert!(result.is_ok())
}

#[test]
fn given_no_chips_in_db_when_add_custom_chip_then_err() {
    let mut designer = Designer::new();
    let key = ChipKey::Custom("Test".to_string());
    let result = designer.add_chip(key);
    assert!(result.is_err())
}

#[test]
fn given_single_input_when_linked_to_self_then_err() {
    let mut designer = Designer::new();
    let input = designer.add_chip(ChipKey::Basic(ChipType::Input)).unwrap();
    let result = designer.add_link(chip_pin!(input, 0), chip_pin!(input, 0));
    assert!(result.is_err());
}

#[test]
fn given_single_io_when_linked_then_succeeds() {
    let mut designer = Designer::new();
    let input = designer.add_chip(ChipKey::Basic(ChipType::Input)).unwrap();
    let output = designer.add_chip(ChipKey::Basic(ChipType::Output)).unwrap();
    let result = designer.add_link(chip_pin!(input, 0), chip_pin!(output, 0));
    assert!(result.is_ok());
}