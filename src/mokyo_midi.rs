/*
 * This file provides basic Rust bindings to the C
 * library MokyoMidi. 
 *
 * Author: ickdanny
 */

use std::ffi::CString;

unsafe extern "C" {
    fn midiSequenceAllocAndParseMidiFile(
        fileName: *const std::ffi::c_char
    ) -> *mut MidiSequenceDummy;
    fn midiSequenceFreeAlloc(
        midiSequencePtr: *mut MidiSequenceDummy
    );
    fn midiHubAlloc(
        muted: bool,
        short_msg_callback: extern "C" fn(u32)
    ) -> *mut MidiHubDummy;
    fn midiHubStart(
        midiHubPtr: *mut MidiHubDummy,
        midiSequencePtr: *mut MidiSequenceDummy
    );
    fn midiHubStop(midiHubPtr: *mut MidiHubDummy);
    fn midiHubFreeAlloc(midiHubPtr: *mut MidiHubDummy);
}

#[repr(C)]
struct MidiHubDummy {
    _dummy: [u8; 0],
}

pub struct MidiHub {
    ptr: *mut MidiHubDummy,
}

#[repr(C)]
struct MidiSequenceDummy {
    _dummy: [u8; 0],
}

pub struct MidiSequence {
    ptr: *mut MidiSequenceDummy,
}

impl MidiHub {
    pub fn new(short_msg_callback: extern "C" fn(u32)) -> Self {
        unsafe {
            MidiHub { ptr: midiHubAlloc(false, short_msg_callback) }
        }
    }
    pub fn start(&self, seq: &MidiSequence) {
        unsafe {
            midiHubStart(self.ptr, seq.ptr);
        }
    }
    pub fn stop(&self) {
        unsafe {
            midiHubStop(self.ptr);
        }
    }
}

impl Drop for MidiHub {
    fn drop(&mut self) {
        unsafe {
            midiHubFreeAlloc(self.ptr);
        }
    }
}

impl MidiSequence {
    // TODO: better to return Option
    pub fn from_file(file_name: &str) -> Self {
        let file_name = CString::new(file_name).unwrap();
        // TODO: best to test result of above
        unsafe {
            MidiSequence {
                ptr: midiSequenceAllocAndParseMidiFile(file_name.as_ptr())
            }
        }
    }
}

impl Drop for MidiSequence {
    fn drop(&mut self) {
        unsafe {
            midiSequenceFreeAlloc(self.ptr);
        }
    }
}