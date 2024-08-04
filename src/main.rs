#[macro_use]
extern crate simple_error;

use std::io::{Read, Seek, SeekFrom, Write};

use byteorder::ReadBytesExt;
use twiddler6::HidCommand;

mod buttons;
mod csv;
mod dido;
mod hid;
mod twiddler5;
mod twiddler6;
mod twiddler7;

use clap::{ArgAction, Parser};
use clio::*;

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

fn main() -> std::io::Result<()> {
    let mut opt = Opt::parse();

    //opt.input.seek(SeekFrom::Start(0));
    if opt.input.read_u8().unwrap() == 0x05 {
        println!("Reading input as Twiddler 5 config");
        opt.input.seek(SeekFrom::Start(0));
        twiddler5_to_twiddler6(&mut opt.input, &mut opt.output, opt.generate_caps);
        return Ok(());
    }

    opt.input.seek(SeekFrom::Start(4));
    if opt.input.read_u8().unwrap() == 0x06 {
        println!("Twiddler 6 config detected");
        return Ok(());
    }

    opt.input.seek(SeekFrom::Start(4));
    if opt.input.read_u8().unwrap() == 0x07 {
        println!("Twiddler 7 config detected");
        opt.input.seek(SeekFrom::Start(0));
        let conf = twiddler7::parse(&mut opt.input).unwrap();
        twiddler7::write(conf, &mut opt.output, None)?;

        return Ok(());
    }

    opt.input.seek(SeekFrom::Start(0));
    if opt.input.read_u8().unwrap() == '#' as u8 {
        println!("Starts with a #, assuming Dido config");
        dido_to_twiddler7(&mut opt.input, &mut opt.output, opt.generate_caps);
        return Ok(());
    }

    println!("Reading input as csv config");
    opt.input.seek(SeekFrom::Start(0));
    csv_to_twiddler6(&mut opt.input, &mut opt.output, opt.generate_caps);

    Ok(())
}

fn dido_to_twiddler7<R: Read + Seek, W: Write + Seek>(
    reader: &mut R,
    writer: &mut W,
    gen_caps: Option<i32>,
) -> std::io::Result<()> {
    let res = dido::parse(reader);

    match res {
        Ok(config) => {
            let mut config7 = twiddler7::Config::new();
            config.chords.iter().for_each(|c| {
                let command = match &c.output {
                    dido::ChordOutput::HidCode(key_code) => {
                        let key_code = key_code.parse().unwrap();
                        twiddler7::Command {
                            command_type: twiddler7::CommandType::Keyboard,
                            data: twiddler7::CommandData::Keyboard(
                                twiddler7::HidCommand {
                                    modifier: c.modifiers,
                                    key_code,
                                },
                                0,
                            ),
                        }
                    }
                    dido::ChordOutput::StringIndex(index) => {
                        let index = index.parse::<usize>().unwrap();
                        let command = twiddler7::Command {
                            command_type: twiddler7::CommandType::ListOfCommands,
                            data: twiddler7::CommandData::ListOfCommands(0, 0),
                        };

                        let out_string_hids = &config.strings[index];

                        let mut command_list = vec![];

                        for hids in out_string_hids {
                            command_list.push(twiddler7::Command {
                                command_type: twiddler7::CommandType::Keyboard,
                                data: twiddler7::CommandData::Keyboard(
                                    twiddler7::HidCommand {
                                        key_code: hids.0,
                                        modifier: hids.1,
                                    },
                                    0,
                                ),
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

            twiddler7::write(config7, writer, gen_caps)?;
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }

    Ok(())
}

fn csv_to_twiddler6<R: Read + Seek, W: Write + Seek>(
    reader: &mut R,
    writer: &mut W,
    gen_caps: Option<i32>,
) -> std::io::Result<()> {
    let res = csv::parse(reader);
    match res {
        Ok(_) => {
            println!("Parsed CSV");
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }

    Ok(())
}

fn twiddler5_to_twiddler6<R: Read + Seek, W: Write + Seek>(
    reader: &mut R,
    writer: &mut W,
    gen_caps: Option<i32>,
) -> std::io::Result<()> {
    let res = twiddler5::parse(reader);
    match res {
        Ok(config) => {
            let mut config6 = twiddler6::Config::new();
            config.chords.iter().for_each(|c| {
                let button_state = c.button_state();

                let command = match c.mapping {
                    twiddler5::ChordMapping::KeyMapping(modifier, key_code) => twiddler6::Command {
                        command_type: twiddler6::CommandType::Keyboard,
                        data: twiddler6::CommandData::Keyboard(
                            HidCommand { modifier, key_code },
                            0,
                        ),
                    },
                    twiddler5::ChordMapping::StringMapping(_, index) => {
                        let command = twiddler6::Command {
                            command_type: twiddler6::CommandType::ListOfCommands,
                            data: twiddler6::CommandData::ListOfCommands(0, 0),
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
                                    command_list.push(twiddler6::Command {
                                        command_type: twiddler6::CommandType::Keyboard,
                                        data: twiddler6::CommandData::Keyboard(
                                            HidCommand {
                                                modifier: *modifier,
                                                key_code: *key_code,
                                            },
                                            0,
                                        ),
                                    });
                                }
                                _ => {}
                            }
                        }

                        config6
                            .command_lists
                            .push(twiddler6::CommandList(command_list));

                        command
                    }
                };

                config6.chords.push(twiddler6::Chord {
                    buttons: button_state.into(),
                    command,
                });
            });

            twiddler6::write(config6, writer, gen_caps)?;
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }

    Ok(())
}
