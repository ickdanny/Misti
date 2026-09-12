/*
 * This file defines the state of a midi file being
 * played back (that we are interested in visualizing).
 * 
 * Author: ickdanny
 */

const NUM_NOTES_PER_CHANNEL: usize = 128;
const NUM_CHANNELS: usize = 16;

pub struct MidiState {
    song_name: String,
    // todo time sig
    // todo tempo
    channel_states: [ChannelState; NUM_CHANNELS],
}

impl MidiState {
    pub fn new() -> Self {
        Self {
            song_name: String::new(),
            channel_states: std::array::from_fn(|_| ChannelState::new()),
        }
    }

    pub fn update(short_msg: u32) {
    }
}

struct ChannelState {
    inst_name: String,
    program: u8,
    bank: u8,
    pan: u8,
    main_vol: u8,
    expression: u8,
    modulation: u8,
    reverb: u8,
    chorus: u8,
    delay: u8,
    last_rpn_lsb: u8,
    last_rpn_msb: u8,
    was_last_param_registered: bool
    pitchbend_sens: u8,
    pitchbend_amount: i16,
    note_states: [NoteState; NUM_NOTES_PER_CHANNEL],
}

impl ChannelState {
    fn new() -> Self {
        Self {
            inst_name: String::new(),
            program: 0,
            bank: 0,
            pan: 64,
            main_vol: 100,
            expression: 100,
            modulation: 0,
            reverb: 0,
            chorus: 0,
            delay: 0,
            last_rpn_lsb: 0,
            last_rpn_msb: 0,
            pitchbend_sens: 0,
            pitchbend_amount: 0,
            note_states: std::array::from_fn(|_| NoteState::new()),
        }
    }
}

struct NoteState {
    on: bool,
    velocity: u8,
}

impl NoteState {
    fn new() -> Self {
        Self { on: false, velocity: 0 }
    }
}