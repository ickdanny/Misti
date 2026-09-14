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
    pub song_name: String,
    // todo time sig
    // todo tempo
    pub channel_states: [ChannelState; NUM_CHANNELS],
}

impl MidiState {
    pub fn new() -> Self {
        Self {
            song_name: String::new(),
            channel_states: std::array::from_fn(|i| ChannelState::new(i as u8)),
        }
    }

    pub fn reset(&mut self) {
        self.song_name = String::new();
        self.channel_states = std::array::from_fn(|i| ChannelState::new(i as u8));
    }

    pub fn update(
        &mut self,
        raw_msg: u32,
        inst_names: &[String; 128],
        drum_names: &[String; 128],
    ) {
        let short_msg = MidiShortMsg::from(raw_msg);
        match short_msg {
            MidiShortMsg::NoteOff { note_num: _, velocity: _, channel } => {
                self.channel_states[channel as usize]
                    .update(short_msg, inst_names, drum_names);
            },
            MidiShortMsg::NoteOn { note_num: _, velocity: _, channel } => {
                self.channel_states[channel as usize]
                    .update(short_msg, inst_names, drum_names);
            }
            MidiShortMsg::ControlChange { cc: _, value: _, channel } => {
                self.channel_states[channel as usize]
                    .update(short_msg, inst_names, drum_names);
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
                self.channel_states[channel as usize]
                    .update(short_msg, inst_names, drum_names);
            }
            MidiShortMsg::PitchBendChange { amount: _, channel } => {
                self.channel_states[channel as usize]
                    .update(short_msg, inst_names, drum_names);
            }
            _ => {
                println!("other update in state: {:x}", raw_msg);
            }
        }
    }
}

pub struct ChannelState {
    pub channel: u8,
    pub inst_name: String,
    pub program: u8,
    pub bank: u8,
    pub pan: u8,
    pub main_vol: u8,
    pub expression: u8,
    pub modulation: u8,
    pub reverb: u8,
    pub chorus: u8,
    pub delay: u8,
    pub last_rpn_lsb: u8,
    pub last_rpn_msb: u8,
    pub was_last_param_registered: bool,
    pub pitchbend_sens: u8, // in semitones
    pub pitchbend_amount: i16,
    pub note_states: [NoteState; NUM_NOTES_PER_CHANNEL],
}

impl ChannelState {
    fn new(channel: u8) -> Self {
        Self {
            channel,
            inst_name: String::from("- - -"),
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

    fn update_inst_name(
        &mut self,
        inst_names: &[String; 128],
        drum_names: &[String; 128],
    ) {
        if self.channel != 9 {
            self.inst_name = inst_names[self.program as usize].clone();
        } else {
            self.inst_name = drum_names[self.program as usize].clone();
        }
    }

    fn update_cc(
        &mut self,
        cc: CCType, value: u8,
        inst_names: &[String; 128],
        drum_names: &[String; 128]
    ) {
        match cc {
            CCType::BankSel => {
                self.bank = value;
                self.update_inst_name(inst_names, drum_names);
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

    fn update(
        &mut self,
        short_msg: MidiShortMsg,
        inst_names: &[String; 128],
        drum_names: &[String; 128],
    ) {
        // assume the channel is correct lmao
        match short_msg {
            MidiShortMsg::NoteOff { note_num, velocity, .. } => {
                self.note_states[note_num as usize].turn_off(velocity);
            },
            MidiShortMsg::NoteOn { note_num, velocity, .. } => {
                self.note_states[note_num as usize].turn_on(velocity);
            }
            MidiShortMsg::ControlChange { cc, value, .. } => {
                self.update_cc(cc, value, inst_names, drum_names);
            }
            MidiShortMsg::ProgramChange { value, .. } => {
                self.program = value;
                self.update_inst_name(inst_names, drum_names);
            }
            MidiShortMsg::PitchBendChange { amount, .. } => {
                self.pitchbend_amount = amount;
            }
            _ => {
                println!("other update in channel");
            }
        }
    }
}

pub struct NoteState {
    on_count: u32,
    pub velocity: u8,
}

impl NoteState {
    fn new() -> Self {
        Self { on_count: 0, velocity: 0 }
    }

    fn turn_on(&mut self, velocity: u8) {
        if velocity == 0 {
            self.turn_off(0);
        } else {
            self.on_count += 1;
            self.velocity = velocity;
        }
    }

    fn turn_off(&mut self, velocity: u8) {
        if self.on_count > 0 {
            self.on_count -= 1;
        }
        self.velocity = velocity;
    }

    pub fn is_on(&self) -> bool {
        self.on_count > 0
    }
}