use crate::component::Component;

// A component is directed dependency graph of sub components, which may contain cycles.
// The fundamental components are input, output, and NAND gate.
// Inputs and outputs are always leaves in the graph.
// All nodes must be connected in the graph.
// To update a component, the inputs are set and subcomponents connected to the inputs are updated.
// A component can only be updated once per tick.
// When update is called on a subcomponent that has already been updated in the current tick, the update is ignored.
// Thus in any one tick the graph is acyclic and updates propagate in a topological order.
// Input states do not reset on tick, allowing for the possibility of feedback loops in the graph between ticks.
// Ticks are called on all components in the graph simultaneously.
// Thus tick frequency should be signficantly faster than the input update frequency.

#[test]
fn given_no_nodes_then_err() {
    let component = Component::new(vec![]);
    assert!(component.is_err());
}