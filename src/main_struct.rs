
use std::sync::mpsc::Receiver;

use crate::mokyo_midi::*;
use crate::midi_state::*;

pub struct MainStruct {
    pub midi_state: MidiState,
    pub short_msg_receiver: Receiver<u32>,
    pub midi_hub: MidiHub,
    pub midi_seq: Option<MidiSequence>,
    pub file_name: Option<String>,
}