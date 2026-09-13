/*
 * This file describes the graphical layout of the
 * screen.
 *
 * Author: ickdanny
 */

use bevy::prelude::*;
use bevy::sprite::*;

use crate::main_struct::*;

pub const GAME_WIDTH: u32 = 850;
pub const GAME_HEIGHT: u32 = 380;
pub const PIXEL_RATIO: u32 = 2;
pub const WINDOW_WIDTH: u32 = GAME_WIDTH * PIXEL_RATIO;
pub const WINDOW_HEIGHT: u32 = GAME_HEIGHT * PIXEL_RATIO;

const KEY_INIT_X: i32 = 300;
const KEY_INIT_Y: i32 = -30;
const KEY_X_INC: i32 = 7;
const KEY_Y_INC: i32 = -20;
const BLACK_KEY_X_OFFSET: i32 = -(KEY_X_INC/2) + 1;

#[derive(Resource)]
pub struct Sprites {
    white_off: Handle<Image>,
    black_off: Handle<Image>,
    white_ons: [Handle<Image>; 16],
    black_ons: [Handle<Image>; 16],
}

#[derive(Component)]
pub struct Key{
    channel: u8,
    note_num: u8,
    is_black: bool,
}

impl Sprites {
    fn load(asset_server: Res<AssetServer>) -> Self {
        Self {
            white_off: asset_server.load("white_off.png"),
            black_off: asset_server.load("black_off.png"),
            white_ons: std::array::from_fn(|i| {
                let file_name = format!("white_{}.png", i);
                asset_server.load(file_name)
            }),
            black_ons: std::array::from_fn(|i| {
                let file_name = format!("black_{}.png", i);
                asset_server.load(file_name)
            }),
        }
    }
}

fn spawn_graphic(
    commands: &mut Commands,
    image: &Handle<Image>,
    x: i32,
    y: i32,
    z: i32,
) {
    commands.spawn((
        Sprite{
            image: image.clone(),
            ..default()
        },
        Transform::from_xyz(x as f32, y as f32, z as f32),
        Anchor::TOP_LEFT,
    ));
}

fn spawn_key(
    commands: &mut Commands,
    image: &Handle<Image>,
    x: i32,
    y: i32,
    z: i32,
    channel: u8,
    note_num: u8,
    is_black: bool,
) {
    commands.spawn((
        Sprite{
            image: image.clone(),
            ..default()
        },
        Transform::from_xyz(x as f32, y as f32, z as f32),
        Anchor::TOP_LEFT,
        Key{
            channel,
            note_num,
            is_black,
        },
    ));
}

fn is_note_black(note_num: i32) -> bool {
    match note_num % 12 {
        1 => true,
        3 => true,
        6 => true,
        8 => true,
        10 => true,
        _ => false,
    }
}

fn spawn_keys(commands: &mut Commands, sprites: &Sprites) {
    let white_off = &sprites.white_off;
    let black_off = &sprites.black_off;

    let mut y = KEY_INIT_Y;
    for channel in 0..16 {
        let mut x = KEY_INIT_X;
        for note_num in 0..128 {
            if is_note_black(note_num) {
                spawn_graphic(
                    commands,
                    black_off,
                    x + BLACK_KEY_X_OFFSET,
                    y,
                    5,
                );
                spawn_key(
                    commands,
                    black_off,
                    x + BLACK_KEY_X_OFFSET,
                    y,
                    10,
                    channel as u8,
                    note_num as u8,
                    true,
                );
            } else {
                spawn_graphic(
                    commands,
                    white_off,
                    x,
                    y,
                    -5,
                );
                spawn_key(
                    commands,
                    white_off,
                    x,
                    y,
                    0, // has to be below black background
                    channel as u8,
                    note_num as u8,
                    false,
                );
                x += KEY_X_INC;
            }
        }
        y += KEY_Y_INC;
    }
}

pub fn spawn_graphics_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let sprites = Sprites::load(asset_server);

    spawn_keys(&mut commands, &sprites);

    commands.insert_resource(sprites);
}

pub fn key_update_system(
    mut query: Query<(&mut Sprite, &Key)>,
    main_struct: NonSendMut<MainStruct>,
    sprites: Res<Sprites>,
) {
    for (mut sprite, key) in &mut query {
        let midi_state = &main_struct.midi_state;
        let channel_state
            = &midi_state.channel_states[key.channel as usize];
        let note_state
            = &channel_state.note_states[key.note_num as usize];
        if note_state.on {
            let sprite_index = if key.channel == 9 {
                0
            } else {
                channel_state.program / 8
            };
            sprite.image = if key.is_black {
                sprites.black_ons[sprite_index as usize].clone()
            } else {
                sprites.white_ons[sprite_index as usize].clone()
            };
        } else {
            sprite.image = if key.is_black {
                sprites.black_off.clone()
            } else {
                sprites.white_off.clone()
            };
        }
    }
}