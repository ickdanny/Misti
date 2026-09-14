/*
 * This file controls the reading of the config
 * file.
 *
 * Author: ickdanny
 */

use bevy::prelude::*;
use serde::Deserialize;
use serde_big_array::BigArray;

#[derive(Debug, Deserialize)]
struct ColorRGB {
    r: u8,
    g: u8,
    b: u8,
}

impl From<ColorRGB> for Srgba {
    fn from(color_rgb: ColorRGB) -> Srgba {
        Srgba{
            red: color_rgb.r as f32 / 255.0,
            green: color_rgb.g as f32 / 255.0,
            blue: color_rgb.b as f32 / 255.0,
            alpha: 1.0,
        }
    }
}

#[derive(Debug, Deserialize)]
struct ReadInConfig {
    midi_out_index: i32,
    program_group_colors: [ColorRGB; 16],
    #[serde(with = "BigArray")]
    inst_names: [String; 128],
    #[serde(with = "BigArray")]
    drum_names: [String; 128],
}

#[derive(Debug, Resource)]
pub struct Config {
    pub midi_out_index: i32,
    pub program_group_colors: [Srgba; 16],
    pub inst_names: [String; 128],
    pub drum_names: [String; 128],
}

pub fn read_config() -> Config {
    let file_name = "config.toml";
    let contents = std::fs::read_to_string(file_name).unwrap();
    let read_in_config: ReadInConfig = toml::from_str(&contents).unwrap();
    Config {
        midi_out_index: read_in_config.midi_out_index,
        program_group_colors:
            read_in_config.program_group_colors.map(|color_rgb| {
                Srgba::from(color_rgb)
            }),
        inst_names: read_in_config.inst_names,
        drum_names: read_in_config.drum_names,
    }
}