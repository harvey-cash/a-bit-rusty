use ntest::timeout;

use crate::chip::{
    types::*,
    chip::{Chip, CustomChip, GroundChip, NAndChip, SupplyChip, Tickable}, 
    chip_description::ChipDescription
};

#[test]
fn given_ground_then_output_0() {
    let chip = GroundChip::new();
    assert_eq!(chip.read_pin(0), 0);
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
    assert_eq!(chip.read_pin(0), 1);
}

#[test]
fn given_supply_when_off_then_output_0() {
    let mut chip = SupplyChip::new();
    chip.turn_off();
    assert_eq!(chip.read_pin(0), 0);
}

#[test]
fn given_supply_when_on_then_output_1() {
    let mut chip = SupplyChip::new();
    chip.turn_off();
    chip.turn_on();
    assert_eq!(chip.read_pin(0), 1);
}

#[test]
#[should_panic]
fn given_bad_description_then_panics() {
    let description = ChipDescription::new(0, 1, 0, vec![Link::new(0, 0)]);
    CustomChip::new(description);
}

#[test]
fn given_one_link_when_input_0_then_output_0() {
    let description = ChipDescription::new(1, 1, 0, vec![Link::new(2, 3)]);
    let mut chip = CustomChip::new(description);
    chip.write_pin(CustomChip::SUPPLY_PIN, 1);
    chip.write_pin(2, 0);
    chip.tick();
    assert_eq!(chip.read_pin(3), 0);
}

#[test]
fn given_one_link_when_input_1_then_output_1() {
    let description = ChipDescription::new(1, 1, 0, vec![Link::new(2, 3)]);
    let mut chip = CustomChip::new(description);
    chip.write_pin(CustomChip::SUPPLY_PIN, 1);
    chip.write_pin(2, 1);
    chip.tick();
    assert_eq!(chip.read_pin(3), 1);
}

#[test]
fn given_supply_0_then_output_0() {
    let description = ChipDescription::new(1, 1, 0, vec![Link::new(2, 3)]);
    let mut chip = CustomChip::new(description);
    chip.write_pin(CustomChip::SUPPLY_PIN, 0);
    chip.write_pin(2, 1);
    chip.tick();
    assert_eq!(chip.read_pin(3), 0);
}

#[test]
fn given_ground_1_then_output_0() {
    let description = ChipDescription::new(1, 1, 0, vec![Link::new(2, 3)]);
    let mut chip = CustomChip::new(description);
    chip.write_pin(CustomChip::GROUND_PIN, 1);
    chip.write_pin(2, 1);
    chip.tick();
    assert_eq!(chip.read_pin(3), 0);
}

#[test]
fn given_one_link_then_output_not_set_before_tick() {
    let description = ChipDescription::new(1, 1, 0, vec![Link::new(2, 3)]);
    let mut chip = CustomChip::new(description);
    chip.write_pin(CustomChip::SUPPLY_PIN, 1);
    chip.write_pin(2, 1);
    assert_eq!(chip.read_pin(3), 0);
}

#[test]
fn given_two_separate_links_then_outputs_equal_corresponding_inputs() {
    let links = vec![Link::new(2, 4), Link::new(3, 5)];
    let description = ChipDescription::new(2, 2, 0, links);
    let mut chip = CustomChip::new(description);
    chip.write_pin(CustomChip::SUPPLY_PIN, 1);

    chip.write_pin(2, 0);
    chip.write_pin(3, 1);
    chip.tick();
    assert_eq!(chip.read_pin(4), 0);
    assert_eq!(chip.read_pin(5), 1);
}

#[test]
fn given_two_crossed_links_then_outputs_equal_corresponding_inputs() {
    let links = vec![Link::new(2, 5), Link::new(3, 4)];
    let description = ChipDescription::new(2, 2, 0, links);
    let mut chip = CustomChip::new(description);
    chip.write_pin(CustomChip::SUPPLY_PIN, 1);

    chip.write_pin(2, 0);
    chip.write_pin(3, 1);
    chip.tick();
    assert_eq!(chip.read_pin(4), 1);
    assert_eq!(chip.read_pin(5), 0);
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
    chip.write_pin(CustomChip::SUPPLY_PIN, 1);
    chip.write_pin(2, 0);
    chip.write_pin(3, 0);
    chip.tick();
    assert_eq!(chip.read_pin(4), 1);
}

