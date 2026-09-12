

use std::thread;
use std::time::Duration;

use macroquad::prelude::*;

use crate::mokyo_midi::*;
mod mokyo_midi;

extern fn callback(a: u32) {
    println!("{:x}", a);
}

#[macroquad::main("Misti")]
async fn main() {
    clear_background(BLACK);
 
    draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
    draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);

    draw_text("Hello Macroquad!", 20.0, 20.0, 30.0, DARKGRAY);

    next_frame().await;

    let file_name = "test.mid";

    let midihub = MidiHub::new(callback);
    let seq = MidiSequence::from_file(file_name);
    midihub.start(&seq);
    thread::sleep(Duration::from_millis(1000 * 5));
}