use crate::{chip::{chip_description::ChipDescription, types::*}, node_type_map};

// ToDo:
// [ ] ChipDescriptions define the XY size of a new Chip.
// [ ] ChipDescriptions define the co-ordinates of Pins adjacent the the size bounds.
// [ ] ChipDescriptions are invalid if any Chip's Ground Pin is not connected to a Ground Chip.
// [ ] ChipDescriptions are invalid if any Chip's Supply Pin is not connected to a Supply Chip.
// [ ] ChipDescriptions are invalid if there is not >=1 Ground, >=1 Supply, >=1 Input and >=1 Output.

#[test]
fn given_no_inputs_then_invalid() {
    let id_types = node_type_map!{2 => NodeType::Output};
    let description = ChipDescription::new(id_types, vec![Link::new(2, 2)]);
    assert_eq!(description.is_valid(), false);
}

#[test]
fn given_no_outputs_then_invalid() {
    let id_types = node_type_map!{2 => NodeType::Input};
	let description = ChipDescription::new(id_types, vec![Link::new(2, 2)]);
	assert_eq!(description.is_valid(), false);
}

#[test]
fn given_no_links_then_invalid() {
    let id_types = node_type_map!{2 => NodeType::Input, 3 => NodeType::Output};
	let description = ChipDescription::new(id_types, vec![]);
	assert_eq!(description.is_valid(), false);
}

#[test]
fn given_no_nands_then_still_valid() {
    let id_types = node_type_map!{2 => NodeType::Input, 3 => NodeType::Output};
	let description = ChipDescription::new(id_types, vec![Link::new(2, 3)]);
	assert_eq!(description.is_valid(), true);
}

#[test]
fn given_single_wired_up_nand_then_valid() {
    let (input_1, input_2, output, nand) = (2, 3, 4, 5);
    let id_types = node_type_map!{
        input_1 => NodeType::Input,
        input_2 => NodeType::Input,
        output => NodeType::Output,
        nand => NodeType::NAnd
    };
    let description = ChipDescription::new(
        id_types, vec![Link::new(input_1, nand), Link::new(input_2, nand), Link::new(nand, output)]
    );
    assert_eq!(description.is_valid(), true);
}

#[test]
fn given_link_source_out_of_range_then_invalid() {
    let id_types = node_type_map!{2 => NodeType::Input, 3 => NodeType::Output};
	let description = ChipDescription::new(id_types, vec![Link::new(7, 2)]);
    assert_eq!(description.is_valid(), false);

    let links = vec![Link::new(2, 2), Link::new(7, 2)];
    let id_types = node_type_map!{};
	let description = ChipDescription::new(id_types, links);
    assert_eq!(description.is_valid(), false);
}

#[test]
fn given_link_targets_input_then_invalid() {
    let (input_1, input_2, output) = (2, 3, 4);
    let id_types = node_type_map!{
        input_1 => NodeType::Input,
        input_2 => NodeType::Input,
        output => NodeType::Output
    };
    let description = ChipDescription::new(
        id_types, 
        vec![Link::new(input_1, input_2), Link::new(input_2, output)],
    );
    assert_eq!(description.is_valid(), false);
}

#[test]
fn given_link_sources_output_then_invalid() {
    let (input_1, output_1, output_2) = (2, 3, 4);
    let id_types = node_type_map!{
        input_1 => NodeType::Input,
        output_1 => NodeType::Output,
        output_2 => NodeType::Output
    };
    let description = ChipDescription::new(
        id_types, 
        vec![Link::new(input_1, output_1), Link::new(input_1, output_2), Link::new(output_1, output_2)],
    );
    assert_eq!(description.is_valid(), false);
}

#[test]
fn given_output_targeted_by_two_links_then_invalid() {
    let (input_1, input_2, output) = (2, 3, 4);
    let links = vec![Link::new(input_1, output), Link::new(input_2, output)];
    let id_types = node_type_map!{
        input_1 => NodeType::Input,
        input_2 => NodeType::Input,
        output => NodeType::Output
    };
	let description = ChipDescription::new(id_types, links);
	assert_eq!(description.is_valid(), false);
}

#[test]
fn given_any_node_unconnected_then_invalid() {
    let (input, output, nand) = (2, 3, 4);
    let id_types = node_type_map!{
        input => NodeType::Input,
        output => NodeType::Output,
        nand => NodeType::NAnd,
    };
	let description = ChipDescription::new(id_types.clone(), vec![Link::new(input, nand)]);
    assert_eq!(description.is_valid(), false);

    let description = ChipDescription::new(
        id_types.clone(), 
        vec![Link::new(input, output), Link::new(nand, nand), Link::new(nand, nand)],
    );
    assert_eq!(description.is_valid(), false);
}

#[test]
fn given_nand_with_no_targets_then_invalid() {
    let (input, output, nand) = (2, 3, 4);
    let links = vec![Link::new(input, nand), Link::new(input, nand), Link::new(input, output)];
    let id_types = node_type_map!{
        input => NodeType::Input,
        output => NodeType::Output,
        nand => NodeType::NAnd,
    };
	let description = ChipDescription::new(id_types, links);
	assert_eq!(description.is_valid(), false);
}

#[test]
fn given_nand_no_sources_then_invalid() {
    let (input, output_1, output_2, nand) = (2, 3, 4, 5);
    let id_types = node_type_map!{
        input => NodeType::Input,
        output_1 => NodeType::Output,
        output_2 => NodeType::Output,
        nand => NodeType::NAnd,
    };
	let description = ChipDescription::new(id_types, vec![Link::new(input, output_1), Link::new(nand, output_2)]);
	assert_eq!(description.is_valid(), false);
}

#[test]
fn given_nand_three_sources_then_invalid() {
    let (input_1, input_2, input_3, output, nand) = (2, 3, 4, 5, 6);
    let links = vec![
        Link::new(input_1, nand),
        Link::new(input_2, nand),
        Link::new(input_3, nand),
        Link::new(nand, output),
    ];
    let id_types = node_type_map!{
        input_1 => NodeType::Input,
        input_2 => NodeType::Input,
        input_3 => NodeType::Input,
        output => NodeType::Output,
        nand => NodeType::NAnd,
    };
	let description = ChipDescription::new(id_types, links);
	assert_eq!(description.is_valid(), false);
}

#[test]
fn given_nand_same_source_three_times_then_invalid() {
    let (input, output, nand) = (2, 3, 4);
    let links = vec![
        Link::new(input, nand),
        Link::new(input, nand),
        Link::new(input, nand),
        Link::new(nand, output),
    ];
    let id_types = node_type_map!{
        input => NodeType::Input,
        output => NodeType::Output,
        nand => NodeType::NAnd,
    };
	let description = ChipDescription::new(id_types, links);
	assert_eq!(description.is_valid(), false);
}