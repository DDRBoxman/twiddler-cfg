
mod buttons;
mod twiddler5;
mod twiddler6;

fn main() -> std::io::Result<()> {
    let res = twiddler5::parse();
    match res {
        Ok(config) => {
            config.chords.iter().for_each(|c| {
                println!("{:?}", c);
            });

            let mut config6 = twiddler6::Config::new();
            config.chords.iter().for_each(|c| {
                let button_state = c.button_state();

                let command = match c.mapping {
                    twiddler5::ChordMapping::KeyMapping(modifier, key_code) => twiddler6::Command {
                        command_type: twiddler6::CommandType::Keyboard,
                        data: twiddler6::CommandData::Keyboard(modifier, key_code),
                    },
                    twiddler5::ChordMapping::StringMapping(_, _) => {
                       /*  let command = twiddler6::Command {
                            command_type: twiddler6::CommandType::ListOfCommands,
                            data: twiddler6::CommandData::ListOfCommands(0),
                        };

                        let command_list = vec![
                            twiddler6::Command {
                                command_type: twiddler6::CommandType::Keyboard,
                                data: twiddler6::CommandData::Keyboard(0, 0x0C),
                            },
                            twiddler6::Command {
                                command_type: twiddler6::CommandType::Keyboard,
                                data: twiddler6::CommandData::Keyboard(0, 0x0D),
                            },
                        ];

                        config6.command_lists.push(twiddler6::CommandList(command_list));
                        */

                        twiddler6::Command {
                            command_type: twiddler6::CommandType::Keyboard,
                            data: twiddler6::CommandData::Keyboard(0, 0x0C),
                        }
                    }
                };

                config6.chords.push(twiddler6::Chord {
                    buttons: button_state.into(),
                    command
                });
            });

            twiddler6::write(config6)?;
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }

    /*let res = twiddler6::parse();
    match res {
        Ok(config) => {
            twiddler6::write(config)?;
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }*/

    Ok(())
}
