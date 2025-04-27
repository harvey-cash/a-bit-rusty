use ntest::timeout;

use crate::chip::{
    types::*,
    chip::{Chip, CustomChip, GroundChip, NAndChip, SupplyChip, Tickable}, 
    chip_description::ChipDescription
};

// ToDo:
// [ ] Chips have a Pin for Ground, Supply, each Input and each Output.

#[test]
fn given_ground_then_output_0() {
    let chip = GroundChip::new();
    assert_eq!(chip.get_output(0), 0);
}

#[test]
fn given_ground_then_has_1_output_pin() {
    let layout = GroundChip::new().get_layout();
    assert_eq!(layout.input_pins.len(), 0);
    assert_eq!(layout.output_pins.len(), 1);
}

#[test]
fn given_supply_then_output_1() {
    let chip = SupplyChip::new();
    assert_eq!(chip.get_output(0), 1);
}

#[test]
fn given_supply_when_off_then_output_0() {
    let mut chip = SupplyChip::new();
    chip.turn_off();
    assert_eq!(chip.get_output(0), 0);
}

#[test]
fn given_supply_when_on_then_output_1() {
    let mut chip = SupplyChip::new();
    chip.turn_off();
    chip.turn_on();
    assert_eq!(chip.get_output(0), 1);
}

#[test]
#[should_panic]
fn given_bad_description_then_panics() {
    let description = ChipDescription::new(0, 1, 0, vec![Link::new(0, 0)]);
    CustomChip::new(description);
}

#[test]
fn given_one_link_when_input_0_then_output_0() {
    let description = ChipDescription::new(1, 0, 1, vec![Link::new(2, 3)]);
    let mut chip = CustomChip::new(description);
    chip.set_supply(1);
    chip.set_input(2, 0);
    chip.tick();
    assert_eq!(chip.get_output(3), 0);
}

#[test]
fn given_one_link_when_input_1_then_output_1() {
    let description = ChipDescription::new(1, 0, 1, vec![Link::new(2, 3)]);
    let mut chip = CustomChip::new(description);
    chip.set_supply(1);
    chip.set_input(2, 1);
    chip.tick();
    assert_eq!(chip.get_output(3), 1);
}

#[test]
fn given_supply_0_then_output_0() {
    let description = ChipDescription::new(1, 0, 1, vec![Link::new(2, 3)]);
    let mut chip = CustomChip::new(description);
    chip.set_supply(0);
    chip.set_input(2, 1);
    chip.tick();
    assert_eq!(chip.get_output(3), 0);
}

#[test]
fn given_ground_1_then_output_0() {
    let description = ChipDescription::new(1, 0, 1, vec![Link::new(2, 3)]);
    let mut chip = CustomChip::new(description);
    chip.set_ground(1);
    chip.set_input(2, 1);
    chip.tick();
    assert_eq!(chip.get_output(3), 0);
}

#[test]
fn given_one_link_then_output_not_set_before_tick() {
    let description = ChipDescription::new(1, 0, 1, vec![Link::new(2, 3)]);
    let mut chip = CustomChip::new(description);
    chip.set_supply(1);
    chip.set_input(2, 1);
    assert_eq!(chip.get_output(3), 0);
}

#[test]
fn given_two_separate_links_then_outputs_equal_corresponding_inputs() {
    let links = vec![Link::new(2, 4), Link::new(3, 5)];
    let description = ChipDescription::new(2, 0, 2, links);
    let mut chip = CustomChip::new(description);
    chip.set_supply(1);

    chip.set_input(2, 0);
    chip.set_input(3, 1);
    chip.tick();
    assert_eq!(chip.get_output(4), 0);
    assert_eq!(chip.get_output(5), 1);
}

#[test]
fn given_two_crossed_links_then_outputs_equal_corresponding_inputs() {
    let links = vec![Link::new(2, 5), Link::new(3, 4)];
    let description = ChipDescription::new(2, 0, 2, links);
    let mut chip = CustomChip::new(description);
    chip.set_supply(1);

    chip.set_input(2, 0);
    chip.set_input(3, 1);
    chip.tick();
    assert_eq!(chip.get_output(4), 1);
    assert_eq!(chip.get_output(5), 0);
}

#[test]
fn given_nand_then_has_ground_pin() {
    let layout = NAndChip::new().get_layout();
    assert_eq!(layout.ground_pins.len(), 1);
}

#[test]
fn given_nand_then_has_supply_pin() {
    let layout = NAndChip::new().get_layout();
    assert_eq!(layout.supply_pins.len(), 1);
}

#[test]
fn given_nand_when_inputs_both_0_then_output_is_1() {
    let mut chip = NAndChip::new();
    chip.set_supply(1);
    chip.set_input(2, 0);
    chip.set_input(3, 0);
    chip.tick();
    assert_eq!(chip.get_output(4), 1);
}

