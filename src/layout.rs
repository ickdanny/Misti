/*
 * This file describes the graphical layout of the
 * screen.
 *
 * Author: ickdanny
 */

// TODO: stop looping

use either::*;
use bevy::prelude::*;
use bevy::sprite::*;

use crate::main_struct::*;
use crate::config::*;

pub const GAME_WIDTH: u32 = 840;
pub const GAME_HEIGHT: u32 = 380;
pub const PIXEL_RATIO: u32 = 2;
pub const WINDOW_WIDTH: u32 = GAME_WIDTH * PIXEL_RATIO;
pub const WINDOW_HEIGHT: u32 = GAME_HEIGHT * PIXEL_RATIO;

const KEY_INIT_X: i32 = 305;
const KEY_INIT_Y: i32 = -30;
const KEY_X_INC: i32 = 7;
const KEY_Y_INC: i32 = -20;
const BLACK_KEY_X_OFFSET: i32 = -(KEY_X_INC/2) + 1;

const TEXT_INIT_X: i32 = 77;
const TEXT_X_INC: i32 = 25;
const TEXT_Y_OFFSET: i32 = -3;

const INST_NAME_X: i32 = 9;

const HEADER_Y: i32 = -5;

const FOOTER_Y: i32 = -362;

const LOGO_X: i32 = (GAME_WIDTH - 71) as i32;
const LOGO_Y: i32 = HEADER_Y + 1;

const TEMPO_X: i32 = KEY_INIT_X + 7;
const TIME_SIG_X: i32 = 382;

const VERSION_STR_X: i32 = (GAME_WIDTH - 36) as i32;

const VERSION_STR: &str = concat!("v", env!("CARGO_PKG_VERSION"));

#[derive(Resource)]
pub struct Sprites {
    white_off: Handle<Image>,
    black_off: Handle<Image>,
    white_l_on: Handle<Image>,
    white_r_on: Handle<Image>,
    white_lr_on: Handle<Image>,
    black_on: Handle<Image>,
    horizontal_divider: Handle<Image>,
    logo: Handle<Image>,
}

#[derive(Component)]
pub struct BasePosition {
    x: f32,
}

impl BasePosition {
    fn new(x: f32) -> Self {
        Self { x }
    }
}

#[derive(Component)]
pub struct MoveWithPitchbend;

#[derive(Component)]
pub struct Channel(u8);

#[derive(Component)]
pub struct Key {
    note_num: u8,
}

#[derive(Component, PartialEq)]
pub enum NumericTextInterest {
    Program,
    Bank,
    Pan,
    MainVol,
    Expression,
    Modulation,
    Reverb,
    Chorus,
    Delay,
}

#[derive(Component)]
pub struct InstNameDisplay;

#[derive(Component)]
pub struct FileNameDisplay;

#[derive(Component)]
pub struct TempoDisplay;

#[derive(Component)]
pub struct TimeSigDisplay;

