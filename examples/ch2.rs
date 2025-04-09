use winnow::Parser;
use winnow::Result;

pub fn parse_prefix<'s>(input: &mut &'s str) -> Result<char> {
    let c = '0'.parse_next(input)?;
    Ok(c)
}

fn main() {
    let mut input = "0x1a2b Hello";

    let output = parse_prefix.parse_next(&mut input).unwrap();

    assert_eq!(input, "x1a2b Hello");
    assert_eq!(output, '0');

    assert!(parse_prefix.parse_next(&mut "d").is_err());
}

//tests
#[cfg(test)]
mod tests {
    use winnow::ascii::hex_digit1;

    use super::*;

    #[test]
    fn test_parse_prefix() {
        fn parse_digits<'s>(input: &mut &'s str) -> Result<&'s str> {
            hex_digit1.parse_next(input)
        }
        
        let mut input = "1a2b Hello";

        let output = parse_digits.parse_next(&mut input).unwrap();
        assert_eq!(input, " Hello");
        assert_eq!(output, "1a2b");

        assert!(parse_digits.parse_next(&mut "Z").is_err());
    }
}
