use std::io::{BufRead, Read, Seek};

use nom::{
    branch::alt,
    bytes::streaming::{is_not, tag, take},
    character::{
        complete::{alphanumeric0, digit0},
        streaming::{alpha0, digit1, space0, space1},
    },
    combinator::{map, opt},
    error::VerboseError,
    sequence::{delimited, pair, preceded, separated_pair, tuple},
};

use crate::buttons::{self, ButtonState};

pub struct Config {
    pub chords: Vec<Chord>,
    pub strings: Vec<Vec<(u8, u8)>>,
}

#[derive(Debug)] // Add the Debug trait to the Chord struct
pub struct Chord {
    pub buttons: ButtonState,
    pub output: ChordOutput,
    pub modifiers: u8,
    pub comment: String,
}

#[derive(Debug, PartialEq, Eq)]
enum ParseState {
    Options,
    Settings,
    Header,
    Chords,
    Strings,
    Done,
}

pub(crate) fn parse<R: Read + Seek>(reader: &mut R) -> Result<Config, Box<dyn std::error::Error>> {
    let mut parse_state = ParseState::Options;

    let buffered = std::io::BufReader::new(reader);

    let mut lines = buffered.lines();

    let mut chords = vec![];
    let mut strings = vec![];

    while let Some(Ok(line)) = lines.next() {
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

        // Ignore comments unless we're parsing the string section
        if line.starts_with("#") && parse_state != ParseState::Strings {
            continue;
        }

        match parse_state {
            ParseState::Options => {
                // todo: parse options
                //parse_key_value(line);
            }
            ParseState::Settings => {
                // todo: parse settings
            }
            ParseState::Header => {
                let res = parse_key_value(line);
                match res {
                    Ok((key, value)) => match key.as_str() {
                        "mouse_left" => {
                            println!("mouse_left: {}", value);
                            if value == "false" {
                                lines.next();
                            }
                        }
                        "mouse_right" => {
                            println!("mouse_right: {}", value);
                            if value == "false" {
                                lines.next();
                            }
                        }
                        "mouse_mid" => {
                            println!("mouse_mid: {}", value);
                            if value == "false" {
                                lines.next();
                            }
                        }
                        _ => {}
                    },
                    Err(e) => {
                        println!("error: {:?}", e);
                    }
                }
            }
            ParseState::Chords => {
                if let Ok(chord) = parse_chord_line(line) {
                    chords.push(chord);
                }
            }
            ParseState::Strings => {
                let res = parse_string_index(&line);
                match res {
                    Ok((index, len)) => {
                        let mut hids = vec![];

                        for i in 0..len {
                            let line = lines.next().unwrap().unwrap();

                            let mut string_line_parser = pair(
                                digit1,
                                opt(preceded(
                                    nom::character::streaming::char('+'),
                                    alphanumeric0,
                                )),
                            );

                            assert!(matches!(
                                string_line_parser("asl;kfjsl;kfdj"),
                                Err(nom::Err::Error(VerboseError { .. }))
                            ));

                            let res = string_line_parser(&line);
                            match res {
                                Ok((_, (hid, out_mods))) => {
                                    let hid_u8 = hid.parse::<u8>().unwrap();
                                    let mod_u8 = parse_mod_out(out_mods.unwrap_or_default());

                                    hids.push((hid_u8, mod_u8));
                                }
                                Err(e) => {
                                    println!("error: {:?}", e);
                                }
                            }
                        }

                        strings.push(hids);
                    }
                    Err(e) => {
                        println!("error: {:?}", e);
                    }
                }
            }
            ParseState::Done => {
                // done
            }
        }
    }

    Ok(Config { chords, strings })
}

fn parse_mod_out(out: &str) -> u8 {
    out.chars()
        .zip(out.chars().skip(1))
        .fold(0, |acc, (a, b)| match (a, b) {
            ('L', 'C') => acc | 0x1,
            ('L', 'S') => acc | 0x2,
            ('L', 'A') => acc | 0x4,
            ('L', 'G') => acc | 0x8,
            ('R', 'C') => acc | 0x10,
            ('R', 'S') => acc | 0x20,
            ('R', 'A') => acc | 0x40,
            ('R', 'G') => acc | 0x80,
            _ => acc,
        })
}

