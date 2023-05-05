use crate::structure::{AccelConfig, Repartition, Tagging, Time, Wrappers, W};
use nom::{
    branch::alt,
    bytes::complete::{is_not, take_till1},
    character::{
        complete::{char, digit1, multispace1, newline},
        streaming::hex_digit1,
    },
    combinator::{map_res, recognize, value},
    error::{ErrorKind, VerboseError},
    multi::many0,
    sequence::{delimited, pair, preceded, separated_pair},
    IResult,
};
use std::time::Duration;

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

fn hex_or_dec(i: &str) -> IResult<&str, &str> {
    alt((recognize(preceded(char('\''), digit1)), hex_digit1))(i)
}

fn hex_or_dec_resolve(i: &str) -> (u8, bool) {
    alt((
        map_res(
            preceded(char::<&str, VerboseError<&str>>('\''), digit1),
            move |s: &str| -> Result<(u8, bool), ErrorKind> {
                Ok((s.parse::<u8>().unwrap(), false))
            },
        ),
        map_res(hex_digit1, move |o| -> Result<(u8, bool), ErrorKind> {
            Ok((u8::from_str_radix(o, 16).unwrap(), true))
        }),
    ))(i)
    .unwrap()
    .1
}

fn wrapper(w: W) -> impl Fn(&'static str) -> IResult<&'static str, Wrappers> {
    wrapper_parser_generator(
        w.tag(),
        move |i| -> IResult<&str, Vec<&str>> {
            match w {
                W::Song | W::Album | W::Artist | W::Channel => map_res(
                    alt((
                        delimited(char('\''), take_till1(|c| c == '\''), char('\'')),
                        delimited(char('"'), take_till1(|c| c == '"'), char('"')),
                        take_till1(|c| c == ' '),
                    )),
                    move |o| -> Result<Vec<&str>, ErrorKind> { Ok(vec![o]) },
                )(i),
                W::Accel => map_res(
                    separated_pair(digit1, junk, digit1),
                    move |(o1, o2)| -> Result<Vec<&str>, ErrorKind> { Ok(vec![o1, o2]) },
                )(i),
                W::NoteDef => todo!(),
                W::Instrument => todo!(),
                W::Length | W::Octave | W::Year | W::Tempo | W::Loop => {
                    map_res(digit1, move |o| -> Result<Vec<&str>, ErrorKind> {
                        Ok(vec![o])
                    })(i)
                }
                W::Glissando => map_res(
                    separated_pair(hex_or_dec, junk, hex_or_dec),
                    move |(o1, o2)| -> Result<Vec<&str>, ErrorKind> { Ok(vec![o1, o2]) },
                )(i),
                W::Vibrato => todo!(),
                W::Volume => todo!(),
                W::VolumeFader => todo!(),
                W::ADSR => todo!(),
                W::Singleton => todo!(),
                W::Mask => todo!(),
                W::Masked => todo!(),
                W::Test => todo!(),
                W::Test2 => todo!(),
            }
        },
        move |o| match w {
            W::Song => Wrappers::Song(o.get(0).unwrap().to_string(), vec![]),
            W::Album => Wrappers::Album(o.get(0).unwrap().to_string(), vec![]),
            W::Artist => Wrappers::Artist(o.get(0).unwrap().to_string(), vec![]),
            W::Year => Wrappers::Year(o.get(0).unwrap().parse().unwrap(), vec![]),
            W::Tempo => Wrappers::Tempo(o.get(0).unwrap().parse().unwrap(), vec![]),
            W::Accel => Wrappers::Accel(
                AccelConfig::new(
                    o.get(0).unwrap().parse().unwrap(),
                    o.get(1).unwrap().parse().unwrap(),
                ),
                vec![],
            ),
            W::NoteDef => todo!(),
            W::Channel => Wrappers::Channel(o.get(0).unwrap().to_string(), vec![]),
            W::Instrument => todo!(),
            W::Length => Wrappers::Length(o.get(0).unwrap().parse().unwrap(), vec![]),
            W::Octave => Wrappers::Octave(o.get(0).unwrap().parse().unwrap(), vec![]),
            W::Loop => Wrappers::Loop(o.get(0).unwrap().parse().unwrap(), vec![]),
            W::Glissando => Wrappers::Glissando(
                Repartition::new(
                    {
                        let result = hex_or_dec_resolve(o.get(0).unwrap());
                        if result.1 {
                            Time::Dynamic(result.0)
                        } else {
                            Time::Static(Duration::from_millis(result.0.into()))
                        }
                    },
                    {
                        let result = hex_or_dec_resolve(o.get(1).unwrap());
                        if result.1 {
                            Time::Dynamic(result.0)
                        } else {
                            Time::Static(Duration::from_millis(result.0.into()))
                        }
                    },
                ),
                vec![],
            ),
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

#[cfg(test)]
mod base_tests {
    use super::{comment, hex_or_dec, junk, space};
    #[test]
    fn junk_parser() {
        assert_eq!(Ok(("oij", ())), space(" \n    \n oij"));
        assert_eq!(Ok(("hi btw", ())), comment("; comment comment\nhi btw"));
        assert_eq!(
            Ok(("clean", ())),
            junk("; some junk \n    ; that's still junk\n;more junk\nclean")
        )
    }
    #[test]
    fn hex_or_dec_parser() {
        assert_eq!(Ok(("", "'12")), hex_or_dec("'12"));
    }
}

#[cfg(test)]
mod wrapper_tests {
    use crate::structure::AccelConfig;

    use super::{wrapper, Wrappers, W};
    #[test]
    fn length_parser() {
        let parser = wrapper(W::Length);
        assert_eq!(Ok(("", Wrappers::Length(1, vec![]))), parser("l 1"));
    }

    #[test]
    fn song_parser() {
        let parser = wrapper(W::Song);
        assert_eq!(
            Ok(("", Wrappers::Song("Song".to_string(), vec![]))),
            parser("S Song")
        );
        assert_eq!(
            Ok(("", Wrappers::Song("Song".to_string(), vec![]))),
            parser("S \"Song\"")
        );
        assert_eq!(
            Ok(("", Wrappers::Song("Song".to_string(), vec![]))),
            parser("S 'Song'")
        );
    }

    #[test]
    fn accel_parser() {
        let parser = wrapper(W::Accel);
        assert_eq!(
            Ok(("", Wrappers::Accel(AccelConfig::new(12, 16), vec![]))),
            parser("t 12 16")
        );
    }
}
