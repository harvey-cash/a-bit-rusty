use crate::chip::{Chip, Link};

// A Circuit comprises Inputs, Outputs, and Chips connected by traces.
// A Circuit may only be compiled to a Chip if all its Chips have GND and VCC connected.

// A Chip has GND, VCC, at least 1 Input and at least 1 Output.
// If GND != 0 and/or VCC != 1, all of a Chip's Outputs will be 0.
// A Chip is a dependency graph of Nodes connected by directed Links.
// The Node types are Input, Output, and NAnd.
// Nodes have a state value.
// Input values can be set on the Chip. Output values can be read from the Chip.
// Inputs and Outputs are leaves in the graph.
// Inputs are the source of >= 1 Link.
// Inputs can not be a target.
// Outputs are the target of 1 Link only.
// Outputs can not be a source.
// NAnd Nodes must be the target of 2 Links.
// NAnd Nodes must be the source of 1 Link.
// A Chip must have at least 1 Input node and at least 1 Output node. It may have 0 NANDs.
// When an Input is updated, it does nothing.
// When an Output is updated, it copies the value of its source to itself.
// When a NAnd is updated, it reads the values of its sources and writes the result of their logical NAnd to itself.
// When a Chip is updated, it calls update throughout its directed graph in topological order.
// The graph is connected and may contain cycles.
// Chips and Nodes can only be updated once per tick. Subsequent updates are ignored.
// In any 1 tick the graph's update is acyclic.
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
    Chip::new(1, 0, 1, vec![]);
}

#[test]
fn given_no_nands_then_does_not_panic() {
    Chip::new(1, 0, 1, vec![Link::new(0, 1)]);
}

#[test]
fn given_link_source_out_of_range_then_panics() {
    let f = || -> Chip { Chip::new(1, 0, 1, vec![Link::new(5, 0)]) };
    let result = std::panic::catch_unwind(f);
    assert!(result.is_err());

    let links = vec![Link::new(0, 0), Link::new(5, 0)];
    let f = || -> Chip { Chip::new(1, 0, 1, links) };
    let result = std::panic::catch_unwind(f);
    assert!(result.is_err());
}

#[test]
#[should_panic]
fn given_link_targets_input_then_panics() {
    Chip::new(2, 0, 2, vec![Link::new(0, 1), Link::new(0, 2), Link::new(1, 3)]);
}

#[test]
#[should_panic]
fn given_link_sources_output_then_panics() {
    Chip::new(1, 0, 2, vec![Link::new(0, 1), Link::new(0, 2), Link::new(1, 2)]);
}

#[test]
#[should_panic]
fn given_any_node_unconnected_then_panics() {
    Chip::new(1, 0, 2, vec![Link::new(0, 1)]);
}

#[test]
fn given_one_link_then_output_equals_input() {
    let mut chip = Chip::new(1, 0, 1, vec![Link::new(0, 1)]);

    chip.set_input(0, 1);
    chip.update();
    assert_eq!(chip.get_output(0), 1);

    chip.set_input(0, 0);
    chip.update();
    assert_eq!(chip.get_output(0), 0);
}

#[test]
fn given_one_link_then_output_not_set_before_update() {
    let mut chip = Chip::new(1, 0, 1, vec![Link::new(0, 1)]);
    chip.set_input(0, 1);
    assert_eq!(chip.get_output(0), 0);
}

#[test]
fn given_two_separate_links_then_outputs_equal_corresponding_inputs() {
    let links = vec![Link::new(0, 2), Link::new(1, 3)];
    let mut chip = Chip::new(2, 0, 2, links);

    chip.set_input(0, 0);
    chip.set_input(1, 1);
    chip.update();
    assert_eq!(chip.get_output(0), 0);
    assert_eq!(chip.get_output(1), 1);
}

#[test]
fn given_two_crossed_links_then_outputs_equal_corresponding_inputs() {
    let links = vec![Link::new(0, 3), Link::new(1, 2)];
    let mut chip = Chip::new(2, 0, 2, links);

    chip.set_input(0, 0);
    chip.set_input(1, 1);
    chip.update();
    assert_eq!(chip.get_output(0), 1);
    assert_eq!(chip.get_output(1), 0);
}

#[test]
fn given_nand_when_inputs_both_0_then_output_is_1() {
    let links = vec![Link::new(0, 2), Link::new(1, 2), Link::new(2, 3)];
    let mut chip = Chip::new(2, 1, 1, links);
    chip.set_input(0, 0);
    chip.set_input(1, 0);
    chip.update();
    assert_eq!(chip.get_output(0), 1);
}

#[test]
fn given_nand_when_inputs_both_1_then_output_is_0() {
    let links = vec![Link::new(0, 2), Link::new(1, 2), Link::new(2, 3)];
    let mut chip = Chip::new(2, 1, 1, links);
    chip.set_input(0, 1);
    chip.set_input(1, 1);
    chip.update();
    assert_eq!(chip.get_output(0), 0);
}

#[test]
fn given_nand_when_single_input_1_then_output_is_1() {
    let links = vec![Link::new(0, 2), Link::new(1, 2), Link::new(2, 3)];
    let mut chip = Chip::new(2, 1, 1, links);
    chip.set_input(0, 1);
    chip.set_input(1, 0);
    chip.update();
    assert_eq!(chip.get_output(0), 1);

    chip.set_input(0, 0);
    chip.set_input(1, 1);
    chip.update();
    assert_eq!(chip.get_output(0), 1);
}

#[test]
fn given_nand_linked_inputs_then_output_is_not_input() {
    let links = vec![Link::new(0, 1), Link::new(0, 1), Link::new(1, 2)];
    let mut chip = Chip::new(1, 1, 1, links);
    chip.set_input(0, 0);
    chip.update();
    assert_eq!(chip.get_output(0), 1);
    chip.set_input(0, 1);
    chip.update();
    assert_eq!(chip.get_output(0), 0);
}

#[test]
#[should_panic]
fn given_nand_no_sources_then_panics() {
    Chip::new(1, 1, 2, vec![Link::new(0, 2), Link::new(1, 3)]);
}

#[test]
#[should_panic]
fn given_nand_three_sources_then_panics() {
    let links = vec![
        Link::new(0, 3),
        Link::new(1, 3),
        Link::new(2, 3),
        Link::new(3, 4),
    ];
    Chip::new(3, 1, 1, links);
}

#[test]
#[should_panic]
fn given_nand_same_source_three_times_then_panics() {
    let links = vec![
        Link::new(0, 1),
        Link::new(0, 1),
        Link::new(0, 1),
        Link::new(1, 2),
    ];
    Chip::new(1, 1, 1, links);
}

#[test]
fn given_two_nots_in_series_then_output_equals_input() {
    let links = vec![
        Link::new(0, 2),
        Link::new(0, 2),
        Link::new(2, 1),
        Link::new(2, 1),
        Link::new(1, 3),
    ];
    let mut chip = Chip::new(1, 2, 1, links);

    chip.set_input(0, 1);
    chip.update();
    assert_eq!(chip.get_output(0), 1);

    chip.set_input(0, 0);
    chip.update();
    assert_eq!(chip.get_output(0), 0);
}
