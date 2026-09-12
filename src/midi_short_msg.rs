/*
 * This file provides a convenient interface for
 * dealing with midi short messages programatically
 * 
 * Author: ickdanny
 */

/* these are the CC types we are interested in */
pub enum CCType {
    BankSel,
    Modulation,
    DataEntry,
    MainVol,
    Pan,
    Expression,
    Reverb,
    Chorus,
    Delay,
    NrpnLsb,
    NrpnMsb,
    RpnLsb,
    RpnMsb,
    AllSoundOff,
    AllNotesOff,
    Other,
}

pub enum MidiShortMsg {
    NoteOff { 
        note_num: u8,
        velocity: u8,
    },
    NoteOn { 
        note_num: u8,
        velocity: u8,
    },
    PolyphonicKeyPressure { // aftertouch
        note_num: u8,
        pressure: u8,
    },
    ControlChange { 
        cc: CCType,
        value: u8,
    },
    ProgramChange { 
        value: u8,
    },
    ChannelPressure { 
        pressure: u8,
    },
    PitchBendChange { 
        amount: i16,
    },
    Invalid,
}

impl MidiShortMsg {
    pub fn new(raw_msg: u32) {
        // todo
    }
}