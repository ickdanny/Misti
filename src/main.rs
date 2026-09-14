
use std::sync::mpsc::{Sender, Receiver, channel, TryRecvError};
use std::sync::OnceLock;

use bevy::prelude::*;

mod mokyo_midi;
mod midi_state;
mod midi_short_msg;
mod main_struct;
mod config;
mod layout;

use crate::mokyo_midi::*;
use crate::midi_state::*;
use crate::main_struct::*;
use crate::config::*;
use crate::layout::*;

/* used for callback from C to Rust */
static SHORT_MSG_SENDER: OnceLock<Sender<u32>> = OnceLock::new();

extern "C" fn short_msg_callback(a: u32) {
    match SHORT_MSG_SENDER.get() {
        Some(sender) => {
            match sender.send(a) {
                Ok(_) => {
                    // do nothing
                },
                Err(err) => {
                    println!("{:?}", err);
                },
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

impl MainStruct {
    fn new(midi_out_index: i32) -> Self {
        let midi_state = MidiState::new();
        let short_msg_receiver = init_channel().unwrap();
        let midi_hub = MidiHub::new(midi_out_index, short_msg_callback);
        Self {
            midi_state,
            short_msg_receiver,
            midi_hub,
            midi_seq: None,
            file_name: None,
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



fn update_state_system(
    mut main_struct: NonSendMut<MainStruct>,
    config: Res<Config>,
) {
    loop {
        match main_struct.short_msg_receiver.try_recv() {
            Ok(data) => {
                main_struct.midi_state.update(
                    data,
                    &config.inst_names,
                    &config.drum_names,
                );
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

fn clear_short_msg_receiver(short_msg_receiver: &Receiver<u32>) {
    loop {
        match short_msg_receiver.try_recv() {
            Ok(_) => (),
            Err(_) => {
                break;
            },
        }
    }
}

fn file_drop_system(
    mut events: MessageReader<FileDragAndDrop>,
    mut main_struct: NonSendMut<MainStruct>,
) {
    for event in events.read() {
        match event {
            FileDragAndDrop::DroppedFile { window: _, path_buf } => {
                if let Some(extension) = path_buf.extension() {
                    if extension == "mid" {
                        if let Some(path) = path_buf.to_str() {
                            main_struct.midi_hub.stop();
                            main_struct.midi_seq = Some(MidiSequence::from_file(&path));
                            main_struct.file_name = Some(path_buf.file_name().unwrap().to_str().unwrap().into());
                            main_struct.midi_hub.start(&main_struct.midi_seq.as_ref().unwrap());
                            main_struct.midi_state.reset();
                            clear_short_msg_receiver(&main_struct.short_msg_receiver);
                        }
                    }
                }
            }
            _ => (),
        }
    }
}

fn main() {
    let config = read_config();
    App::new()
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
        .insert_non_send(MainStruct::new(config.midi_out_index))
        .insert_resource(ClearColor(Color::srgb(
            51.0/255.0,
            47.0/255.0,
            53.0/255.0)))
        .insert_resource(config)
        .add_systems(Startup, spawn_camera_system)
        .add_systems(Startup, spawn_graphics_system)
        .add_systems(Update, update_state_system)
        .add_systems(Update, key_update_system)
        .add_systems(Update, pitchbend_move_system)
        .add_systems(Update, file_drop_system)
        .add_systems(Update, text_color_update_system)
        .add_systems(Update, numeric_text_update_system)
        .add_systems(Update, inst_name_display_system)
        .add_systems(Update, file_name_display_system)
        .run();
}