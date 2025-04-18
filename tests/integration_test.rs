use a_bit_rusty::add;

#[test]
fn given_two_numbers_then_result_is_sum() {
    let result = add(2, 5);
    assert_eq!(result, 7);
    panic!("Deliberate panic to test CI/CD pipeline");
}
