/*
 * This file provides a convenient interface for
 * dealing with midi short messages programatically
 * 
 * Author: ickdanny
 */

use either::*;

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
    Other,
}

pub enum ChannelModeCCType {
    AllSoundOff,
    AllNotesOff,
    Other,
}

pub enum MidiShortMsg {
    NoteOff { 
        note_num: u8,
        velocity: u8,
        channel: u8,
    },
    NoteOn { 
        note_num: u8,
        velocity: u8,
        channel: u8,
    },
    PolyphonicKeyPressure { // aftertouch
        note_num: u8,
        pressure: u8,
        channel: u8,
    },
    ControlChange { 
        cc: CCType,
        value: u8,
        channel: u8,
    },
    ChannelModeControlChange {
        cc: ChannelModeCCType,
    },
    ProgramChange { 
        value: u8,
        channel: u8,
    },
    ChannelPressure { 
        pressure: u8,
        channel: u8,
    },
    PitchBendChange { 
        amount: i16,
        channel: u8,
    },
    Invalid,
}

/* ================== *
 * MIDI BYTE MEANINGS *
 * ================== */

// translated from MokyoMidi via LLM

/* status codes */
const MIDI_STATUS_MASK: u8 = 0xF0;
const MIDI_CHANNEL_MASK: u8 = 0x0F;
/* note: midi event last 4 bits are channel number */
const MIDI_NOTE_OFF: u8 = 0x80;
const MIDI_NOTE_ON: u8 = 0x90;
const MIDI_POLYPHONIC_KEY_PRESSURE: u8 = 0xA0;
const MIDI_CONTROL_CHANGE: u8 = 0xB0;
const MIDI_PROGRAM_CHANGE: u8 = 0xC0;
const MIDI_CHANNEL_PRESSURE: u8 = 0xD0;
const MIDI_PITCH_BEND_CHANGE: u8 = 0xE0;
const MIDI_META_EVENT_OR_SYSEX: u8 = 0xF0;
/* meta events should not be sent to the synth */
const MIDI_META_EVENT: u8 = 0xFF;
/*
 * system exclusive events should be sent to the synth
 * including the byte F0
 */
const MIDI_SYSEX_START: u8 = 0xF0;
/*
 * used to end sysex, and also starts continuation
 * events and escape sequences
 */
const MIDI_SYSEX_END: u8 = 0xF7;

/* meta events have a secondary status code */
const MIDI_META_SEQUENCE_NUMBER: u8 = 0x00;
const MIDI_META_TEXT: u8 = 0x01;
const MIDI_META_COPYRIGHT: u8 = 0x02;
const MIDI_META_SEQUENCE_OR_TRACK_NAME: u8 = 0x03;
const MIDI_META_INSTRUMENT_NAME: u8 = 0x04;
const MIDI_META_LYRIC: u8 = 0x05;
const MIDI_META_MARKER: u8 = 0x06;
const MIDI_META_CUE_POINT: u8 = 0x07;
const MIDI_META_PROGRAM_NAME: u8 = 0x08;
const MIDI_META_DEVICE_NAME: u8 = 0x09;
const MIDI_META_MIDI_CHANNEL_PREFIX: u8 = 0x20;
const MIDI_META_MIDI_PORT: u8 = 0x21;
const MIDI_META_END_OF_TRACK: u8 = 0x2F;
const MIDI_META_TEMPO: u8 = 0x51;
const MIDI_META_SMPTE_OFFSET: u8 = 0x54;
const MIDI_META_TIME_SIGNATURE: u8 = 0x58;
const MIDI_META_KEY_SIGNATURE: u8 = 0x59;
const MIDI_META_SEQUENCER_SPECIFIC: u8 = 0x7F;

