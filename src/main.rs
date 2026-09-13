
use std::sync::mpsc::{Sender, Receiver, channel, TryRecvError};
use std::sync::OnceLock;
use std::thread;
use std::time::Duration;

use bevy::prelude::*;
use bevy::sprite::*;

mod mokyo_midi;
mod midi_state;
mod midi_short_msg;
mod graphics;

use crate::mokyo_midi::*;
use crate::midi_state::*;
use crate::graphics::*;

const GAME_WIDTH: u32 = 640;
const GAME_HEIGHT: u32 = 380;
const PIXEL_RATIO: u32 = 2;
const WINDOW_WIDTH: u32 = GAME_WIDTH * PIXEL_RATIO;
const WINDOW_HEIGHT: u32 = GAME_HEIGHT * PIXEL_RATIO;

static SHORT_MSG_SENDER: OnceLock<Sender<u32>> = OnceLock::new();

extern "C" fn short_msg_callback(a: u32) {
    match SHORT_MSG_SENDER.get() {
        Some(sender) => {
            match sender.send(a) {
                Ok(_) => (),
                Err(err) => {
                    // println!("{:?}", err);
                }
            }
        },
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

struct MainStruct {
    midi_state: MidiState,
    short_msg_receiver: Receiver<u32>,
    midi_hub: MidiHub,
    midi_seq: Option<MidiSequence>,
}

impl MainStruct {
    fn new() -> Self {
        let midi_state = MidiState::new();
        let short_msg_receiver = init_channel().unwrap();
        let midi_hub = MidiHub::new(short_msg_callback);
        Self {
            midi_state,
            short_msg_receiver,
            midi_hub,
            midi_seq: None
        }
    }
}

fn spawn_camera_system(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: bevy::camera::ScalingMode::Fixed {
                width: GAME_WIDTH as f32,
                height: GAME_HEIGHT as f32,
            },
            ..OrthographicProjection::default_2d()
        }),
        Transform::from_xyz(
            (GAME_WIDTH / 2) as f32,
            -((GAME_HEIGHT / 2) as f32),
            0.0
        )
    ));
}

fn spawn_graphics_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.spawn((
        Sprite::from_image(asset_server.load("test.png")),
        Transform::from_xyz(0.0, -0.0, 2.0),
        Anchor::TOP_LEFT,
    ));
}

fn update_state_system(
    mut main_struct: NonSendMut<MainStruct>,
) {
    loop {
        match main_struct.short_msg_receiver.try_recv() {
            Ok(data) => {
                main_struct.midi_state.update(data);
            },
            Err(TryRecvError::Empty) => {
                break;
            },
            Err(TryRecvError::Disconnected) => {
                break;
            }
        }
    }
}

fn file_drop_system(
    mut events: MessageReader<FileDragAndDrop>,
    mut main_struct: NonSendMut<MainStruct>,
) {
    for event in events.read() {
        match event {
            FileDragAndDrop::DroppedFile { window, path_buf } => {
                if let Some(extension) = path_buf.extension() {
                    if extension == "mid" {
                        if let Some(path) = path_buf.to_str() {
                            main_struct.midi_hub.stop();
                            main_struct.midi_seq = Some(MidiSequence::from_file(&path));
                            main_struct.midi_hub.start(&main_struct.midi_seq.as_ref().unwrap());
                            // todo reset state
                            // todo dredge out receiver
                        }
                    }
                }
            }
            _ => (),
        }
    }
}

fn main() {
    App::new()
        //todo need to set nearest neighbor?
        .add_plugins(DefaultPlugins.set(
            WindowPlugin {
                primary_window: Some(Window {
                    resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                    resizable: false,
                    ..default()
                }),
                ..default()
            }
        ).set(
            ImagePlugin::default_nearest()
        ))
        .insert_non_send(MainStruct::new())
        .add_systems(Startup, spawn_camera_system)
        .add_systems(Startup, spawn_graphics_system)
        .add_systems(Update, update_state_system)
        .add_systems(Update, file_drop_system)
        .run();

    println!("end of main!");
}