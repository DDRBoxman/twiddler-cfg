use std::{
    fs::File,
    io::{Read, Seek, SeekFrom, Write},
};

use binrw::{binrw, BinRead, BinResult, BinWrite, Endian, PosValue};

#[derive(Debug, PartialEq)]
#[binrw]
#[brw(big, repr = u8)]
pub enum CommandType {
    None = 0,
    System = 1,
    Keyboard = 2,
    Mouse = 3,
    Delay = 5,
    ListOfCommands = 7,
}

#[binrw]
#[brw(little)]
#[derive(Debug)]
pub struct Config {
    #[brw(pad_before = 0x4)]
    version: u8,
    left_mouse: u8,
    number_of_chords: u8,
    #[brw(seek_before = SeekFrom::Start(0x28))]
    #[br(count = number_of_chords)]
    chords: Vec<Chord>,

    #[br(count = chords.iter().filter(|c| c.command.command_type == CommandType::ListOfCommands).count())]
    command_lists: Vec<CommandList>,
}

#[derive(Debug)]
#[binrw]
#[brw(little)]
pub struct Chord {
    keys: u32,
    command: Command,
}

#[derive(Debug)]
#[binrw]
#[brw(little)]
pub struct Command {
    command_type: CommandType,
    #[brw(pad_after = 1)]
    data: u16,
}

#[derive(Default, Debug)]
pub struct CommandList(pub Vec<Command>);

impl BinRead for CommandList {
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        endian: Endian,
        (): Self::Args<'_>,
    ) -> BinResult<Self> {
        let mut values = vec![];

        loop {
            let command = <Command>::read_options(reader, endian, ())?;
            if command.command_type == CommandType::None {
                return Ok(Self(values));
            }
            values.push(command);
        }
    }
}

impl BinWrite for CommandList {
    type Args<'a> = ();

    fn write_options<W: Write + Seek>(
        &self,
        writer: &mut W,
        endian: Endian,
        args: Self::Args<'_>,
    ) -> BinResult<()> {
        self.0.write_options(writer, endian, args)?;
        0u32.write_options(writer, endian, args)?;

        Ok(())
    }
}

pub(crate) fn parse() -> Result<Config, Box<dyn std::error::Error>> {
    let mut file = File::open("./configs/default_v6.cfg")?;

    let res = Config::read(&mut &file);
    match res {
        Ok(config) => {
            println!("{:?}", config);
            return Ok(config);
        }
        Err(e) => Err(Box::new(e)),
    }
}

pub(crate) fn write(mut config: Config) -> std::io::Result<()> {
    // update offsets in config
    let command_lists_command_count = config
        .chords
        .iter()
        .filter(|c| c.command.command_type == CommandType::ListOfCommands)
        .count();
    assert!(
        command_lists_command_count == config.command_lists.len(),
        "Commands with CommandType::ListOfCommands count mismatch"
    );

    let chord_section_size = config.chords.len() * 8;

    let mut offset = 0x28 + chord_section_size;

    for i in 0..command_lists_command_count {
        config.chords[i].command.data = offset as u16;
        offset += config.command_lists[i].0.len() * 4;
        offset += 4;
    }

    let mut file = File::create("test_out.cfg").unwrap();

    let res = Config::write(&config, &mut file);
    match res {
        Ok(_) => {
            println!("Wrote config");
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }

    // TODO: Figure out more config format details
    file.seek(SeekFrom::Start(0x8));
    let data =
        hex::decode("58020000000000007F640003000102030405060708090A0C0D0F111416181A1D").unwrap();
    file.write(&data)?;

    Ok(())
}
