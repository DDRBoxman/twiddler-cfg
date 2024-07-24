use std::{
    fs::File,
    io::{Seek, SeekFrom, Write},
};

use binrw::{binrw, BinRead, PosValue};
use modular_bitfield::prelude::*;

use crate::buttons::ButtonState;

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
    chord: ButtonData,
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

#[bitfield]
#[derive(BinRead, Debug)]
#[br(map = Self::from_bytes)]
pub struct ButtonData {
    num: bool,
    a: bool,
    e: bool,
    sp: bool,
    alt: bool,
    b: bool,
    f: bool,
    del: bool,
    ctrl: bool,
    c: bool,
    g: bool,
    bs: bool,
    shift: bool,
    d: bool,
    h: bool,
    ent: bool,
}

impl Into<ButtonState> for ButtonData {
    fn into(self) -> ButtonState {
        ButtonState {
            T1: self.num(),
            T2: self.alt(),
            T3: self.ctrl(),
            T4: self.shift(),
            F0L: false,
            F0M: false,
            F0R: false,
            F1R: self.a(),
            F1M: self.e(),
            F1L: self.sp(),
            F2R: self.b(),
            F2M: self.f(),
            F2L: self.del(),
            F3R: self.c(),
            F3M: self.g(),
            F3L: self.bs(),
            F4R: self.d(),
            F4M: self.h(),
            F4L: self.ent(),
        }
    }
}

pub(crate) fn parse() -> std::io::Result<()> {
    let file = File::open("./configs/backspice2_v5.cfg")?;

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
