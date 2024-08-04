#[macro_use]
extern crate simple_error;

use std::io::{Read, Seek, SeekFrom, Write};

use byteorder::ReadBytesExt;

mod buttons;
mod csv;
mod dido;
mod hid;
mod twiddler5;
mod twiddler6;
mod twiddler7;

use clap::{ArgAction, Parser};
use clio::*;
use serde::de::Error;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Opt {
    #[clap(value_parser)]
    input: Input,

    #[clap(value_parser)]
    output: Output,

    /// Generate upper case versions of chords with shift,
    /// 1 2 3 or 4 for the thumb key that should act as shift
    #[clap(long, short)]
    generate_caps: Option<i32>,
}

fn main() {
    let mut opt = Opt::parse();

    match load_config(&mut opt.input) {
        Ok(config) => {
            let res = twiddler7::write(config, &mut opt.output, opt.generate_caps);
            match res {
                Ok(_) => {
                    println!("Done");
                }
                Err(e) => {
                    println!("Failed to write output config{:?}", e);
                }
            }
        }
        Err(e) => {
            println!("Failed to load input config{:?}", e);
        }
    }
}

fn load_config<R: Read + Seek>(
    reader: &mut R,
) -> std::result::Result<twiddler7::Config, Box<dyn std::error::Error>> {
    reader.seek(SeekFrom::Start(0));
    if reader.read_u8().unwrap() == 0x05 {
        println!("Reading input as Twiddler 5 config");
        reader.seek(SeekFrom::Start(0));
        let config = twiddler5::parse(reader)?;
        return Ok(twiddler5_to_twiddler7(&config));
    }

    reader.seek(SeekFrom::Start(4));
    if reader.read_u8().unwrap() == 0x06 {
        println!("Twiddler 6 config detected");
        bail!("Twiddler 6 config not supported yet");
    }

    reader.seek(SeekFrom::Start(4));
    if reader.read_u8().unwrap() == 0x07 {
        println!("Twiddler 7 config detected");
        println!("Running through twiddler 7 parser to ensure it's valid");
        reader.seek(SeekFrom::Start(0));
        let conf = twiddler7::parse(reader)?;
        return Ok(conf);
    }

    reader.seek(SeekFrom::Start(0));
    if reader.read_u8().unwrap() == '#' as u8 {
        println!("Starts with a #, assuming Dido config");
        let res = dido::parse(reader);
        match res {
            Ok(config) => {
                let config7 = dido_to_twiddler7(config);
                return Ok(config7);
            }
            Err(e) => {
                bail!("Failed parsing dido config {:?}", e);
            }
        }
    }

    println!("Reading input as csv config");
    reader.seek(SeekFrom::Start(0));
    let res = csv::parse(reader);
    bail!("CSV conversion implementation incomplete");
}

fn dido_to_twiddler7(config: dido::Config) -> twiddler7::Config {
    let mut config7 = twiddler7::Config::new();
    config.chords.iter().for_each(|c| {
        let command = match &c.output {
            dido::ChordOutput::HidCode(key_code) => {
                let key_code = key_code.parse().unwrap();
                twiddler7::Command {
                    command_type: twiddler7::CommandType::Keyboard,
                    data: twiddler7::CommandData::Keyboard(twiddler7::HidCommand {
                        modifier: c.modifiers,
                        key_code,
                    }),
                }
            }
            dido::ChordOutput::StringIndex(index) => {
                let index = index.parse::<usize>().unwrap();
                let command = twiddler7::Command {
                    command_type: twiddler7::CommandType::ListOfCommands,
                    data: twiddler7::CommandData::ListOfCommands(0),
                };

                let out_string_hids = &config.strings[index];

                let mut command_list = vec![];

                for hids in out_string_hids {
                    command_list.push(twiddler7::Command {
                        command_type: twiddler7::CommandType::Keyboard,
                        data: twiddler7::CommandData::Keyboard(twiddler7::HidCommand {
                            key_code: hids.0,
                            modifier: hids.1,
                        }),
                    });
                }

                config7
                    .command_lists
                    .push(twiddler7::CommandList(command_list));

                command
            }
        };

        config7.chords.push(twiddler7::Chord {
            buttons: twiddler7::ButtonData::from(&c.buttons),
            command,
        });
    });

    config7
}

fn twiddler5_to_twiddler7(config: &twiddler5::Config) -> twiddler7::Config {
    let mut config7 = twiddler7::Config::new();
    config.chords.iter().for_each(|c| {
        let button_state = c.button_state();

        let command = match c.mapping {
            twiddler5::ChordMapping::KeyMapping(modifier, key_code) => twiddler7::Command {
                command_type: twiddler7::CommandType::Keyboard,
                data: twiddler7::CommandData::Keyboard(twiddler7::HidCommand {
                    modifier,
                    key_code,
                }),
            },
            twiddler5::ChordMapping::StringMapping(_, index) => {
                let command = twiddler7::Command {
                    command_type: twiddler7::CommandType::ListOfCommands,
                    data: twiddler7::CommandData::ListOfCommands(0),
                };

                let string_pos = config.string_locations[index as usize];
                let contents = config
                    .string_contents
                    .iter()
                    .find(|sc| sc.pos == string_pos.into())
                    .unwrap();

                let mut command_list = vec![];

                for c in &contents.keys {
                    match c {
                        twiddler5::ChordMapping::KeyMapping(modifier, key_code) => {
                            command_list.push(twiddler7::Command {
                                command_type: twiddler7::CommandType::Keyboard,
                                data: twiddler7::CommandData::Keyboard(twiddler7::HidCommand {
                                    modifier: *modifier,
                                    key_code: *key_code,
                                }),
                            });
                        }
                        _ => {}
                    }
                }

                config7
                    .command_lists
                    .push(twiddler7::CommandList(command_list));

                command
            }
        };

        config7.chords.push(twiddler7::Chord {
            buttons: button_state.into(),
            command,
        });
    });

    config7
}