/* controller codes */
const MIDI_BANK_SELECT_MSB: u8 = 0;
const MIDI_MODULATION_WHEEL_MSB: u8 = 1;
const MIDI_BREATH_CONTROLLER_MSB: u8 = 2;
/* undefined 3 */
const MIDI_FOOT_PEDAL_MSB: u8 = 4;
const MIDI_PORTAMENTO_TIME_MSB: u8 = 5;
const MIDI_DATA_ENTRY_MSB: u8 = 6;
const MIDI_CHANNEL_VOLUME_MSB: u8 = 7;
const MIDI_MAIN_VOLUME_MSB: u8 = MIDI_CHANNEL_VOLUME_MSB;
const MIDI_BALANCE_MSB: u8 = 8;
/* undefined 9 */
const MIDI_PAN_MSB: u8 = 10;
const MIDI_EXPRESSION_MSB: u8 = 11;
const MIDI_EFFECT_CONTROL_1_MSB: u8 = 12;
const MIDI_EFFECT_CONTROL_2_MSB: u8 = 13;
/* undefined 14, 15 */
const MIDI_GENERAL_PURPOSE_CONTROLLER_1_MSB: u8 = 16;
const MIDI_GENERAL_PURPOSE_CONTROLLER_2_MSB: u8 = 17;
const MIDI_GENERAL_PURPOSE_CONTROLLER_3_MSB: u8 = 18;
const MIDI_GENERAL_PURPOSE_CONTROLLER_4_MSB: u8 = 19;
/* undefined 20-31 */
const MIDI_BANK_SELECT_LSB: u8 = 32;
const MIDI_MODULATION_WHEEL_LSB: u8 = 33;
const MIDI_BREATH_CONTROLLER_LSB: u8 = 34;
/* undefined 35 */
const MIDI_FOOT_PEDAL_LSB: u8 = 36;
const MIDI_PORTAMENTO_TIME_LSB: u8 = 37;
const MIDI_DATA_ENTRY_LSB: u8 = 38;
const MIDI_CHANNEL_VOLUME_LSB: u8 = 39;
const MIDI_MAIN_VOLUME_LSB: u8 = MIDI_CHANNEL_VOLUME_LSB;
const MIDI_BALANCE_LSB: u8 = 40;
/* undefined 41 */
const MIDI_PAN_LSB: u8 = 42;
const MIDI_EXPRESSION_LSB: u8 = 43;
const MIDI_EFFECT_CONTROL_1_LSB: u8 = 44;
const MIDI_EFFECT_CONTROL_2_LSB: u8 = 45;
/* undefined 46, 47 */
const MIDI_GENERAL_PURPOSE_CONTROLLER_1_LSB: u8 = 48;
const MIDI_GENERAL_PURPOSE_CONTROLLER_2_LSB: u8 = 49;
const MIDI_GENERAL_PURPOSE_CONTROLLER_3_LSB: u8 = 50;
const MIDI_GENERAL_PURPOSE_CONTROLLER_4_LSB: u8 = 51;
/* undefined 52-63 */
const MIDI_SUSTAIN_SWITCH: u8 = 64;
const MIDI_PORTAMENTO_SWITCH: u8 = 65;
const MIDI_SOSTENUTO_SWITCH: u8 = 66;
const MIDI_SOFT_PEDAL_SWITCH: u8 = 67;
const MIDI_LEGATO_SWITCH: u8 = 68;
const MIDI_HOLD_2_SWITCH: u8 = 69;
const MIDI_SOUND_CONTROLLER_1: u8 = 70;
const MIDI_SOUND_VARIATION: u8 = MIDI_SOUND_CONTROLLER_1;
const MIDI_SOUND_CONTROLLER_2: u8 = 71;
const MIDI_SOUND_TIMBRE: u8 = MIDI_SOUND_CONTROLLER_2;
const MIDI_SOUND_CONTROLLER_3: u8 = 72;
const MIDI_RELEASE_TIME: u8 = MIDI_SOUND_CONTROLLER_3;
const MIDI_SOUND_CONTROLLER_4: u8 = 73;
const MIDI_ATTACK_TIME: u8 = MIDI_SOUND_CONTROLLER_4;
const MIDI_SOUND_CONTROLLER_5: u8 = 74;
const MIDI_BRIGHTNESS: u8 = MIDI_SOUND_CONTROLLER_5;
const MIDI_SOUND_CONTROLLER_6: u8 = 75;
const MIDI_DECAY_TIME: u8 = MIDI_SOUND_CONTROLLER_6;
const MIDI_SOUND_CONTROLLER_7: u8 = 76;
const MIDI_VIBRATO_RATE: u8 = MIDI_SOUND_CONTROLLER_7;
const MIDI_SOUND_CONTROLLER_8: u8 = 77;
const MIDI_VIBRATO_DEPTH: u8 = MIDI_SOUND_CONTROLLER_8;
const MIDI_SOUND_CONTROLLER_9: u8 = 78;
const MIDI_VIBRATO_DELAY: u8 = MIDI_SOUND_CONTROLLER_9;
const MIDI_SOUND_CONTROLLER_0: u8 = 79;
const MIDI_GENERAL_PURPOSE_CONTROLLER_5: u8 = 80;
const MIDI_GENERAL_PURPOSE_CONTROLLER_6: u8 = 81;
const MIDI_GENERAL_PURPOSE_CONTROLLER_7: u8 = 82;
const MIDI_GENERAL_PURPOSE_CONTROLLER_8: u8 = 83;
const MIDI_PORTAMENTO_CONTROL: u8 = 84;
/* undefined 85-87 */
const MIDI_HIGH_RESOLUTION_VELOCITY_PREFIX: u8 = 88;
/* undefined 89, 90 */
const MIDI_EFFECTS_1_DEPTH: u8 = 91;
const MIDI_REVERB: u8 = MIDI_EFFECTS_1_DEPTH;
const MIDI_EFFECTS_2_DEPTH: u8 = 92;
const MIDI_TREMELO: u8 = MIDI_EFFECTS_2_DEPTH;
const MIDI_EFFECTS_3_DEPTH: u8 = 93;
const MIDI_CHORUS: u8 = MIDI_EFFECTS_3_DEPTH;
const MIDI_EFFECTS_4_DEPTH: u8 = 94;
const MIDI_DETUNE: u8 = MIDI_EFFECTS_4_DEPTH;
const MIDI_DELAY: u8 = MIDI_EFFECTS_4_DEPTH;
const MIDI_EFFECTS_5_DEPTH: u8 = 95;
const MIDI_PHASER: u8 = MIDI_EFFECTS_5_DEPTH;
const MIDI_DATA_INCREMENT: u8 = 96;
const MIDI_DATA_DECREMENT: u8 = 97;
const MIDI_NRPN_LSB: u8 = 98;
const MIDI_NRPN_MSB: u8 = 99;
const MIDI_RPN_LSB: u8 = 100;
const MIDI_RPN_MSB: u8 = 101;
/* undefined 102-119 */