#[test]
fn given_nand_when_inputs_both_1_then_output_is_0() {
    let mut chip = NAndChip::new();
    chip.write_pin(CustomChip::SUPPLY_PIN, 1);
    chip.write_pin(2, 1);
    chip.write_pin(3, 1);
    chip.tick();
    assert_eq!(chip.read_pin(4), 0);
}

#[test]
fn given_nand_when_single_input_1_then_output_is_1() {
    let mut chip = NAndChip::new();
    chip.write_pin(CustomChip::SUPPLY_PIN, 1);
    chip.write_pin(2, 1);
    chip.write_pin(3, 0);
    chip.tick();
    assert_eq!(chip.read_pin(4), 1);

    chip.write_pin(2, 0);
    chip.write_pin(3, 1);
    chip.tick();
    assert_eq!(chip.read_pin(4), 1);
}

#[test]
fn given_nand_when_supply_0_then_output_is_0() {
    let mut chip = NAndChip::new();
    chip.write_pin(2, 1);
    chip.write_pin(3, 0);
    chip.tick();
    assert_eq!(chip.read_pin(4), 0);

    chip.write_pin(2, 0);
    chip.write_pin(3, 1);
    chip.tick();
    assert_eq!(chip.read_pin(4), 0);
}

#[test]
fn given_nand_linked_sources_when_input_0_then_output_1() {
    let (input, output, nand) = (2, 3, 4);
    let links = vec![Link::new(input, nand), Link::new(input, nand), Link::new(nand, output)];
    let description = ChipDescription::new(1, 1, 1, links);
    let mut chip = CustomChip::new(description);
    chip.write_pin(CustomChip::SUPPLY_PIN, 1);
    chip.write_pin(input, 0);
    chip.tick();
    assert_eq!(chip.read_pin(output), 1);
}

#[test]
fn given_nand_linked_sources_when_input_1_then_output_0() {
    let (input, output, nand) = (2, 3, 4);
    let links = vec![Link::new(input, nand), Link::new(input, nand), Link::new(nand, output)];
    let description = ChipDescription::new(1, 1, 1, links);
    let mut chip = CustomChip::new(description);
    chip.write_pin(CustomChip::SUPPLY_PIN, 1);
    chip.write_pin(input, 1);
    chip.tick();
    assert_eq!(chip.read_pin(output), 0);
}

#[test]
fn given_two_nots_in_series_when_input_0_then_output_0() {
    let (input, output, nand_1, nand_2) = (2, 3, 4, 5);
    let links = vec![
        Link::new(input, nand_1),
        Link::new(input, nand_1),
        Link::new(nand_1, nand_2),
        Link::new(nand_1, nand_2),
        Link::new(nand_2, output),
    ];
    let description = ChipDescription::new(1, 1, 2, links);
    let mut chip = CustomChip::new(description);
    chip.write_pin(CustomChip::SUPPLY_PIN, 1);

    chip.write_pin(input, 0);
    chip.tick();
    assert_eq!(chip.read_pin(output), 0);
}

#[test]
fn given_two_nots_in_series_when_input_1_then_output_1() {
    let (input, output, nand_1, nand_2) = (2, 3, 4, 5);
    let links = vec![
        Link::new(input, nand_1),
        Link::new(input, nand_1),
        Link::new(nand_1, nand_2),
        Link::new(nand_1, nand_2),
        Link::new(nand_2, output),
    ];
    let description = ChipDescription::new(1, 1, 2, links);
    let mut chip = CustomChip::new(description);
    chip.write_pin(CustomChip::SUPPLY_PIN, 1);

    chip.write_pin(input, 1);
    chip.tick();
    assert_eq!(chip.read_pin(output), 1);
}

#[test]
#[timeout(5)]
fn given_cycle_when_ticked_then_does_not_loop_forever() {
    let (input, output, nand) = (2, 3, 4);
    let links = vec![Link::new(input, nand), Link::new(nand, nand), Link::new(nand, output)];
    let description = ChipDescription::new(1, 1, 1, links);
    let mut chip = CustomChip::new(description);
    chip.write_pin(CustomChip::SUPPLY_PIN, 1);
    chip.write_pin(input, 1);
    chip.tick();
    assert_eq!(chip.read_pin(output), 1);
}

