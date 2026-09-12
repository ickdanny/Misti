/*
 * This file defines the state of a midi file being
 * played back (that we are interested in visualizing).
 * 
 * Author: ickdanny
 */


use crate::midi_short_msg::*;

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

    pub fn update(&mut self, raw_msg: u32) {
        let short_msg = MidiShortMsg::from(raw_msg);
        match short_msg {
            MidiShortMsg::NoteOff { note_num: _, velocity: _, channel } => {
                self.channel_states[channel as usize].update(short_msg);
            },
            MidiShortMsg::NoteOn { note_num: _, velocity: _, channel } => {
                self.channel_states[channel as usize].update(short_msg);
            }
            MidiShortMsg::ControlChange { cc: _, value: _, channel } => {
                self.channel_states[channel as usize].update(short_msg);
            }
            MidiShortMsg::ChannelModeControlChange { cc } => {
                match cc {
                    ChannelModeCCType::AllSoundOff => {
                        // todo
                    },
                    ChannelModeCCType::AllNotesOff => {
                        // todo
                    },
                    _ => ()
                }
            }
            MidiShortMsg::ProgramChange { value: _, channel } => {
                self.channel_states[channel as usize].update(short_msg);
            }
            MidiShortMsg::PitchBendChange { amount: _, channel } => {
                self.channel_states[channel as usize].update(short_msg);
            }
            _ => ()
        }
    }
}

pub struct ChannelState {
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
    was_last_param_registered: bool,
    pitchbend_sens: u8, // in semitones
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
            was_last_param_registered: true,
            pitchbend_sens: 0,
            pitchbend_amount: 0,
            note_states: std::array::from_fn(|_| NoteState::new()),
        }
    }

    fn update_inst_name(&mut self) {
        self.inst_name = String::from("TODO NOT IMPL");
    }

    fn update_cc(&mut self, cc: CCType, value: u8) {
        match cc {
            CCType::BankSel => {
                self.bank = value;
                self.update_inst_name();
            },
            CCType::Modulation => {
                self.modulation = value;
            },
            CCType::DataEntry => {
                // we only check this for pitchbend sens
                if self.was_last_param_registered 
                    && self.last_rpn_lsb == 0
                    && self.last_rpn_msb == 0
                {
                    self.pitchbend_sens = value;
                }
            },
            CCType::MainVol => {
                self.main_vol = value;
            },
            CCType::Pan => {
                self.pan = value;
            },
            CCType::Expression => {
                self.expression = value;
            },
            CCType::Reverb => {
                self.reverb = value;
            },
            CCType::Chorus => {
                self.chorus = value;
            },
            CCType::Delay => {
                self.delay = value;
            },
            CCType::NrpnLsb => {
                self.was_last_param_registered = false;
            },
            CCType::NrpnMsb => {
                self.was_last_param_registered = false;
            },
            CCType::RpnLsb => {
                self.was_last_param_registered = true;
                self.last_rpn_lsb = value;
            },
            CCType::RpnMsb => {
                self.was_last_param_registered = true;
                self.last_rpn_msb = value;
            },
            CCType::Other => (),
        }
    }

    fn update(&mut self, short_msg: MidiShortMsg) {
        // assume the channel is correct lmao
        match short_msg {
            MidiShortMsg::NoteOff { note_num, velocity, .. } => {
                self.note_states[note_num as usize].turn_off(velocity);
            },
            MidiShortMsg::NoteOn { note_num, velocity, .. } => {
                self.note_states[note_num as usize].turn_on(velocity);
            }
            MidiShortMsg::ControlChange { cc, value, .. } => {
                self.update_cc(cc, value);
            }
            MidiShortMsg::ProgramChange { value, .. } => {
                self.program = value;
                self.update_inst_name();
            }
            MidiShortMsg::PitchBendChange { amount, .. } => {
                self.pitchbend_amount = amount;
            }
            _ => ()
        }
    }
}

pub struct NoteState {
    on: bool,
    velocity: u8,
}

impl NoteState {
    fn new() -> Self {
        Self { on: false, velocity: 0 }
    }

    fn turn_on(&mut self, velocity: u8) {
        self.on = true;
        self.velocity = velocity;
    }

    fn turn_off(&mut self, velocity: u8) {
        self.on = false;
        self.velocity = velocity;
    }
}