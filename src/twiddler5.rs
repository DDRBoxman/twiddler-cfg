use std::io::{Read, Seek};

use binrw::{BinRead, PosValue};
use modular_bitfield::prelude::*;

use crate::buttons::ButtonState;

#[derive(BinRead)]
#[br(little)]
#[derive(Debug)]
pub struct Config {
    version: u8,
    options_a: u8,
    number_of_chords: u16,
    sleep_timeout: u16,
    mouse_left_click: u16,   // todo: these can also be actions and probably
    mouse_middle_click: u16, // invoke string actions, make sure to
    mouse_right_click: u16,  // add these to the count if so
    mouse_accel_factor: u8,
    key_repeat_delay: u8,
    options_b: u8,
    options_c: u8,

    #[br(count = number_of_chords)]
    pub chords: Vec<Chord>,

    #[br(calc = chords.iter().filter(|c| c.mapping.is_string()).count())]
    number_of_strings: usize,

    #[br(count = number_of_strings)]
    pub string_locations: Vec<u32>,

    #[br(count = number_of_strings)]
    pub string_contents: Vec<PosValue<StringContents>>,
}

#[derive(BinRead)]
#[br(little)]
#[derive(Debug)]
pub struct Chord {
    chord: ButtonData,
    #[br(restore_position)]
    modifier: u8,
    #[br(args { modifier })]
    pub mapping: ChordMapping,
}

impl Chord {
    pub fn button_state(&self) -> ButtonState {
        self.chord.into()
    }
}

#[derive(Eq, PartialEq, Hash, Debug, BinRead)]
#[br(little)]
#[br(import { modifier: u8 })]
pub(crate) enum ChordMapping {
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
pub struct StringContents {
    size: u16,

    #[br(count = size / 2 - 1, args { inner: ChordMappingBinReadArgs { modifier: 0 } })]
    pub keys: Vec<ChordMapping>,
}

#[bitfield]
#[derive(BinRead, Debug, Copy, Clone)]
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
            t1: self.num(),
            t2: self.alt(),
            t3: self.ctrl(),
            t4: self.shift(),
            f0l: false,
            f0m: false,
            f0r: false,
            f1r: self.a(),
            f1m: self.e(),
            f1l: self.sp(),
            f2r: self.b(),
            f2m: self.f(),
            f2l: self.del(),
            f3r: self.c(),
            f3m: self.g(),
            f3l: self.bs(),
            f4r: self.d(),
            f4m: self.h(),
            f4l: self.ent(),
        }
    }
}

pub(crate) fn parse<R: Read + Seek>(reader: &mut R) -> Result<Config, Box<dyn std::error::Error>> {
    let res = Config::read(reader);
    match res {
        Ok(config) => {
            assert!(config.version == 5, "Not a version 5 config file");
            Ok(config)
        }
        Err(e) => Err(Box::new(e)),
    }
}
