use crate::chip::{Chip, Link};

// A Circuit comprises Inputs, Outputs, and Chips connected by traces.
// A Circuit may only be compiled to a Chip if all its Chips have GND and VCC connected.

// A Chip has GND, VCC, at least one Input and at least one Output.
// If GND != 0 and/or VCC != 1, all of a Chip's Outputs will be 0.
// A Chip is a dependency graph of Nodes connected by directed links.
// The Node types are Input, Output, and NAND.
// Inputs and Outputs are leaves in the graph.
// Inputs have one out pin which is always the source of exactly one link.
// Outputs have one in pin which is always the target of exactly one link.
// Inputs have a state which can be set. Outputs have a state which can be read.
// NAND nodes have in pins for A and B which each may be the target of either zero or one links.
// NAND nodes have one out pin Y. This must be the source of one link.
// A Chip must have at least one Input node and at least one Output node. It may have 0 NANDs.
// When an Node is updated with source values, it calls update on its targets.
// When a Chip is updated, it sets the value of all Inputs and calls update on them.
// The graph is connected and may contain cycles.
// Chips and Nodes can only be updated once per tick. Subsequent updates are ignored.
// In any one tick the graph's update is acyclic and updates propagate in a topological order.
// Ticks are called on all Nodes in the graph simultaneously.

// Input values do not reset on tick, allowing for the possibility of feedback loops in the graph between ticks.
// Thus tick frequency should be significantly faster than the update frequency.


#[test]
#[should_panic]
fn given_no_inputs_then_panics() {
    Chip::new(0, 1, 0, vec![Link::new(0, 0)]);
}

#[test]
#[should_panic]
fn given_no_outputs_then_panics() {
    Chip::new(1, 0, 0, vec![Link::new(0, 0)]);
}

#[test]
#[should_panic]
fn given_no_links_then_panics() {
    Chip::new(1, 1, 0, vec![]);
}

#[test]
fn given_no_nands_then_does_not_panic() {
    Chip::new(1, 1, 0, vec![Link::new(0, 0)]);
}

#[test]
fn given_link_source_out_of_range_then_panics() {
    let f = || -> Chip { Chip::new(1, 1, 0, vec![Link::new(5, 0)]) };
    let result = std::panic::catch_unwind(f);
    assert!(result.is_err());

    let links = vec![Link::new(0, 0), Link::new(5, 0)];
    let f = || -> Chip { Chip::new(1, 1, 0, links) };
    let result = std::panic::catch_unwind(f);
    assert!(result.is_err());
}

#[test]
fn given_link_source_in_range_then_does_not_panic() {
    Chip::new(2, 1, 0, vec![Link::new(1, 0)]);
}

#[test]
fn given_link_target_in_range_then_does_not_panic() {
    Chip::new(2, 1, 0, vec![Link::new(0, 2)]);
}

// ToDo: given_link_source_and_target_are_equal_then_panics?
// ToDo: given_link_targets_input_then_panics
// ToDo: given_link_sources_output_then_panics

#[test]
fn given_one_link_then_output_equals_input() {
    let mut chip = Chip::new(1, 1, 0, vec![Link::new(0, 1)]);

    chip.set_input(0, 1);
    chip.update();
    assert_eq!(chip.get_output(0), 1);

    chip.set_input(0, 0);
    chip.update();
    assert_eq!(chip.get_output(0), 0);
}

#[test]
fn given_one_link_then_output_not_set_before_update() {
    let mut chip = Chip::new(1, 1, 0, vec![Link::new(0, 1)]);
    chip.set_input(0, 1);
    assert_eq!(chip.get_output(0), 0);
}

#[test]
fn given_two_separate_links_then_outputs_equal_corresponding_inputs() {
    let links = vec![Link::new(0, 2), Link::new(1, 3)];
    let mut chip = Chip::new(2, 2, 0, links);

    chip.set_input(0, 0);
    chip.set_input(1, 1);
    chip.update();
    assert_eq!(chip.get_output(0), 0);
    assert_eq!(chip.get_output(1), 1);
}

#[test]
fn given_two_crossed_links_then_outputs_equal_corresponding_inputs() {
    let links = vec![Link::new(0, 3), Link::new(1, 2)];
    let mut chip = Chip::new(2, 2, 0, links);

    chip.set_input(0, 0);
    chip.set_input(1, 1);
    chip.update();
    assert_eq!(chip.get_output(0), 1);
    assert_eq!(chip.get_output(1), 0);
}

// #[test]
// fn given_nand_when_inputs_both_0_then_output_is_1() {
//     let links = vec![Link::new(0, 2), Link::new(1, 2)];
//     let mut chip = Chip::new(2, 1, 1, links);
//     chip.set_input(0, 0);
//     chip.set_input(1, 0);
//     chip.update();
//     assert_eq!(chip.get_output(0), 1);
// }

// #[test]
// fn given_nand_when_inputs_both_1_then_output_is_0() {
//     let mut nand = NANDChip::new();
//     nand.set_vcc(1);
//     nand.set_inputs(1, 1);
//     assert_eq!(nand.output(), 0);
// }

// #[test]
// fn given_nand_when_single_input_1_then_output_is_1() {
//     let mut nand = NANDChip::new();
//     nand.set_vcc(1);
//     nand.set_inputs(1, 0);
//     assert_eq!(nand.output(), 1);
//     nand.set_inputs(0, 1);
//     assert_eq!(nand.output(), 1);
// }