impl Sprites {
    fn load(asset_server: &Res<AssetServer>) -> Self {
        Self {
            white_off: asset_server.load("white_off.png"),
            black_off: asset_server.load("black_off.png"),
            white_l_on: asset_server.load("white_l_on.png"),
            white_r_on: asset_server.load("white_r_on.png"),
            white_lr_on: asset_server.load("white_lr_on.png"),
            black_on: asset_server.load("black_on.png"),
            horizontal_divider: asset_server.load("horizontal_divider.png"),
            logo: asset_server.load("logo.png"),
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
) {
    commands.spawn((
        Sprite{
            image: image.clone(),
            ..default()
        },
        Transform::from_xyz(x as f32, y as f32, z as f32),
        Anchor::TOP_LEFT,
        Channel(channel),
        Key{
            note_num,
        },
        BasePosition::new(x as f32),
        MoveWithPitchbend{},
        Visibility::Hidden,
    ));
}

fn spawn_static_text(
    commands: &mut Commands,
    font: &Handle<Font>,
    text: &str,
    color: &Srgba,
    x: i32,
    y: i32,
    z: i32,
) {
    commands.spawn((
        Text2d::new(text),
        Transform::from_xyz(x as f32, y as f32, z as f32),
        TextFont {
            font: FontSource::Handle(font.clone()),
            font_size: FontSize::Px(12.0),
            font_smoothing: FontSmoothing::None,
            ..default()
        },
        TextColor((*color).into()),
        Anchor::TOP_LEFT,
    ));
}

fn spawn_file_name_text(
    commands: &mut Commands,
    font: &Handle<Font>,
    text: &str,
    color: &Srgba,
    x: i32,
    y: i32,
    z: i32,
) {
    commands.spawn((
        Text2d::new(text),
        Transform::from_xyz(x as f32, y as f32, z as f32),
        TextFont {
            font: FontSource::Handle(font.clone()),
            font_size: FontSize::Px(12.0),
            font_smoothing: FontSmoothing::None,
            ..default()
        },
        TextColor((*color).into()),
        Anchor::TOP_LEFT,
        FileNameDisplay{},
    ));
}

fn spawn_tempo_text(
    commands: &mut Commands,
    font: &Handle<Font>,
    text: &str,
    color: &Srgba,
    x: i32,
    y: i32,
    z: i32,
) {
    commands.spawn((
        Text2d::new(text),
        Transform::from_xyz(x as f32, y as f32, z as f32),
        TextFont {
            font: FontSource::Handle(font.clone()),
            font_size: FontSize::Px(12.0),
            font_smoothing: FontSmoothing::None,
            ..default()
        },
        TextColor((*color).into()),
        Anchor::TOP_LEFT,
        TempoDisplay{},
    ));
}

fn spawn_time_sig_text(
    commands: &mut Commands,
    font: &Handle<Font>,
    text: &str,
    color: &Srgba,
    x: i32,
    y: i32,
    z: i32,
) {
    commands.spawn((
        Text2d::new(text),
        Transform::from_xyz(x as f32, y as f32, z as f32),
        TextFont {
            font: FontSource::Handle(font.clone()),
            font_size: FontSize::Px(12.0),
            font_smoothing: FontSmoothing::None,
            ..default()
        },
        TextColor((*color).into()),
        Anchor::TOP_LEFT,
        TimeSigDisplay{},
    ));
}

fn spawn_text(
    commands: &mut Commands,
    font: &Handle<Font>,
    text: &str,
    color: &Srgba,
    x: i32,
    y: i32,
    z: i32,
    channel: u8,
    interest: Either<NumericTextInterest, InstNameDisplay>,
) {
    match interest {
        Right(_) => {
            commands.spawn((
                Text2d::new(text),
                Transform::from_xyz(x as f32, y as f32, z as f32),
                TextFont {
                    font: FontSource::Handle(font.clone()),
                    font_size: FontSize::Px(12.0),
                    font_smoothing: FontSmoothing::None,
                    ..default()
                },
                TextColor((*color).into()),
                Anchor::TOP_LEFT,
                Channel(channel),
                InstNameDisplay{},
            ));
        },
        Left(interest) => {
            commands.spawn((
                Text2d::new(text),
                Transform::from_xyz(x as f32, y as f32, z as f32),
                TextFont {
                    font: FontSource::Handle(font.clone()),
                    font_size: FontSize::Px(12.0),
                    font_smoothing: FontSmoothing::None,
                    ..default()
                },
                TextColor((*color).into()),
                Anchor::TOP_LEFT,
                Channel(channel),
                interest,
            ));
        }
    }
}

#[derive(PartialEq)]
enum KeyType {
    Black,
    WhiteL,
    WhiteR,
    WhiteLR,
}

fn get_key_type(note_num: i32) -> KeyType {
    match note_num % 12 {
        0 => KeyType::WhiteR,
        1 => KeyType::Black,
        2 => KeyType::WhiteLR,
        3 => KeyType::Black,
        4 => KeyType::WhiteL,
        5 => KeyType::WhiteR,
        6 => KeyType::Black,
        7 => KeyType::WhiteLR,
        8 => KeyType::Black,
        9 => KeyType::WhiteLR,
        10 => KeyType::Black,
        11 => KeyType::WhiteL,
        _ => panic!("impossible"),
    }
}

fn spawn_graphics(
    commands: &mut Commands,
    sprites: &Sprites,
    font: Handle<Font>,
    config: Res<Config>,
) {
    let white_off = &sprites.white_off;
    let black_off = &sprites.black_off;
    let white_l_on = &sprites.white_l_on;
    let white_r_on = &sprites.white_r_on;
    let white_lr_on = &sprites.white_lr_on;
    let black_on = &sprites.black_on;
    let horizontal_divider = &sprites.horizontal_divider;
    let logo = &sprites.logo;

    let color = config.program_group_colors[15];
    let color = &color;

    // spawn graphics
    spawn_graphic(
        commands,
        logo,
        LOGO_X,
        LOGO_Y,
        10
    );

    // spawn dividers
    spawn_graphic(
        commands,
        horizontal_divider,
        0,
        HEADER_Y - 16,
        0
    );
    spawn_graphic(
        commands,
        horizontal_divider,
        0,
        FOOTER_Y + 3,
        0
    );

    // spawn header
    spawn_static_text(
        commands,
        &font,
        "INST",
        color,
        INST_NAME_X,
        HEADER_Y,
        10,
    );

    let mut x = TEXT_INIT_X;
    spawn_static_text(
        commands,
        &font,
        "PRG",
        color,
        x,
        HEADER_Y,
        10,
    );
    x += TEXT_X_INC;
    spawn_static_text(
        commands,
        &font,
        "BNK",
        color,
        x,
        HEADER_Y,
        10,
    );
    x += TEXT_X_INC;
    spawn_static_text(
        commands,
        &font,
        "PAN",
        color,
        x,
        HEADER_Y,
        10,
    );
    x += TEXT_X_INC;
    spawn_static_text(
        commands,
        &font,
        "VOL",
        color,
        x,
        HEADER_Y,
        10,
    );
    x += TEXT_X_INC;
    spawn_static_text(
        commands,
        &font,
        "EXP",
        color,
        x,
        HEADER_Y,
        10,
    );
    x += TEXT_X_INC;
    spawn_static_text(
        commands,
        &font,
        "MOD",
        color,
        x,
        HEADER_Y,
        10,
    );
    x += TEXT_X_INC;
    spawn_static_text(
        commands,
        &font,
        "RVB",
        color,
        x,
        HEADER_Y,
        10,
    );
    x += TEXT_X_INC;
    spawn_static_text(
        commands,
        &font,
        "CHS",
        color,
        x,
        HEADER_Y,
        10,
    );
    x += TEXT_X_INC;
    spawn_static_text(
        commands,
        &font,
        "DLY",
        color,
        x,
        HEADER_Y,
        10,
    );

    spawn_tempo_text(
        commands,
        &font,
        "Tempo: - - -",
        color,
        TEMPO_X,
        HEADER_Y,
        10,
    );

    spawn_time_sig_text(
        commands,
        &font,
        "Time Sig: - - -",
        color,
        TIME_SIG_X,
        HEADER_Y,
        10,
    );

    let mut y = KEY_INIT_Y;
    for channel in 0..16 {
        // spawn keys
        let mut x = KEY_INIT_X;
        for note_num in 0..128 {
            let key_type = get_key_type(note_num);
            if key_type == KeyType::Black {
                spawn_graphic(
                    commands,
                    black_off,
                    x + BLACK_KEY_X_OFFSET,
                    y,
                    5,
                );
                spawn_key(
                    commands,
                    black_on,
                    x + BLACK_KEY_X_OFFSET,
                    y,
                    10,
                    channel as u8,
                    note_num as u8,
                );
            }
            else {
                spawn_graphic(
                    commands,
                    white_off,
                    x,
                    y,
                    -5,
                );
                let sprite = match key_type {
                    KeyType::WhiteL => white_l_on,
                    KeyType::WhiteR => white_r_on,
                    KeyType::WhiteLR => white_lr_on,
                    _ => panic!("impossible"),
                };
                spawn_key(
                    commands,
                    sprite,
                    x,
                    y,
                    10,
                    channel as u8,
                    note_num as u8,
                );
                x += KEY_X_INC;
            }
        } // end loop for note num

        // spawn text
        let text_y = y + TEXT_Y_OFFSET;

        spawn_text(
            commands,
            &font,
            "",
            color,
            INST_NAME_X,
            text_y,
            10,
            channel as u8,
            Right(InstNameDisplay{})
        );

        let mut x = TEXT_INIT_X;
        spawn_text(
            commands,
            &font,
            "",
            color,
            x,
            text_y,
            10,
            channel as u8,
            Left(NumericTextInterest::Program)
        );
        x += TEXT_X_INC;
        spawn_text(
            commands,
            &font,
            "",
            color,
            x,
            text_y,
            10,
            channel as u8,
            Left(NumericTextInterest::Bank)
        );
        x += TEXT_X_INC;
        spawn_text(
            commands,
            &font,
            "",
            color,
            x,
            text_y,
            10,
            channel as u8,
            Left(NumericTextInterest::Pan)
        );
        x += TEXT_X_INC;
        spawn_text(
            commands,
            &font,
            "",
            color,
            x,
            text_y,
            10,
            channel as u8,
            Left(NumericTextInterest::MainVol)
        );
        x += TEXT_X_INC;
        spawn_text(
            commands,
            &font,
            "",
            color,
            x,
            text_y,
            10,
            channel as u8,
            Left(NumericTextInterest::Expression)
        );
        x += TEXT_X_INC;
        spawn_text(
            commands,
            &font,
            "",
            color,
            x,
            text_y,
            10,
            channel as u8,
            Left(NumericTextInterest::Modulation)
        );
        x += TEXT_X_INC;
        spawn_text(
            commands,
            &font,
            "",
            color,
            x,
            text_y,
            10,
            channel as u8,
            Left(NumericTextInterest::Reverb)
        );
        x += TEXT_X_INC;
        spawn_text(
            commands,
            &font,
            "",
            color,
            x,
            text_y,
            10,
            channel as u8,
            Left(NumericTextInterest::Chorus)
        );
        x += TEXT_X_INC;
        spawn_text(
            commands,
            &font,
            "",
            color,
            x,
            text_y,
            10,
            channel as u8,
            Left(NumericTextInterest::Delay)
        );

        y += KEY_Y_INC;
    } // end loop for channel

    // spawn footer
    spawn_file_name_text(
        commands,
        &font,
        "",
        color,
        INST_NAME_X,
        FOOTER_Y,
        10,
    );
    spawn_static_text(
        commands,
        &font,
        VERSION_STR,
        color,
        VERSION_STR_X,
        FOOTER_Y,
        10,
    );
}

pub fn spawn_graphics_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    config: Res<Config>,
) {
    let sprites = Sprites::load(&asset_server);
    let font = asset_server.load("Pixelify_Sans/PixelifySans-VariableFont_wght.ttf");
    
    spawn_graphics(&mut commands, &sprites, font, config);

    commands.insert_resource(sprites);
}

pub fn key_update_system(
    mut query: Query<(&mut Sprite, &mut Visibility, &Key, &Channel)>,
    mut main_struct: NonSendMut<MainStruct>,
    config: Res<Config>,
) {
    let midi_state = &mut main_struct.midi_state;
    for (mut sprite, mut visibility, key, channel) in &mut query {
        let channel = channel.0;
        let channel_state
            = &mut midi_state.channel_states[channel as usize];
        let note_state
            = &mut channel_state.note_states[key.note_num as usize];
        if note_state.is_on() || note_state.on_flag {
            let color_index = if channel == 9 {
                15
            } else {
                channel_state.program / 8
            };

            *visibility = Visibility::Visible;

            sprite.color = config.program_group_colors
                [color_index as usize].into();
            note_state.on_flag = false;
        } else { // note is off
            *visibility = Visibility::Hidden;
        }

        // let alpha: f32 = note_state.velocity as f32 / 128.0;
        // sprite.color = sprite.color.with_alpha(alpha);
    }
}

pub fn pitchbend_move_system(
    mut query: Query<(&mut Transform,  &Channel, &BasePosition), With<MoveWithPitchbend>>,
    main_struct: NonSendMut<MainStruct>,
) {
    let midi_state = &main_struct.midi_state;
    for (mut transform, channel, base_position) in &mut query {
        let channel = channel.0;
        let channel_state
            = &midi_state.channel_states[channel as usize];
        let pitchbend_sens = channel_state.pitchbend_sens as f32;
        let pitchbend_amount = channel_state.pitchbend_amount as f32;
        let base_x = base_position.x;
        let semitones = pitchbend_sens * (pitchbend_amount / 8192.0);
        let x_offset = KEY_X_INC as f32 * semitones / 2.0;
        // round to keep position to pixel grid
        let x_offset = x_offset.round();
        transform.translation.x = base_x + x_offset;
    }
}

pub fn text_color_update_system(
    mut query: Query<(&mut TextColor, &Channel)>,
    main_struct: NonSendMut<MainStruct>,
    config: Res<Config>,
) {
    let midi_state = &main_struct.midi_state;
    for (mut text_color, channel) in &mut query {
        let channel = channel.0;
        let channel_state
            = &midi_state.channel_states[channel as usize];
        let color_index = if channel == 9 {
            15
        } else {
            channel_state.program / 8
        };

        *text_color = config.program_group_colors
            [color_index as usize].into();
    }
}

pub fn numeric_text_update_system(
    mut query: Query<(&mut Text2d, &Channel, &NumericTextInterest)>,
    main_struct: NonSendMut<MainStruct>,
) {
    let midi_state = &main_struct.midi_state;
    for (mut text2d, channel, interest) in &mut query {
        let channel = channel.0;
        let channel_state
            = &midi_state.channel_states[channel as usize];

        let num = match interest {
            NumericTextInterest::Program => channel_state.program,
            NumericTextInterest::Bank => channel_state.bank,
            NumericTextInterest::Pan => channel_state.pan,
            NumericTextInterest::MainVol => channel_state.main_vol,
            NumericTextInterest::Expression => channel_state.expression,
            NumericTextInterest::Modulation => channel_state.modulation,
            NumericTextInterest::Reverb => channel_state.reverb,
            NumericTextInterest::Chorus => channel_state.chorus,
            NumericTextInterest::Delay => channel_state.delay,
        };

        if *interest != NumericTextInterest::Pan {
            *text2d = Text2d::new(format!("{:3}", num));
        } else {
            if num == 64 {
                *text2d = Text2d::new(format!("{:3}", 0));
            } else if num < 64 {
                let corrected_num = 64 - num;
                *text2d = Text2d::new(format!("L{:2}", corrected_num));
            } else {
                let corrected_num = num - 64;
                *text2d = Text2d::new(format!("R{:2}", corrected_num));
            }
        }
    }
}

pub fn inst_name_display_system(
    mut query: Query<(&mut Text2d, &Channel), With<InstNameDisplay>>,
    main_struct: NonSendMut<MainStruct>,
) {
    let midi_state = &main_struct.midi_state;
    for (mut text2d, channel) in &mut query {
        let channel = channel.0;
        let channel_state
            = &midi_state.channel_states[channel as usize];

        *text2d = Text2d::new(format!("{:.10}", channel_state.inst_name));
    }
}

pub fn file_name_display_system(
    mut query: Query<&mut Text2d, With<FileNameDisplay>>,
    main_struct: NonSendMut<MainStruct>,
) {
    for mut text2d in &mut query {
        match main_struct.file_name {
            None => {
                *text2d = Text2d::new(format!("Now playing: - - -"));
            },
            Some(ref file_name) => {
                *text2d = Text2d::new(format!("Now playing: {}", file_name));
            }
        }
    }
}

pub fn tempo_display_system(
    mut query: Query<&mut Text2d, With<TempoDisplay>>,
    main_struct: NonSendMut<MainStruct>,
) {
    for mut text2d in &mut query {
        let tempo_us_per_beat
            = main_struct.midi_state.tempo_us_per_beat;
        if tempo_us_per_beat > 0 {
            let bpm = (60 * 1000000) / tempo_us_per_beat;
            *text2d = Text2d::new(format!("Tempo: {}", bpm));
        } else {
            *text2d = Text2d::new(format!("Tempo: - - -"));
        }
    }
}

pub fn time_sig_display_system(
    mut query: Query<&mut Text2d, With<TimeSigDisplay>>,
    main_struct: NonSendMut<MainStruct>,
) {
    for mut text2d in &mut query {
        let (numerator, denominator_pow)
            = main_struct.midi_state.time_sig;
        if numerator > 0 {
            *text2d = Text2d::new(format!(
                "Time Sig: {}/{}",
                numerator,
                2_i32.pow(denominator_pow.into()))
            );
        } else {
            *text2d = Text2d::new(format!("Time Sig: - - -"));
        }
    }
}