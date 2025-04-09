use winnow::Parser;

use winnow_learn::parse_digits;



fn main() {
    let mut input = "0x1a2b Hello";

    let digits = parse_digits.parse_next(&mut input).unwrap();

    assert_eq!(input, " Hello");
    assert_eq!(digits, 0x1a2b);

    assert!(parse_digits(&mut "ghiWorld").is_err());
}
