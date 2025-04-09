fn main() {}

//tests
#[cfg(test)]
mod tests {
    use winnow::Parser;
    use winnow::Result;
    use winnow::combinator::alt;
    use winnow_learn::parse_bin_digits;
    use winnow_learn::parse_dec_digits;
    use winnow_learn::parse_hex_digits;
    use winnow_learn::parse_oct_digits;

    #[test]
    fn test_parse_prefix() {
        fn parse_digits<'s>(input: &mut &'s str) -> Result<(&'s str, &'s str)> {
            alt((
                ("0b", parse_bin_digits),
                ("0o", parse_oct_digits),
                ("0d", parse_dec_digits),
                ("0x", parse_hex_digits),
            ))
            .parse_next(input)
        }

        let mut input = "0x1a2b Hello";

        let (prefix, digits) = parse_digits.parse_next(&mut input).unwrap();

        assert_eq!(input, " Hello");
        assert_eq!(prefix, "0x");
        assert_eq!(digits, "1a2b");

        assert!(parse_digits(&mut "ghiWorld").is_err());
    }
}
