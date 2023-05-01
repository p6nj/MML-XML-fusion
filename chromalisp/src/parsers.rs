use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{char, space0},
    combinator::{map, opt},
    multi::{many0, separated_list1},
    sequence::{delimited, tuple},
    IResult,
};

use crate::structure::Wrappers;

fn parse_first(input: &str) -> IResult<&str, Wrappers> {
    map(
        tuple((
            tag("first"),
            space0,
            take_while1(|c: char| c != '('),
            delimited(
                char('('),
                separated_list1(tag(" "), parse_enum_object),
                char(')'),
            ),
        )),
        |(_, _, param, objects)| Wrappers::First(param.to_owned(), objects),
    )(input)
}

fn parse_second(input: &str) -> IResult<&str, Wrappers> {
    map(
        tuple((
            tag("second"),
            space0,
            take_while1(|c: char| c != '('),
            delimited(
                char('('),
                separated_list1(tag(" "), parse_enum_object),
                char(')'),
            ),
        )),
        |(_, _, param, objects)| Wrappers::Second(param.to_owned(), objects),
    )(input)
}

// Add more parsers here for other enum objects as needed

fn parse_enum_object(input: &str) -> IResult<&str, Wrappers> {
    alt((parse_first, parse_second))(input)
}

fn parse_file(input: &str) -> IResult<&str, Vec<Wrappers>> {
    many0(delimited(space0, parse_enum_object, space0))(input)
}
