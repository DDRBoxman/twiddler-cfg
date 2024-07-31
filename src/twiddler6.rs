use std::io::{Read, Seek, SeekFrom, Write};

use binrw::{binrw, BinRead, BinResult, BinWrite, Endian};
use modular_bitfield::{bitfield, prelude::B4};
use std::convert::From;

use crate::buttons::ButtonState;

#[derive(Debug, Eq, PartialEq, Clone)]
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
    pub number_of_chords: u16,
    #[brw(seek_before = SeekFrom::Start(0x28))]
    #[br(count = number_of_chords)]
    pub chords: Vec<Chord>,

    #[br(count = chords.iter().filter(|c| c.command.command_type == CommandType::ListOfCommands).count())]
    pub command_lists: Vec<CommandList>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            version: 6,
            left_mouse: 1,
            number_of_chords: 0,
            chords: vec![],
            command_lists: vec![],
        }
    }
}

#[derive(Debug)]
#[binrw]
#[brw(little)]
pub struct Chord {
    #[brw(pad_after = 1)]
    pub buttons: ButtonData,
    pub command: Command,
}

#[derive(Debug)]
#[binrw]
#[brw(little)]
pub struct Command {
    pub command_type: CommandType,
    #[br(args { command_type: &command_type })]
    pub data: CommandData,
}

#[derive(Debug)]
#[binrw]
#[br(little)]
#[br(import { command_type: &CommandType })]
pub(crate) enum CommandData {
    #[br(assert(*command_type == CommandType::ListOfCommands))]
    ListOfCommands(u8, u16),
    #[br(assert(*command_type == CommandType::Keyboard))]
    Keyboard(HidCommand, u8),
    #[br(assert(*command_type == CommandType::System))]
    System(u8, u8, u8),
    #[br(assert(*command_type == CommandType::None))]
    None(u8, u8, u8),
}

#[derive(Debug)]
#[binrw]
pub struct HidCommand {
    pub modifier: u8,
    pub key_code: u8,
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

#[bitfield]
#[derive(BinRead, BinWrite, Debug, Copy, Clone)]
#[br(map = Self::from_bytes)]
pub struct ButtonData {
    t1: bool,
    f1r: bool,
    f1m: bool,
    f1l: bool,

    t2: bool,
    f2r: bool,
    f2m: bool,
    f2l: bool,

    t3: bool,
    f3r: bool,
    f3m: bool,
    f3l: bool,

    t4: bool,
    f4r: bool,
    f4m: bool,
    f4l: bool,

    t0: bool,
    f0r: bool,
    f0m: bool,
    f0l: bool,

    unknown: B4,
}

impl From<ButtonState> for ButtonData {
    fn from(state: ButtonState) -> Self {
        ButtonData::new()
            .with_f0l(state.f0l)
            .with_f0m(state.f0m)
            .with_f0r(state.f0r)
            .with_t0(false)
            .with_t3(state.t3)
            .with_f3r(state.f3r)
            .with_f3m(state.f3m)
            .with_f3l(state.f3l)
            .with_t4(state.t4)
            .with_f4r(state.f4r)
            .with_f4m(state.f4m)
            .with_f4l(state.f4l)
            .with_t1(state.t1)
            .with_f1r(state.f1r)
            .with_f1m(state.f1m)
            .with_f1l(state.f1l)
            .with_t2(state.t2)
            .with_f2r(state.f2r)
            .with_f2m(state.f2m)
            .with_f2l(state.f2l)
    }
}

impl From<&ButtonState> for ButtonData {
    fn from(state: &ButtonState) -> Self {
        ButtonData::new()
            .with_f0l(state.f0l)
            .with_f0m(state.f0m)
            .with_f0r(state.f0r)
            .with_t0(false)
            .with_t3(state.t3)
            .with_f3r(state.f3r)
            .with_f3m(state.f3m)
            .with_f3l(state.f3l)
            .with_t4(state.t4)
            .with_f4r(state.f4r)
            .with_f4m(state.f4m)
            .with_f4l(state.f4l)
            .with_t1(state.t1)
            .with_f1r(state.f1r)
            .with_f1m(state.f1m)
            .with_f1l(state.f1l)
            .with_t2(state.t2)
            .with_f2r(state.f2r)
            .with_f2m(state.f2m)
            .with_f2l(state.f2l)
    }
}

impl Into<ButtonState> for ButtonData {
    fn into(self) -> ButtonState {
        ButtonState {
            t1: self.t1(),
            t2: self.t2(),
            t3: self.t3(),
            t4: self.t4(),
            f0l: self.f0l(),
            f0m: self.f0m(),
            f0r: self.f0r(),
            f1r: self.f1r(),
            f1m: self.f1m(),
            f1l: self.f1l(),
            f2r: self.f2r(),
            f2m: self.f2m(),
            f2l: self.f2l(),
            f3r: self.f3r(),
            f3m: self.f3m(),
            f3l: self.f3l(),
            f4r: self.f4r(),
            f4m: self.f4m(),
            f4l: self.f4l(),
        }
    }
}

pub(crate) fn parse<R: Read + Seek>(reader: &mut R) -> Result<Config, Box<dyn std::error::Error>> {
    let res = Config::read(reader);
    match res {
        Ok(config) => {
            //println!("{:?}", config);
            return Ok(config);
        }
        Err(e) => Err(Box::new(e)),
    }
}

pub(crate) fn write<W: Write + Seek>(mut config: Config, writer: &mut W) -> std::io::Result<()> {
    // update number of chords
    config.number_of_chords = config.chords.len() as u16;

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

    let mut offset = 0;

    let mut j = 0;
    for i in 0..config.chords.len() {
        if config.chords[i].command.command_type == CommandType::ListOfCommands {
            let size = config.command_lists[j].0.len() * 4;
            config.chords[i].command.data = CommandData::ListOfCommands(0, offset);
            offset += size as u16;
            offset += 4; // 0u32
            j += 1;
        }
    }

    let res = Config::write(&config, writer);
    match res {
        Ok(_) => {
            println!("Wrote config");
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }

    // TODO: Figure out more config format details
    writer.seek(SeekFrom::Start(0x8));
    let data =
        hex::decode("58020000000000007F640003000102030405060708090A0C0D0F111416181A1D").unwrap();
    writer.write(&data)?;

    Ok(())
}
