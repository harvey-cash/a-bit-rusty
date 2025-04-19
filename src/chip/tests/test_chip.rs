use crate::chip::{Chip, Input, Link, Nand, Output};

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
fn given_one_link_when_updated_then_output_equals_input() {
    let inputs = [Input::new()];
    let outputs = [Output::new()];
    let nands = [Nand::new()];
    let links = [Link::new(0, 1)];
    let mut component = Chip::new(inputs, outputs, nands, links);
    component.update_input(0, 1);
    assert_eq!(component.get_output(0, 1), 1);
}

#[test]
fn given_one_link_when_updated_with_zero_then_output_is_zero() {
    let inputs = [Input::new()];
    let outputs = [Output::new()];
    let nands = [Nand::new()];
    let links = [Link::new(0, 0)];
    let mut component = Chip::new(inputs, outputs, nands, links);
    component.update_input(0, 0);
    assert_eq!(component.get_output(0, 1), 0);
}
