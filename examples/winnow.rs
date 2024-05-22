use winnow::{combinator::trace, token::take_while, PResult, Parser};

fn parse_prefix<'s>(input: &mut &'s str) -> PResult<&'s str> {
    trace("parse_prefix", "0x").parse_next(input)
}

fn parse_digits<'s>(input: &mut &'s str) -> PResult<&'s str> {
    trace(
        "parse_digits",
        take_while(1.., (('0'..='9'), ('A'..='F'), ('a'..='f'))),
    )
    .parse_next(input)
}

fn main() {
    let mut input = "0x1a2b3c Hello";

    let prefix = parse_prefix.parse_next(&mut input).unwrap();
    println!("{input}");
    let digits = parse_digits.parse_next(&mut input).unwrap();

    assert_eq!(prefix, "0x");
    assert_eq!(digits, "1a2b3c");
    assert_eq!(input, " Hello");
}
