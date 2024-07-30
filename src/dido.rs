use std::io::{BufRead, Read, Seek};

use nom::{
    branch::alt,
    bytes::streaming::{tag, take},
    character::{
        complete::{alphanumeric0, digit0},
        streaming::{alpha0, alpha1, digit1, space0, space1},
    },
    combinator::{map, opt, value},
    error::{ErrorKind, VerboseError},
    number::{complete::be_u32, streaming::be_u16},
    sequence::{delimited, pair, preceded, separated_pair, terminated, tuple},
    Needed,
};

use crate::{buttons, twiddler5::Config};

enum ParseState {
    Options,
    Settings,
    Header,
    Chords,
    Strings,
    Done,
}

pub(crate) fn parse<R: Read + Seek>(reader: &mut R) -> Result<Chord, Box<dyn std::error::Error>> {
    let mut parse_state = ParseState::Options;

    let buffered = std::io::BufReader::new(reader);

    for line in buffered.lines() {
        match line {
            Ok(line) => {
                if line.contains("---") {
                    match line.as_str() {
                        "# --- end of options" => parse_state = ParseState::Settings,
                        "# --- end of settings" => parse_state = ParseState::Header,
                        "# --- end of header" => parse_state = ParseState::Chords,
                        "# --- end of chords" => parse_state = ParseState::Strings,
                        "# --- end of strings" => parse_state = ParseState::Done,
                        _ => {}
                    }

                    continue;
                }

                if line.starts_with("#") {
                    continue;
                }

                match parse_state {
                    ParseState::Options => {
                        // todo: parse options
                    }
                    ParseState::Settings => {
                        // todo: parse settings
                    }
                    ParseState::Header => {}
                    ParseState::Chords => {
                        let chord = parse_chord_line(line);
                    }
                    ParseState::Strings => {
                        // parse strings
                    }
                    ParseState::Done => {
                        // done
                    }
                }
            }
            Err(e) => {
                println!("error: {:?}", e);
            }
        }
    }

    Ok(Chord {
        hid_modifier: "N".to_owned(),
        buttons: "M000".to_owned(),
        keycode_dec: "034".to_owned(),
        modifiers: "".to_owned(),
        comment: "Keyboard 5 and %".to_owned(),
    })
}

struct Chord {
    hid_modifier: String,
    buttons: String,
    keycode_dec: String,
    modifiers: String,
    comment: String,
}

#[derive(Debug)]
enum ChordOutput<'a> {
    StringIndex(&'a str),
    HidCode(&'a str),
}

pub fn parse_chord_line(line: String) {
    // NACS XXXX:HHH+LCLSLALGRCRSRARG:# comment

    let mut parser = nom::sequence::tuple((
        map(
            separated_pair(alpha0, space1, alphanumeric0),
            |(thumb, finger): (&str, &str)| {
                (buttons::parse_notation(
                    thumb.to_string(),
                    finger.to_string(),
                ),)
            },
        ),
        nom::character::streaming::char(':'),
        alt((
            delimited(
                tag("String["),
                map(digit0, ChordOutput::StringIndex),
                nom::character::streaming::char(']'),
            ),
            map(digit0, ChordOutput::HidCode),
        )),
        opt(preceded(
            nom::character::streaming::char('+'),
            alphanumeric0,
        )),
        space0,
        nom::character::streaming::char(':'),
    ));

    assert!(matches!(
        parser("1.2"),
        Err(nom::Err::Error(VerboseError { .. }))
    ));

    let result = parser(line.as_str());

    match result {
        Ok((comment, (button_state, _, hid, out_mods, _, _))) => {
            println!("button_state: {:?}", button_state);
            println!("hid: {:?}", hid);
            println!("out_mods: {:?}", out_mods);
            println!("comment: {:?}", comment);
        }
        Err(e) => {
            println!("error: {:?}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        parse_chord_line("N    M000:034                 :# Keyboard 5 and %".to_owned());
        parse_chord_line("     LMR0:037+RS              :# Keyboard 8 and *".to_owned());
        parse_chord_line("     LMMM:String[4]:".to_owned());
    }
}