#[test]
#[timeout(5)]
fn given_cycle_nand_when_ticked_then_output_oscillates() {
    let (input, output, nand) = (2, 3, 4);
    let links = vec![Link::new(input, nand), Link::new(nand, nand), Link::new(nand, output)];
    let description = ChipDescription::new(1, 1, 1, links);
    let mut chip = CustomChip::new(description);
    chip.write_pin(CustomChip::SUPPLY_PIN, 1);
    chip.write_pin(input, 1);

    chip.tick();
    assert_eq!(chip.read_pin(output), 1);

    chip.tick();
    assert_eq!(chip.read_pin(output), 0);

    chip.tick();
    assert_eq!(chip.read_pin(output), 1);
}

#[test]
#[timeout(5)]
fn given_three_nand_loop_when_ticked_then_oscillates() {
    let (input, output, nand_1, nand_2, nand_3) = (2, 3, 4, 5, 6);
    let links: Vec<Link> = vec![
        Link::new(input, nand_1),
        Link::new(nand_1, nand_2),
        Link::new(nand_1, nand_2),
        Link::new(nand_2, nand_3),
        Link::new(nand_2, nand_3),
        Link::new(nand_3, nand_1),
        Link::new(nand_3, output),
    ];
    let description = ChipDescription::new(1, 1, 3, links);
    let mut chip = CustomChip::new(description);
    chip.write_pin(CustomChip::SUPPLY_PIN, 1);
    chip.write_pin(input, 1);

    chip.tick();
    assert_eq!(chip.read_pin(output), 1);

    chip.tick();
    assert_eq!(chip.read_pin(output), 0);

    chip.tick();
    assert_eq!(chip.read_pin(output), 1);
}

#[test]
fn given_latch_when_set_does_not_reset() {
    let (set, reset, q, n1, n2) = (2, 3, 4, 5, 6);
    let links = vec![
        Link::new(set, n1), Link::new(reset, n2), Link::new(n1, n2), Link::new(n2, n1), Link::new(n1, q)
    ];
    let description = ChipDescription::new(2, 1, 2, links);
    let mut chip = CustomChip::new(description);
    chip.write_pin(CustomChip::SUPPLY_PIN, 1);

    chip.write_pin(set, 1);
    chip.tick();
    assert_eq!(chip.read_pin(q), 1);

    chip.write_pin(set, 0);
    chip.tick();    
    assert_eq!(chip.read_pin(q), 1);
}

#[test]
fn given_latch_when_reset_then_output_is_0() {
    let (set, reset, q, n1, n2) = (2, 3, 4, 5, 6);
    let links = vec![
        Link::new(set, n1), Link::new(reset, n2), Link::new(n1, n2), Link::new(n2, n1), Link::new(n1, q)
    ];
    let description = ChipDescription::new(2, 1, 2, links);
    let mut chip = CustomChip::new(description);
    chip.write_pin(CustomChip::SUPPLY_PIN, 1);

    chip.write_pin(set, 1);
    chip.tick();
    assert_eq!(chip.read_pin(q), 1);

    chip.write_pin(reset, 1);
    chip.tick();    
    assert_eq!(chip.read_pin(q), 0);
}

#[test]
fn given_supply_0_when_ticked_internal_state_clears() {
    let (set, reset, q, n1, n2) = (2, 3, 4, 5, 6);
    let links = vec![
        Link::new(set, n1), Link::new(reset, n2), Link::new(n1, n2), Link::new(n2, n1), Link::new(n1, q)
    ];
    let description = ChipDescription::new(2, 1, 2, links);
    let mut chip = CustomChip::new(description);
    chip.write_pin(CustomChip::SUPPLY_PIN, 1);

    chip.write_pin(set, 1);
    chip.tick();
    assert_eq!(chip.read_pin(q), 1);

    chip.write_pin(CustomChip::SUPPLY_PIN, 0);
    chip.write_pin(set, 0);
    chip.tick();

    chip.write_pin(CustomChip::SUPPLY_PIN, 1);
    assert_eq!(chip.read_pin(q), 0);
}