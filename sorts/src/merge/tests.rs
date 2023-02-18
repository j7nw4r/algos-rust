#[test]
fn run_tests() {
    let mut input = [3u32, 2, 1, 4, 26, 17];
    let output = [1u32, 2, 3, 4, 17, 26];

    super::sort(&mut input);

    assert_eq!(input[..], output[..])
}