#[test]
fn given_nand_when_inputs_both_1_then_output_is_0() {
    let mut chip = NAndChip::new();
    chip.set_supply(1);
    chip.set_input(2, 1);
    chip.set_input(3, 1);
    chip.tick();
    assert_eq!(chip.get_output(4), 0);
}

#[test]
fn given_nand_when_single_input_1_then_output_is_1() {
    let mut chip = NAndChip::new();
    chip.set_supply(1);
    chip.set_input(2, 1);
    chip.set_input(3, 0);
    chip.tick();
    assert_eq!(chip.get_output(4), 1);

    chip.set_input(2, 0);
    chip.set_input(3, 1);
    chip.tick();
    assert_eq!(chip.get_output(4), 1);
}

#[test]
fn given_nand_when_supply_0_then_output_is_0() {
    let mut chip = NAndChip::new();
    chip.set_input(2, 1);
    chip.set_input(3, 0);
    chip.tick();
    assert_eq!(chip.get_output(4), 0);

    chip.set_input(2, 0);
    chip.set_input(3, 1);
    chip.tick();
    assert_eq!(chip.get_output(4), 0);
}

#[test]
fn given_nand_linked_sources_when_input_0_then_output_1() {
    let links = vec![Link::new(2, 3), Link::new(2, 3), Link::new(3, 4)];
    let description = ChipDescription::new(1, 1, 1, links);
    let mut chip = CustomChip::new(description);
    chip.set_supply(1);
    chip.set_input(2, 0);
    chip.tick();
    assert_eq!(chip.get_output(3), 1);
}

#[test]
fn given_nand_linked_sources_when_input_1_then_output_0() {
    let links = vec![Link::new(2, 3), Link::new(2, 3), Link::new(3, 4)];
    let description = ChipDescription::new(1, 1, 1, links);
    let mut chip = CustomChip::new(description);
    chip.set_supply(1);
    chip.set_input(2, 1);
    chip.tick();
    assert_eq!(chip.get_output(3), 0);
}

#[test]
fn given_two_nots_in_series_when_input_0_then_output_0() {
    let links = vec![
        Link::new(2, 4),
        Link::new(2, 4),
        Link::new(4, 3),
        Link::new(4, 3),
        Link::new(3, 5),
    ];
    let description = ChipDescription::new(1, 2, 1, links);
    let mut chip = CustomChip::new(description);
    chip.set_supply(1);

    chip.set_input(2, 0);
    chip.tick();
    assert_eq!(chip.get_output(3), 0);
}

#[test]
fn given_two_nots_in_series_when_input_1_then_output_1() {
    let links = vec![
        Link::new(2, 4),
        Link::new(2, 4),
        Link::new(4, 3),
        Link::new(4, 3),
        Link::new(3, 5),
    ];
    let description = ChipDescription::new(1, 2, 1, links);
    let mut chip = CustomChip::new(description);
    chip.set_supply(1);

    chip.set_input(2, 1);
    chip.tick();
    assert_eq!(chip.get_output(3), 1);
}

#[test]
#[timeout(5)]
fn given_cycle_when_ticked_then_does_not_loop_forever() {
    let links = vec![Link::new(2, 3), Link::new(3, 3), Link::new(3, 4)];
    let description = ChipDescription::new(1, 1, 1, links);
    let mut chip = CustomChip::new(description);
    chip.set_supply(1);
    chip.set_input(2, 1);
    chip.tick();
    assert_eq!(chip.get_output(3), 1);
}

#[test]
#[timeout(5)]
fn given_cycle_nand_when_ticked_then_output_oscillates() {
    let description = ChipDescription::new(
        1,
        1,
        1,
        vec![Link::new(2, 3), Link::new(3, 3), Link::new(3, 4)],
    );
    let mut chip = CustomChip::new(description);
    chip.set_supply(1);
    chip.set_input(2, 1);

    chip.tick();
    assert_eq!(chip.get_output(3), 1);

    chip.tick();
    assert_eq!(chip.get_output(3), 0);

    chip.tick();
    assert_eq!(chip.get_output(3), 1);
}

#[test]
#[timeout(5)]
fn given_three_nand_loop_when_ticked_then_oscillates() {
    let description = ChipDescription::new(
        1,
        3,
        1,
        vec![
            Link::new(2, 3),
            Link::new(3, 4),
            Link::new(3, 4),
            Link::new(4, 5),
            Link::new(4, 5),
            Link::new(5, 3),
            Link::new(5, 6),
        ],
    );
    let mut chip = CustomChip::new(description);
    chip.set_supply(1);
    chip.set_input(2, 1);

    chip.tick();
    assert_eq!(chip.get_output(3), 1);

    chip.tick();
    assert_eq!(chip.get_output(3), 0);

    chip.tick();
    assert_eq!(chip.get_output(3), 1);
}
