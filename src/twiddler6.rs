use std::{
    default,
    io::{Read, Seek, SeekFrom, Write},
};

use binrw::{binrw, BinRead, BinResult, BinWrite, Endian};
use modular_bitfield::{bitfield, prelude::B4};
use std::convert::From;

use crate::{buttons::ButtonState, hid};

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

#[bitfield]
#[derive(BinRead, BinWrite, Debug, Copy, Clone, Default)]
#[br(map = Self::from_bytes)]
pub struct ConfigFlags {
    repeat_delay_enable: bool,
    haptic: bool,
    left_mouse_pos: bool, // FOL or FOR
    direct: bool,
    sticky_num: bool,
    sticky_alt: bool,
    sticky_ctrl: bool,
    sticky_shift: bool,
}

#[binrw]
#[brw(little)]
#[derive(Debug)]
pub struct Config {
    #[brw(pad_before = 0x4)]
    version: u8,
    flags: ConfigFlags,
    pub number_of_chords: u16,
    #[brw(pad_before = 0x4)]
    pub idle_time: u16,
    pub mouse_sensitivity: u8,
    pub key_repeat_delay: u8,
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
            flags: ConfigFlags::default()
                .with_haptic(true)
                .with_repeat_delay_enable(true),
            number_of_chords: 0,
            chords: vec![],
            command_lists: vec![],
            idle_time: 600,
            mouse_sensitivity: 0x7f,
            key_repeat_delay: 127,
        }
    }
}

#[derive(Debug, Clone)]
#[binrw]
#[brw(little)]
pub struct Chord {
    #[brw(pad_after = 1)]
    pub buttons: ButtonData,
    pub command: Command,
}

#[derive(Debug, Clone)]
#[binrw]
#[brw(little)]
pub struct Command {
    pub command_type: CommandType,
    #[br(args { command_type: &command_type })]
    pub data: CommandData,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

pub(crate) fn write<W: Write + Seek>(
    mut config: Config,
    writer: &mut W,
    gen_caps: Option<i32>,
) -> std::io::Result<()> {
    // Generate chords for caps
    if let Some(caps) = gen_caps {
        let mut new_chords = vec![];
        for chord in &config.chords {
            if chord.command.command_type == CommandType::Keyboard
                && chord.buttons.t0() == false
                && chord.buttons.t1() == false
                && chord.buttons.t2() == false
                && chord.buttons.t3() == false
                && chord.buttons.t4() == false
            {
                if let CommandData::Keyboard(hid_command, _) = &chord.command.data {
                    if hid::ALPHA_HID_CODES.contains(&hid_command.key_code) {
                        let mut chord = chord.clone();

                        match caps {
                            1 => chord.buttons.set_t1(true),
                            2 => chord.buttons.set_t2(true),
                            3 => chord.buttons.set_t3(true),
                            4 => chord.buttons.set_t4(true),
                            _ => {}
                        }

                        chord.command.data = CommandData::Keyboard(
                            HidCommand {
                                key_code: hid_command.key_code,
                                modifier: hid_command.modifier | 0x2, //  Add left shift
                            },
                            0,
                        );

                        new_chords.push(chord);
                    }
                }
            }
        }

        if new_chords.len() > 0 {
            println!("Adding {} uppercase chords", new_chords.len());
            config.chords.append(&mut new_chords);
        }
    }

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
    writer.seek(SeekFrom::Start(0x13));
    let data = hex::decode("03000102030405060708090A0C0D0F111416181A1D").unwrap();
    writer.write(&data)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_header() {
        let mut file = std::fs::File::open("test/configs/v6/haptic_off.cfg").unwrap();
        let conf = Config::read(&mut file).unwrap();
        assert!(conf.flags.haptic() == false);

        let mut file = std::fs::File::open("test/configs/v6/sticky_alt.cfg").unwrap();
        let conf = Config::read(&mut file).unwrap();
        assert!(conf.flags.sticky_alt() == true);

        let mut file = std::fs::File::open("test/configs/v6/sticky_num.cfg").unwrap();
        let conf = Config::read(&mut file).unwrap();
        assert!(conf.flags.sticky_num() == true);

        let mut file = std::fs::File::open("test/configs/v6/sticky_shift.cfg").unwrap();
        let conf = Config::read(&mut file).unwrap();
        assert!(conf.flags.sticky_shift() == true);

        let mut file = std::fs::File::open("test/configs/v6/sticky_ctrl.cfg").unwrap();
        let conf = Config::read(&mut file).unwrap();
        assert!(conf.flags.sticky_ctrl() == true);

        let mut file = std::fs::File::open("test/configs/v6/key_repeat_delay_off.cfg").unwrap();
        let conf = Config::read(&mut file).unwrap();
        assert!(conf.flags.repeat_delay_enable() == false);

        let mut file = std::fs::File::open("test/configs/v6/left_mouse_pos.cfg").unwrap();
        let conf = Config::read(&mut file).unwrap();
        assert!(conf.flags.left_mouse_pos() == true);

        let mut file = std::fs::File::open("test/configs/v6/empty.cfg").unwrap();
        let conf = Config::read(&mut file).unwrap();
        assert!(conf.flags.left_mouse_pos() == false);
        assert!(conf.flags.repeat_delay_enable() == true);
        assert!(conf.flags.haptic() == false);
        assert!(conf.flags.sticky_alt() == false);
        assert!(conf.flags.sticky_num() == false);
        assert!(conf.flags.sticky_shift() == false);
        assert!(conf.flags.sticky_ctrl() == false);
        assert!(conf.flags.repeat_delay_enable() == true);
    }
}
