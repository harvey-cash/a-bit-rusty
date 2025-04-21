use crate::chip::chip_description::{ChipDescription, Link};


#[test]
#[should_panic]
fn given_no_inputs_then_panics() {
    ChipDescription::new(0, 1, 0, vec![Link::new(0, 0)]);
}

#[test]
#[should_panic]
fn given_no_outputs_then_panics() {
    ChipDescription::new(1, 0, 0, vec![Link::new(0, 0)]);
}

#[test]
#[should_panic]
fn given_no_links_then_panics() {
    ChipDescription::new(1, 0, 1, vec![]);
}

#[test]
fn given_no_nands_then_does_not_panic() {
    ChipDescription::new(1, 0, 1, vec![Link::new(0, 1)]);
}

#[test]
fn given_link_source_out_of_range_then_panics() {
    let f = || -> ChipDescription { ChipDescription::new(1, 0, 1, vec![Link::new(5, 0)]) };
    let result = std::panic::catch_unwind(f);
    assert!(result.is_err());

    let links = vec![Link::new(0, 0), Link::new(5, 0)];
    let f = || -> ChipDescription { ChipDescription::new(1, 0, 1, links) };
    let result = std::panic::catch_unwind(f);
    assert!(result.is_err());
}

#[test]
#[should_panic]
fn given_link_targets_input_then_panics() {
    ChipDescription::new(
        2,
        0,
        2,
        vec![Link::new(0, 1), Link::new(0, 2), Link::new(1, 3)],
    );
}

#[test]
#[should_panic]
fn given_link_sources_output_then_panics() {
    ChipDescription::new(
        1,
        0,
        2,
        vec![Link::new(0, 1), Link::new(0, 2), Link::new(1, 2)],
    );
}

#[test]
#[should_panic]
fn given_output_targeted_by_two_links_then_panics() {
    let links = vec![Link::new(0, 2), Link::new(1, 2)];
    ChipDescription::new(2, 0, 1, links);
}

#[test]
fn given_any_node_unconnected_then_panics() {
    let f = || -> ChipDescription { ChipDescription::new(1, 0, 2, vec![Link::new(0, 1)]) };
    assert!(std::panic::catch_unwind(f).is_err());

    let f = || -> ChipDescription {
        ChipDescription::new(
            1,
            1,
            1,
            vec![Link::new(0, 2), Link::new(1, 1), Link::new(1, 1)],
        )
    };
    assert!(std::panic::catch_unwind(f).is_err());
}

#[test]
#[should_panic]
fn given_nand_with_no_targets_then_panics() {
    let links = vec![Link::new(0, 1), Link::new(0, 1), Link::new(0, 2)];
    ChipDescription::new(1, 1, 1, links);
}

#[test]
#[should_panic]
fn given_nand_no_sources_then_panics() {
    ChipDescription::new(1, 1, 2, vec![Link::new(0, 2), Link::new(1, 3)]);
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
    ChipDescription::new(3, 1, 1, links);
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
    ChipDescription::new(1, 1, 1, links);
}