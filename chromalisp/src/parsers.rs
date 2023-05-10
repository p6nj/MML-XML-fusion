use crate::structure::{
    AccelConfig, Dynamics, Repartition, Tagging, Time, VibratoConfig, VolumeFadeConfig, Wrappers, W,
};
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_till1},
    character::{
        complete::{char, digit1, multispace1, newline, one_of},
        streaming::hex_digit1,
    },
    combinator::{map_res, opt, recognize, value},
    error::{dbg_dmp, ErrorKind, VerboseError},
    multi::{count, many0, many1},
    sequence::{delimited, pair, preceded, separated_pair, terminated, tuple},
    IResult,
};
use std::time::Duration;

pub fn parse(i: &str) {}

fn space(i: &str) -> IResult<&str, ()> {
    value((), multispace1)(i)
}

fn comment(i: &str) -> IResult<&str, ()> {
    value(
        (), // Output is thrown away.
        delimited(char(';'), is_not("\n\r"), newline),
    )(i)
}

fn junk(i: &str) -> IResult<&str, ()> {
    value((), many0(alt((space, comment))))(i)
}

fn dynamics(i: &str) -> IResult<&str, Dynamics> {
    map_res(
        tuple((
            opt(char('m')),
            alt((
                map_res(many1(char('f')), move |o| -> Result<Dynamics, ErrorKind> {
                    Ok(match o.len() {
                        1 => Dynamics::Forte,
                        2 => Dynamics::Fortissimo,
                        3 => Dynamics::Fortississimo,
                        _ => unreachable!(),
                    })
                }),
                map_res(many1(char('p')), move |o| -> Result<Dynamics, ErrorKind> {
                    Ok(match o.len() {
                        1 => Dynamics::Piano,
                        2 => Dynamics::Pianissimo,
                        3 => Dynamics::Pianississimo,
                        _ => unreachable!(),
                    })
                }),
            )),
        )),
        move |(mezzo, o)| -> Result<Dynamics, ErrorKind> {
            Ok(if mezzo.is_some() {
                match o {
                    Dynamics::Forte => Dynamics::MezzoForte,
                    Dynamics::Piano => Dynamics::MezzoPiano,
                    _ => unreachable!(),
                }
            } else {
                o
            })
        },
    )(i)
}

fn bool_digit(i: &str) -> IResult<&str, bool> {
    map_res(one_of("01"), move |o| -> Result<bool, ErrorKind> {
        Ok(o == '1')
    })(i)
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

fn hex_or_dec(i: &str) -> IResult<&str, (u8, bool)> {
    alt((
        map_res(
            preceded(char('\''), digit1),
            move |s: &str| -> Result<(u8, bool), ErrorKind> {
                Ok((s.parse::<u8>().unwrap(), false))
            },
        ),
        map_res(hex, move |o| -> Result<(u8, bool), ErrorKind> {
            Ok((o, true))
        }),
    ))(i)
}

fn rephelp(i: (u8, bool)) -> Time {
    match i.1 {
        true => Time::Dynamic(i.0),
        false => Time::Static(Duration::from_millis(i.0.into())),
    }
}

fn rephelp2(i: ((u8, bool), (u8, bool))) -> (Time, Time) {
    (rephelp(i.0), rephelp(i.1))
}

fn hex(i: &str) -> IResult<&str, u8> {
    map_res(
        many1(one_of("123456789abcdef")),
        move |o| -> Result<u8, ErrorKind> {
            Ok(u8::from_str_radix(o.iter().collect::<String>().as_str(), 16).unwrap())
        },
    )(i)
}

fn arg_string<'a>(i: &'a str) -> IResult<&'a str, &'a str> {
    alt((
        delimited(char('\''), take_till1(|c| c == '\''), char('\'')),
        delimited(char('"'), take_till1(|c| c == '"'), char('"')),
        take_till1(|c| c == ' '),
    ))(i)
}

fn song(i: &str) -> IResult<&str, Wrappers> {
    wrapper_parser_generator(W::Song.tag(), arg_string, move |o| {
        Wrappers::Song(o.to_string(), vec![])
    })(i)
}

fn album(i: &str) -> IResult<&str, Wrappers> {
    wrapper_parser_generator(W::Album.tag(), arg_string, move |o| {
        Wrappers::Album(o.to_string(), vec![])
    })(i)
}

fn artist(i: &str) -> IResult<&str, Wrappers> {
    wrapper_parser_generator(W::Artist.tag(), arg_string, move |o| {
        Wrappers::Artist(o.to_string(), vec![])
    })(i)
}

fn year(i: &str) -> IResult<&str, Wrappers> {
    wrapper_parser_generator(W::Year.tag(), arg_string, move |o| {
        Wrappers::Year(o.parse().unwrap(), vec![])
    })(i)
}

fn tempo(i: &str) -> IResult<&str, Wrappers> {
    wrapper_parser_generator(W::Tempo.tag(), digit1, move |o| {
        Wrappers::Tempo(o.parse().unwrap(), vec![])
    })(i)
}

fn accel(i: &str) -> IResult<&str, Wrappers> {
    wrapper_parser_generator(
        W::Accel.tag(),
        move |i| separated_pair(digit1, junk, digit1)(i),
        move |(o1, o2)| {
            Wrappers::Accel(
                AccelConfig::new(o1.parse().unwrap(), o2.parse().unwrap()),
                vec![],
            )
        },
    )(i)
}

