use crate::chip::NAND;

#[test]
fn given_new_then_inputs_are_0() {
    let nand = NAND::new();
    assert_eq!(nand.input_a, 0);
    assert_eq!(nand.input_b, 0);
}

#[test]
fn given_new_then_vcc_is_0() {
    let nand = NAND::new();
    assert_eq!(nand.vcc, 0);
}

#[test]
fn given_vcc_0_then_output_is_0() {
    let mut nand = NAND::new();
    nand.set_inputs(0, 0);
    assert_eq!(nand.output(), 0);
    nand.set_inputs(0, 1);
    assert_eq!(nand.output(), 0);
    nand.set_inputs(1, 0);
    assert_eq!(nand.output(), 0);
    nand.set_inputs(1, 1);
    assert_eq!(nand.output(), 0);
}

#[test]
fn given_inputs_both_0_then_output_is_1() {
    let mut nand = NAND::new();
    nand.set_vcc(1);
    nand.set_inputs(0, 0);
    assert_eq!(nand.output(), 1);
}

#[test]
fn given_inputs_both_1_then_output_is_0() {
    let mut nand = NAND::new();
    nand.set_vcc(1);
    nand.set_inputs(1, 1);
    assert_eq!(nand.output(), 0);
}

#[test]
fn given_single_input_1_then_output_is_1() {
    let mut nand = NAND::new();
    nand.set_vcc(1);
    nand.set_inputs(1, 0);
    assert_eq!(nand.output(), 1);
    nand.set_inputs(0, 1);
    assert_eq!(nand.output(), 1);
}

#[test]
fn given_gnd_1_then_output_is_0() {
    let mut nand = NAND::new();
    nand.set_vcc(1);
    nand.set_gnd(1);
    nand.set_inputs(0, 0);
    assert_eq!(nand.output(), 0);
}