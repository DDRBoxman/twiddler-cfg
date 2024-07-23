use std::{
    fs::File,
    io::{Seek, SeekFrom, Write},
};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use keycode::{KeyMap, KeyMapping, KeyMappingCode, KeyMappingId};

#[derive(Debug)]
#[repr(u8)]
pub enum CommandType {
    System = 1,
    Keyboard = 2,
    Mouse = 3,
    Delay = 5,
    ListOfCommands = 7,
}

impl CommandType {
    fn from_u8(value: u8) -> CommandType {
        match value {
            1 => CommandType::System,
            2 => CommandType::Keyboard,
            3 => CommandType::Mouse,
            5 => CommandType::Delay,
            7 => CommandType::ListOfCommands,
            _ => panic!("Unknown value: {}", value),
        }
    }
}

#[derive(Debug)]
pub struct Config {
    header: TwiddlerHeader,
    chords: Vec<Chord>,
}

#[derive(Debug)]
pub struct TwiddlerHeader {
    version: u8,
    left_mouse: u8,
    number_of_chords: u8,
}

#[derive(Debug)]
pub struct Command {
    command_type: CommandType,
    data: u16,
}

#[derive(Debug)]
pub struct Chord {
    keys: u32,
    command: Command,
    command_list: Option<Vec<Command>>,
}

pub(crate) fn parse() -> std::io::Result<Config> {
    let mut file = File::open("test.cfg")?;

    file.seek(SeekFrom::Start(4))?;

    let version = file.read_u8()?;
    let left_mouse = file.read_u8()?;
    let number_of_chords = file.read_u8()?;
    file.read_u8()?;

    let header = TwiddlerHeader {
        version,
        left_mouse,
        number_of_chords,
    };

    file.seek(SeekFrom::Start(0x28))?;

    let mut chords: Vec<Chord> = Vec::new();

    for _ in 0..number_of_chords {
        let keys = file.read_u32::<LittleEndian>()?;
        let command_type = file.read_u8()?;
        let data = file.read_u16::<LittleEndian>()?;
        file.read_u8()?;

        print!(
            "Keys: {:X}, Command Type: {:X}, Data: {:X}\n",
            keys, command_type, data
        );

        let command = Command {
            command_type: CommandType::from_u8(command_type),
            data,
        };

        let chord = Chord {
            keys,
            command,
            command_list: None,
        };

        chords.push(chord);
    }

    Ok(Config { header, chords })
}

pub(crate) fn write(config: Config) -> std::io::Result<()> {
    let mut file = File::create("test_out.cfg").unwrap();

    file.write_u32::<LittleEndian>(0x00000000)?;
    file.write_u8(config.header.version)?;
    file.write_u8(config.header.left_mouse)?;
    file.write_u8(config.header.number_of_chords)?;
    file.write_u8(0)?;

    // TODO: Figure out more config format details
    let data =
        hex::decode("58020000000000007F640003000102030405060708090A0C0D0F111416181A1D").unwrap();

    file.write(&data)?;

    let chord_list_end = 0x28 + (config.header.number_of_chords as u64 * 8);

    let command_list_offset = 0;

    for chord in config.chords {
        file.write_u32::<LittleEndian>(chord.keys)?;
        file.write_u8(chord.command.command_type as u8)?;
        file.write_u16::<LittleEndian>(chord.command.data)?;
        file.write_u8(0)?;
    }

    //let a = KeyMap::from(KeyMappingId::UsA);
    //KeyMap::from(KeyMappingCode::Win(22));

    // assert_eq!(a.usb, 0x0004);

    Ok(())
}
