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

// translated from MokyoMidi via LLM

/* status codes */
pub const BYTE_STATUS_MASK: u8 = 0xF0;
/* note: midi event last 4 bits are channel number */
pub const BYTE_NOTE_OFF: u8 = 0x80;
pub const BYTE_NOTE_ON: u8 = 0x90;
pub const BYTE_POLYPHONIC_KEY_PRESSURE: u8 = 0xA0;
pub const BYTE_CONTROL_CHANGE: u8 = 0xB0;
pub const BYTE_PROGRAM_CHANGE: u8 = 0xC0;
pub const BYTE_CHANNEL_PRESSURE: u8 = 0xD0;
pub const BYTE_PITCH_BEND_CHANGE: u8 = 0xE0;
pub const BYTE_META_EVENT_OR_SYSEX: u8 = 0xF0;
/* meta events should not be sent to the synth */
pub const BYTE_META_EVENT: u8 = 0xFF;
/*
 * system exclusive events should be sent to the synth
 * including the byte F0
 */
pub const BYTE_SYSEX_START: u8 = 0xF0;
/*
 * used to end sysex, and also starts continuation
 * events and escape sequences
 */
pub const BYTE_SYSEX_END: u8 = 0xF7;

/* meta events have a secondary status code */
pub const BYTE_META_SEQUENCE_NUMBER: u8 = 0x00;
pub const BYTE_META_TEXT: u8 = 0x01;
pub const BYTE_META_COPYRIGHT: u8 = 0x02;
pub const BYTE_META_SEQUENCE_OR_TRACK_NAME: u8 = 0x03;
pub const BYTE_META_INSTRUMENT_NAME: u8 = 0x04;
pub const BYTE_META_LYRIC: u8 = 0x05;
pub const BYTE_META_MARKER: u8 = 0x06;
pub const BYTE_META_CUE_POINT: u8 = 0x07;
pub const BYTE_META_PROGRAM_NAME: u8 = 0x08;
pub const BYTE_META_DEVICE_NAME: u8 = 0x09;
pub const BYTE_META_MIDI_CHANNEL_PREFIX: u8 = 0x20;
pub const BYTE_META_MIDI_PORT: u8 = 0x21;
pub const BYTE_META_END_OF_TRACK: u8 = 0x2F;
pub const BYTE_META_TEMPO: u8 = 0x51;
pub const BYTE_META_SMPTE_OFFSET: u8 = 0x54;
pub const BYTE_META_TIME_SIGNATURE: u8 = 0x58;
pub const BYTE_META_KEY_SIGNATURE: u8 = 0x59;
pub const BYTE_META_SEQUENCER_SPECIFIC: u8 = 0x7F;

