use buttons::ButtonState;

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

            let config6 = twiddler6::Config::new();
            config.chords.iter().for_each(|c| {
                let buttonState = c.button_state();

                let res = match c.mapping {
                    twiddler5::ChordMapping::KeyMapping(modifier, key_code) => twiddler6::Command {
                        command_type: twiddler6::CommandType::Keyboard,
                        data: twiddler6::CommandData::Keyboard(modifier, key_code),
                    },
                    twiddler5::ChordMapping::StringMapping(_, _) => twiddler6::Command {
                        command_type: twiddler6::CommandType::ListOfCommands,
                        data: twiddler6::CommandData::ListOfCommands(0),
                    },
                };
            });
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
