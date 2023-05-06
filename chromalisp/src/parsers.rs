use crate::structure::{AccelConfig, Repartition, Tagging, Time, Wrappers, W};
use nom::{
    branch::alt,
    bytes::complete::{is_not, take_till1},
    character::{
        complete::{char, digit1, multispace1, newline},
        streaming::hex_digit1,
    },
    combinator::{map_res, recognize, value},
    error::{dbg_dmp, ErrorKind, VerboseError},
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

fn hex_or_dec(i: &str) -> IResult<&str, (u8, bool)> {
    alt((
        map_res(
            preceded(char('\''), digit1),
            move |s: &str| -> Result<(u8, bool), ErrorKind> {
                Ok((s.parse::<u8>().unwrap(), false))
            },
        ),
        map_res(hex_digit1, move |o| -> Result<(u8, bool), ErrorKind> {
            Ok((u8::from_str_radix(o, 16).unwrap(), true))
        }),
    ))(i)
}

fn glissando_repartition_helper(o: u8, b: bool) -> Time {
    match b {
        true => Time::Dynamic(o),
        false => Time::Static(Duration::from_millis(o.into())),
    }
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

fn channel(i: &str) -> IResult<&str, Wrappers> {
    wrapper_parser_generator(W::Channel.tag(), arg_string, move |o| {
        Wrappers::Channel(o.to_string(), vec![])
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

fn length(i: &str) -> IResult<&str, Wrappers> {
    wrapper_parser_generator(W::Length.tag(), digit1, move |o| {
        Wrappers::Length(o.parse().unwrap(), vec![])
    })(i)
}

fn glissando(i: &str) -> IResult<&str, Wrappers> {
    wrapper_parser_generator(
        W::Glissando.tag(),
        move |i| separated_pair(hex_or_dec, junk, hex_or_dec)(i),
        move |(o1, o2)| {
            Wrappers::Glissando(
                Repartition::new(
                    glissando_repartition_helper(o1.0, o1.1),
                    glissando_repartition_helper(o2.0, o1.1),
                ),
                vec![],
            )
        },
    )(i)
}

// fn wrapper(w: W) -> impl Fn(&'static str) -> IResult<&'static str, Wrappers> {
//     wrapper_parser_generator(
//         w.tag(),
//         move |i| -> IResult<&str, Vec<&str>> {
//             match w {
//                 W::Song | W::Album | W::Artist | W::Channel => map_res(
//                     alt((
//                         delimited(char('\''), take_till1(|c| c == '\''), char('\'')),
//                         delimited(char('"'), take_till1(|c| c == '"'), char('"')),
//                         take_till1(|c| c == ' '),
//                     )),
//                     move |o| -> Result<Vec<&str>, ErrorKind> { Ok(vec![o]) },
//                 )(i),
//                 W::Accel => map_res(
//                     separated_pair(digit1, junk, digit1),
//                     move |(o1, o2)| -> Result<Vec<&str>, ErrorKind> { Ok(vec![o1, o2]) },
//                 )(i),
//                 W::NoteDef => todo!(),
//                 W::Instrument => todo!(),
//                 W::Length | W::Octave | W::Year | W::Tempo | W::Loop => {
//                     map_res(digit1, move |o| -> Result<Vec<&str>, ErrorKind> {
//                         Ok(vec![o])
//                     })(i)
//                 }
//                 W::Glissando => map_res(
//                     separated_pair(hex_or_dec, junk, hex_or_dec),
//                     move |(o1, o2)| -> Result<Vec<&str>, ErrorKind> { Ok(vec![o1, o2]) },
//                 )(i),
//                 W::Vibrato => todo!(),
//                 W::Volume => todo!(),
//                 W::VolumeFader => todo!(),
//                 W::ADSR => todo!(),
//                 W::Singleton => todo!(),
//                 W::Mask => todo!(),
//                 W::Masked => todo!(),
//                 W::Test => todo!(),
//                 W::Test2 => todo!(),
//             }
//         },
//         move |o| match w {
//             W::Song => Wrappers::Song(o.get(0).unwrap().to_string(), vec![]),
//             W::Album => Wrappers::Album(o.get(0).unwrap().to_string(), vec![]),
//             W::Artist => Wrappers::Artist(o.get(0).unwrap().to_string(), vec![]),
//             W::Year => Wrappers::Year(o.get(0).unwrap().parse().unwrap(), vec![]),
//             W::Tempo => Wrappers::Tempo(o.get(0).unwrap().parse().unwrap(), vec![]),
//             W::Accel => Wrappers::Accel(
//                 AccelConfig::new(
//                     o.get(0).unwrap().parse().unwrap(),
//                     o.get(1).unwrap().parse().unwrap(),
//                 ),
//                 vec![],
//             ),
//             W::NoteDef => todo!(),
//             W::Channel => Wrappers::Channel(o.get(0).unwrap().to_string(), vec![]),
//             W::Instrument => todo!(),
//             W::Length => Wrappers::Length(o.get(0).unwrap().parse().unwrap(), vec![]),
//             W::Octave => Wrappers::Octave(o.get(0).unwrap().parse().unwrap(), vec![]),
//             W::Loop => Wrappers::Loop(o.get(0).unwrap().parse().unwrap(), vec![]),
//             W::Glissando => Wrappers::Glissando(
//                 Repartition::new(
//                     {
//                         let result = hex_or_dec_resolve(o.get(0).unwrap());
//                         if result.1 {
//                             Time::Dynamic(result.0)
//                         } else {
//                             Time::Static(Duration::from_millis(result.0.into()))
//                         }
//                     },
//                     {
//                         let result = hex_or_dec_resolve(o.get(1).unwrap());
//                         if result.1 {
//                             Time::Dynamic(result.0)
//                         } else {
//                             Time::Static(Duration::from_millis(result.0.into()))
//                         }
//                     },
//                 ),
//                 vec![],
//             ),
//             W::Vibrato => todo!(),
//             W::Volume => todo!(),
//             W::VolumeFader => todo!(),
//             W::ADSR => todo!(),
//             W::Singleton => todo!(),
//             W::Mask => todo!(),
//             W::Masked => todo!(),
//             W::Test => todo!(),
//             W::Test2 => todo!(),
//         },
//     )
// }

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
        assert_eq!(Ok(("", (12, false))), hex_or_dec("'12"));
        assert_eq!(Ok(("", (12, true))), hex_or_dec("12"));
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
                    Repartition::new(Time::Static(Duration::from_millis(10)), Time::Dynamic(0)),
                    vec![]
                )
            )),
            parser("G '10 0")
        );
    }
}
