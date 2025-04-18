use crate::component::{Component, Input, Output, Link };

// A component is directed dependency graph of sub components, which may contain cycles.
// The fundamental components are input, output, and NAND gate.
// Inputs and outputs are always leaves in the graph.
// A component must have at least one input and at least one output.
// All nodes must be connected in the graph.
// To update a component, the inputs are set and subcomponents connected to the inputs are updated.
// A component can only be updated once per tick.
// When update is called on a subcomponent that has already been updated in the current tick, the update is ignored.
// Thus in any one tick the graph is acyclic and updates propagate in a topological order.
// Input states do not reset on tick, allowing for the possibility of feedback loops in the graph between ticks.
// Ticks are called on all components in the graph simultaneously.

// Thus tick frequency should be signficantly faster than the input update frequency.
// A components is the compiled version of a circuit (except for fundamental components).

#[test]
fn given_one_link_when_updated_then_output_equals_input() {
    let inputs = [ Input::new() ];
    let outputs = [ Output::new() ];
    let links = [ Link::new(0, 1) ];
    let mut component = Component::new(inputs, outputs, links);
    component.update_input(0, 1);
    assert_eq!(component.get_output(0, 1), 1);
}

#[test]
fn given_one_link_when_updated_with_zero_then_output_is_zero() {
    let inputs = [ Input::new() ];
    let outputs = [ Output::new() ];
    let links = [ Link::new(0, 0) ];
    let mut component = Component::new(inputs, outputs, links);
    component.update_input(0, 0);
    assert_eq!(component.get_output(0, 1), 0);
}