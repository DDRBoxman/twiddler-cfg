use std::{
    fs::File,
    io::{Seek, SeekFrom, Write},
};

use binrw::{binrw, BinRead, PosValue};
use itertools::Itertools;
use keycode::KeyMapping;

#[derive(BinRead)]
#[br(little)]
#[derive(Debug)]
struct Config {
    version: u8,
    options_a: u8,
    number_of_chords: u16,
    sleep_timeout: u16,
    mouse_left_click: u16,
    mouse_middle_click: u16,
    mouse_right_click: u16,
    mouse_accel_factor: u8,
    key_repeat_delay: u8,
    options_b: u8,
    options_c: u8,

    #[br(count = number_of_chords)]
    chords: Vec<Chord>,

    #[br(calc = chords.iter().filter(|c| c.mapping.is_string()).count())]
    number_of_strings: usize,

    #[br(count = number_of_strings)]
    string_locations: Vec<u32>,

    #[br(count = number_of_strings)]
    string_contents: Vec<PosValue<StringContents>>,
}

#[derive(BinRead)]
#[br(little)]
#[derive(Debug)]
struct Chord {
    chord: u16,
    #[br(restore_position)]
    modifier: u8,
    #[br(args { modifier })]
    mapping: ChordMapping,
}

#[derive(Eq, PartialEq, Hash, Debug, BinRead)]
#[br(little)]
#[br(import { modifier: u8 })]
enum ChordMapping {
    #[br(assert(modifier == 0xFF))]
    StringMapping(u8, u8),
    KeyMapping(u8, u8),
}

impl ChordMapping {
    fn is_string(&self) -> bool {
        matches!(self, ChordMapping::StringMapping(_, _))
    }
}

#[derive(Debug, BinRead)]
#[br(little)]
struct StringContents {
    size: u16,

    #[br(count = size / 2 - 1, args { inner: ChordMappingBinReadArgs { modifier: 0 } })]
    keys: Vec<ChordMapping>,
}

pub(crate) fn parse() -> std::io::Result<()> {
    let mut file = File::open("./configs/backspice2_v5.cfg")?;

    let res = Config::read(&mut &file);
    match res {
        Ok(config) => {
            println!("{:?}", config);
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }

    Ok(())
}
