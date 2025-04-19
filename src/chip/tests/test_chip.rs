use crate::chip::{Chip, Input, Link, NAND, Output };

// A Chip is a dependency graph of Nodes connected by directed links.
// The Node types are Input, Output, and NAND.
// Inputs and Outputs are leaves in the graph.
// Inputs have one out pin which is always the source of exactly one link.
// Outputs have one in pin which is always the target of exactly one link.
// Inputs have a state which can be set. Outputs have a state which can be read.
// NAND nodes have in pins for A, B, GND, and VCC.
// NAND nodes GND and VCC pins must each be the target of one link. 
// NAND nodes A and B pins may be the target of either zero or one links.
// NAND nodes have one out pin Y. This must be the source of one link.
// A Chip must have at least one Input node and at least one Output node. It may have 0 NANDs.
// Chips and Nodes can be updated with source values.
// When an Input is updated, it calls update on its target with its own state.
// When an Output is updated, it stores the provided state.
// When a NAND is updated, it reads the state of its source links and calls update on its target.
// When a Chip is updated, it sets the value of all Inputs and calls update on them.
// The graph is connected and may contain cycles.
// Chips and Nodes can only be updated once per tick. Subsequent updates are ignored.
// In any one tick the graph's update is acyclic and updates propagate in a topological order.
// Ticks are called on all Nodes in the graph simultaneously.

// Input values do not reset on tick, allowing for the possibility of feedback loops in the graph between ticks.
// Thus tick frequency should be significantly faster than the update frequency.

// A Chip is the compiled form of a Circuit.

#[test]
fn given_one_link_when_updated_then_output_equals_input() {
    let inputs = [ Input::new() ];
    let outputs = [ Output::new() ];
    let links = [ Link::new(0, 1) ];
    let mut component = Chip::new(inputs, outputs, links);
    component.update_input(0, 1);
    assert_eq!(component.get_output(0, 1), 1);
}

#[test]
fn given_one_link_when_updated_with_zero_then_output_is_zero() {
    let inputs = [ Input::new() ];
    let outputs = [ Output::new() ];
    let links = [ Link::new(0, 0) ];
    let mut component = Chip::new(inputs, outputs, links);
    component.update_input(0, 0);
    assert_eq!(component.get_output(0, 1), 0);
}