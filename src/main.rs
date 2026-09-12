
use std::sync::mpsc::{Sender, Receiver, channel};
use std::sync::OnceLock;
use std::thread;
use std::time::Duration;

use macroquad::prelude::*;

mod mokyo_midi;
mod midi_state;

use crate::mokyo_midi::*;
use crate::midi_state::*;


static SHORT_MSG_SENDER: OnceLock<Sender<u32>> = OnceLock::new();

extern "C" fn short_msg_callback(a: u32) {
    match SHORT_MSG_SENDER.get() {
        Some(sender) => sender.send(a).unwrap(),
        _ => ()
    }
}

fn init_channel() -> Option<Receiver<u32>> {
    if !SHORT_MSG_SENDER.get().is_none() {
        None
    } else {
        let (sender, receiver) = channel();
        SHORT_MSG_SENDER.set(sender).unwrap();
        Some(receiver)
    }
}

#[macroquad::main("Misti")]
async fn main() {
    let midi_state = MidiState::new();
    let receiver = init_channel().unwrap();
    clear_background(BLACK);
 
    draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
    draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);

    draw_text("Hello Macroquad!", 20.0, 20.0, 30.0, DARKGRAY);

    next_frame().await;

    let file_name = "test.mid";

    let midihub = MidiHub::new(short_msg_callback);
    let seq = MidiSequence::from_file(file_name);
    midihub.start(&seq);

    thread::sleep(Duration::from_millis(1000 * 5));
    
    for _ in 0..100 {
        draw_text(format!("received {:x}", receiver.recv().unwrap()), 50.0, 50.0, 50.0, GRAY);
        next_frame().await;
        thread::sleep(Duration::from_millis(50));
    }
    midihub.stop();
}