use chronicler::extract_between_dashes;

#[test]
fn public_api_extracts_section() {
    let input = "prefix ---payload--- trailing";
    assert_eq!(extract_between_dashes(input), Some("payload"));
}
