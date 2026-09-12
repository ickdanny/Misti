// todo get mokyomidi to work on Rust

use std::ffi::CString;
use std::thread;
use std::time::Duration;

#[repr(C)]
struct MidiHub {
    _dummy: [u8; 0],
}

#[repr(C)]
struct MidiSequence {
    _dummy: [u8; 0],
}

unsafe extern "C" {
    fn midiSequenceAllocAndParseMidiFile(
        fileName: *const std::ffi::c_char
    ) -> *mut MidiSequence;
    fn midiSequenceFreeAlloc(
        midiSequencePtr: *mut MidiSequence
    );

    fn midiHubAlloc(muted: bool) -> *mut MidiHub;
    fn midiHubStart(
        midiHubPtr: *mut MidiHub,
        midiSequencePtr: *mut MidiSequence
    );
    fn midiHubStop(midiHubPtr: *mut MidiHub);
    fn midiHubFreeAlloc(midiHubPtr: *mut MidiHub);
}

fn main() {
    let file_name = CString::new("test.mid").unwrap();

    unsafe {
        let seq = midiSequenceAllocAndParseMidiFile(
            file_name.as_ptr()
        );
        let midi_hub = midiHubAlloc(false);
        println!("just before starting midi hub");
        midiHubStart(midi_hub, seq);
        println!("after starting midi hub");
        thread::sleep(Duration::from_millis(1000 * 5));
        midiHubFreeAlloc(midi_hub);
        midiSequenceFreeAlloc(seq);
    }
}