// NoteDef needs the parser accumulator

fn channel(i: &str) -> IResult<&str, Wrappers> {
    wrapper_parser_generator(W::Channel.tag(), arg_string, move |o| {
        Wrappers::Channel(o.to_string(), vec![])
    })(i)
}

// Instrument needs the parser accumulator

fn length(i: &str) -> IResult<&str, Wrappers> {
    wrapper_parser_generator(W::Length.tag(), digit1, move |o| {
        Wrappers::Length(o.parse().unwrap(), vec![])
    })(i)
}

fn octave(i: &str) -> IResult<&str, Wrappers> {
    wrapper_parser_generator(W::Octave.tag(), digit1, move |o| {
        Wrappers::Octave(o.parse().unwrap(), vec![])
    })(i)
}

fn loop_(i: &str) -> IResult<&str, Wrappers> {
    wrapper_parser_generator(W::Loop.tag(), digit1, move |o| {
        Wrappers::Loop(o.parse().unwrap(), vec![])
    })(i)
}

fn glissando(i: &str) -> IResult<&str, Wrappers> {
    wrapper_parser_generator(
        W::Glissando.tag(),
        move |i| separated_pair(hex_or_dec, junk, hex_or_dec)(i),
        move |(o1, o2)| Wrappers::Glissando(Repartition::new(rephelp2((o1, o2))), vec![]),
    )(i)
}

fn vibrato(i: &str) -> IResult<&str, Wrappers> {
    wrapper_parser_generator(
        W::Vibrato.tag(),
        move |i| {
            tuple((
                terminated(hex, junk),
                terminated(hex, junk),
                terminated(hex_or_dec, junk),
                hex_or_dec,
            ))(i)
        },
        move |(amp, fre, start, end)| {
            Wrappers::Vibrato(
                Repartition::new(rephelp2((start, end))),
                VibratoConfig::new(amp, fre),
                vec![],
            )
        },
    )(i)
}

fn volume(i: &str) -> IResult<&str, Wrappers> {
    wrapper_parser_generator(W::Volume.tag(), dynamics, move |o| {
        Wrappers::Volume(o, vec![])
    })(i)
}

fn volume_fader(i: &str) -> IResult<&str, Wrappers> {
    wrapper_parser_generator(
        W::VolumeFader.tag(),
        move |i| separated_pair(separated_pair(dynamics, junk, dynamics), junk, bool_digit)(i),
        move |((from, to), inside)| {
            Wrappers::VolumeFader(VolumeFadeConfig::new(from, to, inside), vec![])
        },
    )(i)
}

#[cfg(test)]
mod base_tests {
    use crate::parsers::bool_digit;

    use super::{comment, dynamics, hex_or_dec, junk, space, Dynamics};
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
        assert_eq!(Ok(("", (12, false))), hex_or_dec("'12"));
        assert_eq!(Ok(("", (18, true))), hex_or_dec("12"));
    }
    #[test]
    fn dynamics_parser() {
        assert_eq!(Ok(("", Dynamics::Forte)), dynamics("f"));
        assert_eq!(Ok(("", Dynamics::Fortissimo)), dynamics("ff"));
        assert_eq!(Ok(("", Dynamics::Pianississimo)), dynamics("ppp"));
        assert_eq!(Ok(("", Dynamics::MezzoForte)), dynamics("mf"));
    }
    #[test]
    fn bool_digit_parser() {
        assert_eq!(Ok(("", true)), bool_digit("1"));
        assert_eq!(Ok(("", false)), bool_digit("0"));
    }
}

#[cfg(test)]
mod wrapper_tests {
    use super::*;
    #[test]
    fn length_parser() {
        let parser = length;
        assert_eq!(Ok(("", Wrappers::Length(1, vec![]))), parser("l 1"));
    }
    #[test]
    fn song_parser() {
        let parser = song;
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
        let parser = accel;
        assert_eq!(
            Ok(("", Wrappers::Accel(AccelConfig::new(12, 16), vec![]))),
            parser("t 12 16")
        );
    }
    #[test]
    fn glissando_parser() {
        let parser = glissando;
        assert_eq!(
            Ok((
                "",
                Wrappers::Glissando(
                    Repartition::new((Time::Static(Duration::from_millis(10)), Time::Dynamic(0))),
                    vec![]
                )
            )),
            parser("G '10 0")
        );
    }
    #[test]
    fn vibrato_parser() {
        let parser = vibrato;
        assert_eq!(
            Ok((
                "",
                Wrappers::Vibrato(
                    Repartition::new(rephelp2(((10, false), (1, true)))),
                    VibratoConfig::new(1, 1),
                    vec![],
                )
            )),
            parser("V 1 1 '10 1")
        )
    }
    #[test]
    fn volume_parser() {
        let parser = volume;
        assert_eq!(
            Ok(("", Wrappers::Volume(Dynamics::Fortississimo, vec![]))),
            parser("v fff")
        );
    }
    #[test]
    fn volume_fader_parser(){
        let parser=volume_fader;
        assert_eq!(
            Ok(("",Wrappers::VolumeFader(VolumeFadeConfig::new(Dynamics::Forte, Dynamics::Piano, false), vec![]))),
            parser("F f p 0")
        );
    }
}