/* controller codes */
pub const BYTE_BANK_SELECT_MSB: u8 = 0;
pub const BYTE_MODULATION_WHEEL_MSB: u8 = 1;
pub const BYTE_BREATH_CONTROLLER_MSB: u8 = 2;
/* undefined 3 */
pub const BYTE_FOOT_PEDAL_MSB: u8 = 4;
pub const BYTE_PORTAMENTO_TIME_MSB: u8 = 5;
pub const BYTE_DATA_ENTRY_MSB: u8 = 6;
pub const BYTE_CHANNEL_VOLUME_MSB: u8 = 7;
pub const BYTE_MAIN_VOLUME_MSB: u8 = BYTE_CHANNEL_VOLUME_MSB;
pub const BYTE_BALANCE_MSB: u8 = 8;
/* undefined 9 */
pub const BYTE_PAN_MSB: u8 = 10;
pub const BYTE_EXPRESSION_MSB: u8 = 11;
pub const BYTE_EFFECT_CONTROL_1_MSB: u8 = 12;
pub const BYTE_EFFECT_CONTROL_2_MSB: u8 = 13;
/* undefined 14, 15 */
pub const BYTE_GENERAL_PURPOSE_CONTROLLER_1_MSB: u8 = 16;
pub const BYTE_GENERAL_PURPOSE_CONTROLLER_2_MSB: u8 = 17;
pub const BYTE_GENERAL_PURPOSE_CONTROLLER_3_MSB: u8 = 18;
pub const BYTE_GENERAL_PURPOSE_CONTROLLER_4_MSB: u8 = 19;
/* undefined 20-31 */
pub const BYTE_BANK_SELECT_LSB: u8 = 32;
pub const BYTE_MODULATION_WHEEL_LSB: u8 = 33;
pub const BYTE_BREATH_CONTROLLER_LSB: u8 = 34;
/* undefined 35 */
pub const BYTE_FOOT_PEDAL_LSB: u8 = 36;
pub const BYTE_PORTAMENTO_TIME_LSB: u8 = 37;
pub const BYTE_DATA_ENTRY_LSB: u8 = 38;
pub const BYTE_CHANNEL_VOLUME_LSB: u8 = 39;
pub const BYTE_MAIN_VOLUME_LSB: u8 = BYTE_CHANNEL_VOLUME_LSB;
pub const BYTE_BALANCE_LSB: u8 = 40;
/* undefined 41 */
pub const BYTE_PAN_LSB: u8 = 42;
pub const BYTE_EXPRESSION_LSB: u8 = 43;
pub const BYTE_EFFECT_CONTROL_1_LSB: u8 = 44;
pub const BYTE_EFFECT_CONTROL_2_LSB: u8 = 45;
/* undefined 46, 47 */
pub const BYTE_GENERAL_PURPOSE_CONTROLLER_1_LSB: u8 = 48;
pub const BYTE_GENERAL_PURPOSE_CONTROLLER_2_LSB: u8 = 49;
pub const BYTE_GENERAL_PURPOSE_CONTROLLER_3_LSB: u8 = 50;
pub const BYTE_GENERAL_PURPOSE_CONTROLLER_4_LSB: u8 = 51;
/* undefined 52-63 */
pub const BYTE_SUSTAIN_SWITCH: u8 = 64;
pub const BYTE_PORTAMENTO_SWITCH: u8 = 65;
pub const BYTE_SOSTENUTO_SWITCH: u8 = 66;
pub const BYTE_SOFT_PEDAL_SWITCH: u8 = 67;
pub const BYTE_LEGATO_SWITCH: u8 = 68;
pub const BYTE_HOLD_2_SWITCH: u8 = 69;
pub const BYTE_SOUND_CONTROLLER_1: u8 = 70;
pub const BYTE_SOUND_VARIATION: u8 = BYTE_SOUND_CONTROLLER_1;
pub const BYTE_SOUND_CONTROLLER_2: u8 = 71;
pub const BYTE_SOUND_TIMBRE: u8 = BYTE_SOUND_CONTROLLER_2;
pub const BYTE_SOUND_CONTROLLER_3: u8 = 72;
pub const BYTE_RELEASE_TIME: u8 = BYTE_SOUND_CONTROLLER_3;
pub const BYTE_SOUND_CONTROLLER_4: u8 = 73;
pub const BYTE_ATTACK_TIME: u8 = BYTE_SOUND_CONTROLLER_4;
pub const BYTE_SOUND_CONTROLLER_5: u8 = 74;
pub const BYTE_BRIGHTNESS: u8 = BYTE_SOUND_CONTROLLER_5;
pub const BYTE_SOUND_CONTROLLER_6: u8 = 75;
pub const BYTE_DECAY_TIME: u8 = BYTE_SOUND_CONTROLLER_6;
pub const BYTE_SOUND_CONTROLLER_7: u8 = 76;
pub const BYTE_VIBRATO_RATE: u8 = BYTE_SOUND_CONTROLLER_7;
pub const BYTE_SOUND_CONTROLLER_8: u8 = 77;
pub const BYTE_VIBRATO_DEPTH: u8 = BYTE_SOUND_CONTROLLER_8;
pub const BYTE_SOUND_CONTROLLER_9: u8 = 78;
pub const BYTE_VIBRATO_DELAY: u8 = BYTE_SOUND_CONTROLLER_9;
pub const BYTE_SOUND_CONTROLLER_0: u8 = 79;
pub const BYTE_GENERAL_PURPOSE_CONTROLLER_5: u8 = 80;
pub const BYTE_GENERAL_PURPOSE_CONTROLLER_6: u8 = 81;
pub const BYTE_GENERAL_PURPOSE_CONTROLLER_7: u8 = 82;
pub const BYTE_GENERAL_PURPOSE_CONTROLLER_8: u8 = 83;
pub const BYTE_PORTAMENTO_CONTROL: u8 = 84;
/* undefined 85-87 */
pub const BYTE_HIGH_RESOLUTION_VELOCITY_PREFIX: u8 = 88;
/* undefined 89, 90 */
pub const BYTE_EFFECTS_1_DEPTH: u8 = 91;
pub const BYTE_REVERB: u8 = BYTE_EFFECTS_1_DEPTH;
pub const BYTE_EFFECTS_2_DEPTH: u8 = 92;
pub const BYTE_TREMELO: u8 = BYTE_EFFECTS_2_DEPTH;
pub const BYTE_EFFECTS_3_DEPTH: u8 = 93;
pub const BYTE_CHORUS: u8 = BYTE_EFFECTS_3_DEPTH;
pub const BYTE_EFFECTS_4_DEPTH: u8 = 94;
pub const BYTE_DETUNE: u8 = BYTE_EFFECTS_4_DEPTH;
pub const BYTE_DELAY: u8 = BYTE_EFFECTS_4_DEPTH;
pub const BYTE_EFFECTS_5_DEPTH: u8 = 95;
pub const BYTE_PHASER: u8 = BYTE_EFFECTS_5_DEPTH;
pub const BYTE_DATA_INCREMENT: u8 = 96;
pub const BYTE_DATA_DECREMENT: u8 = 97;
pub const BYTE_NRPN_LSB: u8 = 98;
pub const BYTE_NRPN_MSB: u8 = 99;
pub const BYTE_RPN_LSB: u8 = 100;
pub const BYTE_RPN_MSB: u8 = 101;
/* undefined 102-119 */

/* channel mode messages */
pub const BYTE_ALL_SOUND_OFF: u8 = 120;
pub const BYTE_RESET_ALL_CONTROLLERS: u8 = 121;
pub const BYTE_LOCAL_CONTROL: u8 = 122;
pub const BYTE_ALL_NOTES_OFF: u8 = 123;
pub const BYTE_OMNI_MODE_OFF: u8 = 124;
pub const BYTE_OMNI_MODE_ON: u8 = 125;
pub const BYTE_MONO_MODE_ON: u8 = 126;
pub const BYTE_POLY_MODE_ON: u8 = 127;


impl MidiShortMsg {
    pub fn new(raw_msg: u32) {
        // todo
    }
}
