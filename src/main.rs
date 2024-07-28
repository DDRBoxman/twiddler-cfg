use std::io::{Read, Seek, SeekFrom, Write};

use binrw::BinReaderExt;
use byteorder::ReadBytesExt;
use twiddler6::HidCommand;

mod buttons;
mod csv;
mod hid;
mod twiddler5;
mod twiddler6;

use clap::Parser;
use clio::*;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Opt {
    #[clap(value_parser)]
    input: Input,

    #[clap(value_parser)]
    output: Output,
}

fn main() -> std::io::Result<()> {
    let mut opt = Opt::parse();

    //opt.input.seek(SeekFrom::Start(0));
    if opt.input.read_u8().unwrap() == 0x05 {
        println!("Reading input as Twiddler 5 config");
        opt.input.seek(SeekFrom::Start(0));
        twiddler5_to_twiddler6(&mut opt.input, &mut opt.output);
        return Ok(());
    }

    opt.input.seek(SeekFrom::Start(5));
    if opt.input.read_u8().unwrap() == 0x06 {
        print!("Twiddler 6 config detected");
        return Ok(());
    }

    println!("Reading input as csv config");
    opt.input.seek(SeekFrom::Start(0));
    csv_to_twiddler6(&mut opt.input, &mut opt.output);

    Ok(())
}

fn csv_to_twiddler6<R: Read + Seek, W: Write + Seek>(
    reader: &mut R,
    writer: &mut W,
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

            twiddler6::write(config6, writer)?;
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }

    Ok(())
}
