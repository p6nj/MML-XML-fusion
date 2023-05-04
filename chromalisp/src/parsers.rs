use nom::{
    branch::alt,
    bytes::complete::{is_not, take_till1},
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
    assert_eq!(Ok(("hi btw", ())), comment("; comment comment\nhi btw"));
    assert_eq!(
        Ok(("clean", ())),
        junk("; some junk \n    ; that's still junk\n;more junk\nclean")
    )
}

fn wrapper_parser_generator<'a, F, P, O>(
    tag: char,
    parser: P,
    transform: F,
) -> impl Fn(&'a str) -> IResult<&'a str, Wrappers>
where
    F: Fn(O) -> Wrappers + Copy,
    P: Fn(&'a str) -> IResult<&'a str, O> + Copy,
{
    move |i: &'a str| {
        map_res(preceded(pair(char(tag), junk), parser), move |o| {
            Ok::<Wrappers, ErrorKind>(transform(o))
        })(i)
    }
}

fn length(i: &'static str) -> IResult<&'static str, Wrappers> {
    wrapper(W::Length)(i)
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

fn wrapper(w: W) -> impl Fn(&'static str) -> IResult<&'static str, Wrappers> {
    wrapper_parser_generator(
        w.tag(),
        match w {
            W::Song => |i| -> IResult<&str, &str> {
                alt((
                    delimited(char('\''), take_till1(|c| c == '\''), char('\'')),
                    delimited(char('"'), take_till1(|c| c == '"'), char('"')),
                    take_till1(|c| c == ' '),
                ))(i)
            },
            W::Album => todo!(),
            W::Artist => todo!(),
            W::Year => todo!(),
            W::Tempo => todo!(),
            W::Accel => todo!(),
            W::NoteDef => todo!(),
            W::Channel => todo!(),
            W::Instrument => todo!(),
            W::Length => digit1,
            W::Octave => digit1,
            W::Loop => todo!(),
            W::Glissando => todo!(),
            W::Vibrato => todo!(),
            W::Volume => todo!(),
            W::VolumeFader => todo!(),
            W::ADSR => todo!(),
            W::Singleton => todo!(),
            W::Mask => todo!(),
            W::Masked => todo!(),
            W::Test => todo!(),
            W::Test2 => todo!(),
        },
        move |o| match w {
            W::Song => todo!(),
            W::Album => todo!(),
            W::Artist => todo!(),
            W::Year => todo!(),
            W::Tempo => todo!(),
            W::Accel => todo!(),
            W::NoteDef => todo!(),
            W::Channel => todo!(),
            W::Instrument => todo!(),
            W::Length => Wrappers::Length(o.parse::<u8>().unwrap(), vec![]),
            W::Octave => Wrappers::Octave(o.parse::<u8>().unwrap(), vec![]),
            W::Loop => todo!(),
            W::Glissando => todo!(),
            W::Vibrato => todo!(),
            W::Volume => todo!(),
            W::VolumeFader => todo!(),
            W::ADSR => todo!(),
            W::Singleton => todo!(),
            W::Mask => todo!(),
            W::Masked => todo!(),
            W::Test => todo!(),
            W::Test2 => todo!(),
        },
    )
}
