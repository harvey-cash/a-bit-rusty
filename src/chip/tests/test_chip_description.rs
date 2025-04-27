use crate::chip::{chip_description::ChipDescription, types::*};

// ToDo:
// [X] If a ChipDescription is valid, it can be used to create a new Chip.
// [X] ChipDescriptions are invalid if multiple sources have the same target.
// [X] ChipDescriptions are invalid if any Chip is unconnected.
// [ ] ChipDescriptions define the XY size of a new Chip.
// [ ] ChipDescriptions define the co-ordinates of Pins adjacent the the size bounds.
// [ ] ChipDescriptions are invalid if any Chip's Ground Pin is not connected to a Ground Chip.
// [ ] ChipDescriptions are invalid if any Chip's Supply Pin is not connected to a Supply Chip.
// [ ] ChipDescriptions are invalid if there is not >=1 Ground, >=1 Supply, >=1 Input and >=1 Output.

#[test]
fn given_no_inputs_then_invalid() {
    let description = ChipDescription::new(0, 1, 0, vec![Link::new(2, 2)]);
    assert_eq!(description.is_valid(), false);
}

#[test]
fn given_no_outputs_then_invalid() {
    let description = ChipDescription::new(1, 0, 0, vec![Link::new(2, 2)]);
	assert_eq!(description.is_valid(), false);
}

#[test]
fn given_no_links_then_invalid() {
    let description = ChipDescription::new(1, 0, 1, vec![]);
	assert_eq!(description.is_valid(), false);
}

#[test]
fn given_no_nands_then_still_valid() {
    let description = ChipDescription::new(1, 0, 1, vec![Link::new(2, 3)]);
	assert_eq!(description.is_valid(), true);
}

#[test]
fn given_single_wired_up_nand_then_valid() {
    let description = ChipDescription::new(2, 1, 1, vec![Link::new(2, 4), Link::new(3, 4), Link::new(4, 5)]);
    assert_eq!(description.is_valid(), true);
}

#[test]
fn given_link_source_out_of_range_then_invalid() {
    let description = ChipDescription::new(1, 0, 1, vec![Link::new(7, 2)]);
    assert_eq!(description.is_valid(), false);

    let links = vec![Link::new(2, 2), Link::new(7, 2)];
    let description = ChipDescription::new(1, 0, 1, links);
    assert_eq!(description.is_valid(), false);
}

#[test]
fn given_link_targets_input_then_invalid() {
    let description = ChipDescription::new(
        2,
        0,
        2,
        vec![Link::new(2, 3), Link::new(2, 4), Link::new(3, 5)],
    );
    assert_eq!(description.is_valid(), false);
}

#[test]
fn given_link_sources_output_then_invalid() {
    let description = ChipDescription::new(
        1,
        0,
        2,
        vec![Link::new(2, 3), Link::new(2, 4), Link::new(3, 4)],
    );
    assert_eq!(description.is_valid(), false);
}

#[test]
fn given_output_targeted_by_two_links_then_invalid() {
    let links = vec![Link::new(2, 4), Link::new(3, 4)];
    let description = ChipDescription::new(2, 0, 1, links);
	assert_eq!(description.is_valid(), false);
}

#[test]
fn given_any_node_unconnected_then_invalid() {
    let description = ChipDescription::new(1, 0, 2, vec![Link::new(2, 3)]);
    assert_eq!(description.is_valid(), false);

    let description = ChipDescription::new(
        1,
        1,
        1,
        vec![Link::new(2, 4), Link::new(3, 3), Link::new(3, 3)],
    );
    assert_eq!(description.is_valid(), false);
}

#[test]
fn given_nand_with_no_targets_then_invalid() {
    let links = vec![Link::new(2, 3), Link::new(2, 3), Link::new(2, 4)];
    let description = ChipDescription::new(1, 1, 1, links);
	assert_eq!(description.is_valid(), false);
}

#[test]
fn given_nand_no_sources_then_invalid() {
    let description = ChipDescription::new(1, 1, 2, vec![Link::new(2, 4), Link::new(3, 5)]);
	assert_eq!(description.is_valid(), false);
}

#[test]
fn given_nand_three_sources_then_invalid() {
    let links = vec![
        Link::new(2, 5),
        Link::new(3, 5),
        Link::new(4, 5),
        Link::new(5, 6),
    ];
    let description = ChipDescription::new(3, 1, 1, links);
	assert_eq!(description.is_valid(), false);
}

#[test]
fn given_nand_same_source_three_times_then_invalid() {
    let links = vec![
        Link::new(2, 3),
        Link::new(2, 3),
        Link::new(2, 3),
        Link::new(3, 4),
    ];
    let description = ChipDescription::new(1, 1, 1, links);
	assert_eq!(description.is_valid(), false);
}