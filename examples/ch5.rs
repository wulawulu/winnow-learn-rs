fn main() {}

//tests
#[cfg(test)]
mod tests {
    use winnow::Parser;
    use winnow::Result;
    use winnow::combinator::separated;
    use winnow_learn::parse_digits;

    #[test]
    fn test_separated() {
        fn parse_list(input: &mut &str) -> Result<Vec<usize>> {
            separated(0.., parse_digits, ",").parse_next(input)
        }
        let mut input = "0x1a2b,0x3c4d,0x5e6f Hello";

        let digits = parse_list.parse_next(&mut input).unwrap();

        assert_eq!(input, " Hello");
        assert_eq!(digits, vec![0x1a2b, 0x3c4d, 0x5e6f]);

        assert!(parse_digits(&mut "ghiWorld").is_err());
    }

    #[test]
    fn test_separated2() {
        fn parse_list(input: &mut &str) -> Result<()> {
            separated(0.., parse_digits, ",").parse_next(input)
        }
        fn take_list<'s>(input: &mut &'s str) -> Result<&'s str> {
            parse_list.take().parse_next(input)
        }

        let mut input = "0x1a2b,0x3c4d,0x5e6f Hello";

        let digits = take_list.parse_next(&mut input).unwrap();

        assert_eq!(input, " Hello");
        assert_eq!(digits, "0x1a2b,0x3c4d,0x5e6f");

        assert!(parse_digits(&mut "ghiWorld").is_err());
    }
}