/* channel mode messages */
const MIDI_ALL_SOUND_OFF: u8 = 120;
const MIDI_RESET_ALL_CONTROLLERS: u8 = 121;
const MIDI_LOCAL_CONTROL: u8 = 122;
const MIDI_ALL_NOTES_OFF: u8 = 123;
const MIDI_OMNI_MODE_OFF: u8 = 124;
const MIDI_OMNI_MODE_ON: u8 = 125;
const MIDI_MONO_MODE_ON: u8 = 126;
const MIDI_POLY_MODE_ON: u8 = 127;

fn code_to_cc(code: u8) -> Either<CCType, ChannelModeCCType> {
    match code {
        MIDI_BANK_SELECT_MSB => Left(CCType::BankSel),
        MIDI_MODULATION_WHEEL_MSB => Left(CCType::Modulation),
        MIDI_DATA_ENTRY_MSB => Left(CCType::DataEntry),
        MIDI_MAIN_VOLUME_MSB => Left(CCType::MainVol),
        MIDI_PAN_MSB => Left(CCType::Pan),
        MIDI_EXPRESSION_MSB => Left(CCType::Expression),
        MIDI_REVERB => Left(CCType::Reverb),
        MIDI_CHORUS => Left(CCType::Chorus),
        MIDI_DELAY => Left(CCType::Delay),
        MIDI_NRPN_LSB => Left(CCType::NrpnLsb),
        MIDI_NRPN_MSB => Left(CCType::NrpnMsb),
        MIDI_RPN_LSB => Left(CCType::RpnLsb),
        MIDI_RPN_MSB => Left(CCType::RpnMsb),
        MIDI_ALL_SOUND_OFF => Right(ChannelModeCCType::AllSoundOff),
        MIDI_ALL_NOTES_OFF => Right(ChannelModeCCType::AllNotesOff),
        _ => Left(CCType::Other),
    }
}

impl MidiShortMsg {
    pub fn from(raw_msg: u32) -> Self{
        let byte1: u8 = (raw_msg & 0xFF).try_into().unwrap();
        let byte2: u8 = ((raw_msg >> 8) & 0xFF).try_into().unwrap();
        let byte3: u8 = ((raw_msg >> 16) & 0xFF).try_into().unwrap();
        
        let channel: u8 = byte1 & MIDI_CHANNEL_MASK;
        let status_code: u8 = byte1 & MIDI_STATUS_MASK;

        match status_code {
            MIDI_NOTE_OFF => Self::NoteOff {
                note_num: byte2,
                velocity: byte3,
                channel: channel,
            },
            MIDI_NOTE_ON => Self::NoteOn {
                note_num: byte2,
                velocity: byte3,
                channel: channel,
            },
            MIDI_POLYPHONIC_KEY_PRESSURE
                => Self::PolyphonicKeyPressure {
                    note_num: byte2,
                    pressure: byte3,
                    channel: channel,
                },
            MIDI_CONTROL_CHANGE => match code_to_cc(byte2) {
                Left(cc_type) => Self::ControlChange {
                    cc: cc_type,
                    value: byte3,
                    channel: channel
                },
                Right(channel_mode_cc_type)
                    => Self::ChannelModeControlChange {
                        cc: channel_mode_cc_type
                    }
            },
            MIDI_PROGRAM_CHANGE => Self::ProgramChange {
                value: byte2,
                channel: channel,
            },
            MIDI_CHANNEL_PRESSURE => Self::ChannelPressure {
                pressure: byte2,
                channel: channel,
            },
            MIDI_PITCH_BEND_CHANGE => {
                // byte 3 is most significant 7 bits
                let combined_data: u16 = byte3.into();
                let combined_data = combined_data << 7;
                // byte 2 is least significant 7 bits
                let combined_data = combined_data | (byte2 as u16);
                let amount: i16 = combined_data as i16;
                // account for center
                let amount = amount - 0x2000;
                Self::PitchBendChange {
                    amount: amount,
                    channel: channel,
                }
            },
            _ => Self::Invalid,
        }
    }
}