fn parse_key_value(line: String) -> Result<(String, String), Box<dyn std::error::Error>> {
    let mut items = line.split("=");
    if let Some(key) = items.next() {
        if let Some(value) = items.next() {
            return Ok((key.trim().to_owned(), value.trim().to_owned()));
        }
    }

    bail!("Invalid key value pair: {}", line);
}

#[derive(Debug, Clone, PartialEq)]
pub enum ChordOutput {
    StringIndex(String),
    HidCode(String),
}

fn string_index(i: &str) -> ChordOutput {
    ChordOutput::StringIndex(i.to_string())
}

fn hid_code(i: &str) -> ChordOutput {
    ChordOutput::HidCode(i.to_string())
}

pub fn parse_chord_line(line: String) -> Result<Chord, Box<dyn std::error::Error>> {
    // NACS XXXX:HHH+LCLSLALGRCRSRARG:# comment

    let mut parser = nom::sequence::tuple((
        map(
            tuple((take(4u8), space1, take(4u8))),
            |(thumb, _, finger): (&str, _, &str)| {
                buttons::parse_notation(thumb.to_string(), finger.to_string())
            },
        ),
        nom::character::streaming::char(':'),
        alt((
            delimited(
                tag("String["),
                map(digit0, string_index),
                nom::character::streaming::char(']'),
            ),
            map(digit0, hid_code),
        )),
        opt(preceded(
            nom::character::streaming::char('+'),
            alphanumeric0,
        )),
        space0,
        nom::character::streaming::char(':'),
    ));

    assert!(matches!(
        parser("1.2alksdfjlksaflkasfj"),
        Err(nom::Err::Error(VerboseError { .. }))
    ));

    let result = parser(&line);

    match result {
        Ok((comment, (button_state, _, output, out_mods, _, _))) => {
            let mod_u8 = parse_mod_out(out_mods.unwrap_or_default());

            return Ok(Chord {
                buttons: button_state,
                output,
                modifiers: mod_u8,
                comment: comment.to_string(),
            });
        }
        Err(e) => {
            println!("error: {:?}", e);
        }
    }
    bail!("Invalid chord line: {}", line)
}

fn parse_string_index(input: &str) -> Result<(u32, usize), Box<dyn std::error::Error>> {
    // # String[5]="you "
    // # String[60]="650-489-5484"

    let mut parser = nom::sequence::tuple((
        nom::character::streaming::char('#'),
        space0,
        delimited(tag("String["), digit0, nom::character::streaming::char(']')),
        nom::character::streaming::char('='),
        delimited(tag("\""), is_not("\""), tag("\"")),
    ));

    assert!(matches!(
        parser("1.2"),
        Err(nom::Err::Error(VerboseError { .. }))
    ));

    let res = parser(input);
    match res {
        Ok((_, (_, _, index, _, characters))) => {
            if let Ok(index) = index.parse::<u32>() {
                return Ok((index, characters.len()));
            }
        }
        Err(e) => {
            println!("error: {:?}", e);
        }
    }

    bail!("Invalid string index: {}", input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<(), Box<dyn std::error::Error>> {
        let res = parse_chord_line("N    M000:034                 :# Keyboard 5 and %".to_owned())?;
        assert!(res.output == hid_code("034"));

        parse_chord_line("     LMR0:037+RS              :# Keyboard 8 and *".to_owned());
        parse_chord_line("     LMMM:String[4]:".to_owned());
        parse_chord_line("   S LL00:045+RS              :# Keyboard - and _".to_owned());

        let res = parse_string_index("# String[60]=\"650-489-5484\"")?;
        assert!(res.0 == 60);
        assert!(res.1 == 12);

        let res = parse_string_index("# String[5]=\"you \"")?;
        assert!(res.0 == 5);
        assert!(res.1 == 4);

        Ok(())
    }
}
