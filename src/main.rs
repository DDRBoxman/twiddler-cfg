use twiddler6::HidCommand;

mod buttons;
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

    let res = twiddler5::parse(&mut opt.input);
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

            twiddler6::write(config6, &mut opt.output)?;
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }

    Ok(())
}
