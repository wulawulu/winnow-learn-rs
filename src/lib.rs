use winnow::combinator::dispatch;
use winnow::combinator::fail;
use winnow::token::take;
use winnow::Parser;
use winnow::Result;
use winnow::token::take_while;

pub fn parse_bin_digits<'s>(input: &mut &'s str) -> Result<&'s str> {
    take_while(1.., (('0'..='1'),)).parse_next(input)
}

pub fn parse_oct_digits<'s>(input: &mut &'s str) -> Result<&'s str> {
    take_while(1.., (('0'..='7'),)).parse_next(input)
}

pub fn parse_dec_digits<'s>(input: &mut &'s str) -> Result<&'s str> {
    take_while(1.., (('0'..='9'),)).parse_next(input)
}

pub fn parse_hex_digits<'s>(input: &mut &'s str) -> Result<&'s str> {
    take_while(1.., (('0'..='9'), ('a'..='f'), ('A'..='F'))).parse_next(input)
}

pub fn parse_digits(input: &mut &str) -> Result<usize> {
    dispatch!(take(2usize);
    "0b" => parse_bin_digits.try_map(|s| usize::from_str_radix(s, 2)),
    "0o" => parse_oct_digits.try_map(|s| usize::from_str_radix(s, 8)),
    "0d" => parse_dec_digits.try_map(|s| usize::from_str_radix(s, 10)),
    "0x" => parse_hex_digits.try_map(|s| usize::from_str_radix(s, 16)),
    _ => fail,
    )
    .parse_next(input)
}

#[derive(Debug, PartialEq, Eq)]
pub struct Hex(pub usize);

impl std::str::FromStr for Hex {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_digits
            .map(Hex)
            .parse(s)
            .map_err(|e| anyhow::format_err!("{e}"))
    }
}