use std::{
    io::{Read, Seek, Write},
};

use crate::{
    buttons::{self, ButtonState},
    hid,
};

#[derive(Debug, serde::Deserialize, serde::Serialize,Clone)]
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

pub fn export<W: Write>(writer: &mut W, chords: &[Chord]) -> Result<(), Box<dyn std::error::Error>> {
    let mut wtr = csv::Writer::from_writer(writer);
    for chord in chords {
        wtr.serialize(chord)?;
    }
    wtr.flush()?;
    Ok(())
}

impl Into<ButtonState> for Chord {
    fn into(self) -> ButtonState {
        let thumbs = self.thumbs.unwrap_or_default();
        let fingers = self.fingers.unwrap_or_default();
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
                    tag_start = i;
                    hid_pairs.push((current_modifiers, 0x64));
                }
                ('>', true) => {
                    reading_tag = false;
                    let tag_contents = &self.output[tag_start + 1..i];

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
    use std::io::Cursor;

    #[test]
    fn test_parse() {
        let data = "Thumbs,Fingers,Keyboard Output\n<Thumb1>,<Thumb2>,<L-Ctrl>F";
        let mut cursor = Cursor::new(data);
        let chords = parse(&mut cursor).unwrap();



        assert_eq!(chords.len(), 1);
        assert_eq!(chords[0].output, "<L-Ctrl>F");
    }

    #[test]
    fn test_export() {
        let chords = vec![
            Chord {
                thumbs: Some("T1".to_string()),
                fingers: Some("F1".to_string()),
                output: "<L-Ctrl>F".to_string(),
            },
            Chord {
                thumbs: Some("T2".to_string()),
                fingers: Some("F2".to_string()),
                output: "<R-Shift>A".to_string(),
            },
        ];

        let mut buffer = Vec::new();
        export(&mut buffer, &chords).unwrap();
        let result = String::from_utf8(buffer).unwrap();



        assert!(result.contains("T1,F1,<L-Ctrl>F"));
        assert!(result.contains("T2,F2,<R-Shift>A"));
    }
}


