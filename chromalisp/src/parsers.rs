use nom::{
    branch::alt,
    bytes::complete::is_not,
    character::complete::{char, digit1, multispace1, newline},
    combinator::{map_res, value},
    error::ParseError,
    multi::many0,
    sequence::{delimited, pair, preceded, Tuple},
    IResult, Map, Parser,
};

use crate::structure::Wrappers;

pub fn parse(i: &str) -> Result<(&str, char), nom::Err<()>> {
    let length = char::<&str, ()>('l');
    length(i)
}

fn space<'a>(i: &'a str) -> IResult<&'a str, (), nom::error::VerboseError<&str>> {
    value((), multispace1)(i)
}

fn comment<'a>(i: &'a str) -> IResult<&'a str, (), nom::error::VerboseError<&str>> {
    value(
        (), // Output is thrown away.
        delimited(char(';'), is_not("\n\r"), newline),
    )(i)
}

fn junk<'a>(i: &'a str) -> IResult<&'a str, (), nom::error::VerboseError<&str>> {
    value((), many0(alt((space, comment))))(i)
}

#[test]
fn junk_parser() {
    assert_eq!(Ok(("oij", ())), space(" \n    \n oij"));
    assert_eq!(Ok(("hi btw", ())), comment("; comment comemtn\nhi btw"));
    assert_eq!(
        Ok(("clean", ())),
        junk("; some junk \n    ; that's still junk\n;more junk\nclean")
    )
}

fn length<'a>(i: &'a str) -> IResult<&'a str, Wrappers, nom::error::VerboseError<&str>> {
    map_res(preceded(pair(char('l'), junk), digit1), move |o| {
        Ok::<Wrappers, nom::error::VerboseError<&str>>(Wrappers::Length(
            o.parse::<u8>().unwrap(),
            vec![],
        ))
    })(i)
}

#[test]
fn length_parser() {
    assert_eq!(Ok(("", Wrappers::Length(1, vec![]))), length("l 1"));
}
