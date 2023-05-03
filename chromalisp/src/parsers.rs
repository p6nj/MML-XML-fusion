use nom::{
    branch::alt,
    bytes::complete::is_not,
    character::complete::{char, digit1, multispace1, newline},
    combinator::{map_res, value},
    error::ErrorKind,
    multi::many0,
    sequence::{delimited, pair, preceded},
    IResult,
};

use crate::structure::{Tagging, Wrappers, W};

pub fn parse(i: &str) {}

fn space<'a>(i: &'a str) -> IResult<&'a str, ()> {
    value((), multispace1)(i)
}

fn comment<'a>(i: &'a str) -> IResult<&'a str, ()> {
    value(
        (), // Output is thrown away.
        delimited(char(';'), is_not("\n\r"), newline),
    )(i)
}

fn junk<'a>(i: &'a str) -> IResult<&'a str, ()> {
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

fn wrapper<'a, F, P, O>(
    wrap: W,
    parser: P,
    transform: F,
) -> impl Fn(&'a str) -> IResult<&'a str, Wrappers>
where
    F: Fn(O) -> Wrappers + Copy,
    P: Fn(&'a str) -> IResult<&'a str, O> + Copy,
{
    move |i: &'a str| {
        map_res(preceded(pair(char(wrap.tag()), junk), parser), move |o| {
            Ok::<Wrappers, ErrorKind>(transform(o))
        })(i)
    }
}

fn length<'a>(i: &'a str) -> IResult<&'a str, Wrappers> {
    wrapper(W::Length, digit1, |o| {
        Wrappers::Length(o.parse::<u8>().unwrap(), vec![])
    })(i)
}

#[test]
fn length_parser() {
    assert_eq!(Ok(("", Wrappers::Length(1, vec![]))), length("l 1"));
    assert_eq!(
        Err(nom::Err::Error(nom::error::Error {
            input: "what",
            code: ErrorKind::Char
        })),
        length("what")
    );
}

trait Parser<O> {
    fn parse<'a>(self, i: &'a str) -> IResult<&'a str, O>;
}

// impl<Wrappers> Parser<Wrappers> for W {}
