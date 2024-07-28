use std::{
    io::{Read, Seek},
    vec,
};

use crate::{
    buttons::{self, ButtonState},
    hid,
};

#[derive(Debug, serde::Deserialize)]
pub struct Chord {
    #[serde(alias = "Thumbs")]
    thumbs: Option<String>,
    #[serde(alias = " Fingers")] // Twiddler Tuner puts a space in the header name here lol
    #[serde(alias = "Fingers")]
    fingers: Option<String>,
    #[serde(alias = "Keyboard Output")]
    output: String,
}

pub fn parse<R: Read + Seek>(reader: &mut R) -> Result<Vec<Chord>, Box<dyn std::error::Error>> {
    let mut rdr = csv::Reader::from_reader(reader);

    let result: Result<Vec<Chord>, csv::Error> = rdr.deserialize().collect();
    match result {
        Ok(chords) => Ok(chords),
        Err(e) => Err(Box::new(e)),
    }
}

impl Into<ButtonState> for Chord {
    fn into(self) -> ButtonState {
        let thumbs = self.thumbs.unwrap();
        let fingers = self.fingers.unwrap();
        buttons::parse_notation(thumbs, fingers)
    }
}

impl Chord {
    pub fn get_hid_pairs(&self) -> Vec<(u8, u8)> {
        if self.output.len() == 1 {
            match hid::keys_hid().get_by_right(&self.output) {
                Some(key) => return vec![(0, *key)],
                None => return vec![(0, 0)],
            }
        }

        let mut hid_pairs: Vec<(u8, u8)> = Vec::new();

        let mut current_modifiers: u8 = 0;

        let mut reading_tag = false;
        let mut tag_start = 0;
        let mut closing = false;

        for (i, c) in self.output.chars().enumerate() {
            match (c, reading_tag) {
                ('<', false) => {
                    reading_tag = true;
                    tag_start = i;
                }
                ('<', true) => {
                    // Unexpected '<' in tag treat it as the user putting the last '<' as a key
                    // todo: this needs shift key to type properly
                    // update tag start to this position too
                    tag_start = i;
                    hid_pairs.push((current_modifiers, 0x64));
                }
                ('>', true) => {
                    reading_tag = false;
                    let tag_contents = &self.output[tag_start + 1..i];
                    println!("{}", tag_contents);

                    let modifier = match tag_contents {
                        "L-Ctrl" => 0x01,
                        "L-Shift" => 0x02,
                        "L-Alt" => 0x04,
                        "L-Gui" => 0x08,
                        "R-Ctrl" => 0x10,
                        "R-Shift" => 0x20,
                        "R-Alt" => 0x40,
                        "R-Gui" => 0x80,
                        _ => 0,
                    };

                    if closing {
                        current_modifiers &= !modifier;
                    } else {
                        current_modifiers |= modifier;
                    }
                }
                ('>', false) => {
                    // Unexpected '>' in tag treat it as the user putting '>' as a key
                    // todo: this needs shift key to type properly
                    hid_pairs.push((current_modifiers, 0x64));
                }
                ('/', true) => {
                    closing = false;
                }
                ('/', false) => hid_pairs.push((current_modifiers, 0x38)),
                (_, false) => match hid::keys_hid().get_by_right(&self.output) {
                    Some(key) => hid_pairs.push((current_modifiers, *key)),
                    None => {}
                },
                (_, true) => {}
            }
        }

        hid_pairs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        /*  parse_output("<L-Ctrl></L-Ctrl>");

        parse_output("<L-Ctrl>a</L-Ctrl>");

        parse_output("<L-Ctrl><HIDCode 0x04></L-Ctrl>");

        parse_output("<L-Ctrl>></L-Ctrl>");

        parse_output("<");*/
    }
}
