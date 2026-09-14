/*
 * This file controls the reading of the config
 * file.
 *
 * Author: ickdanny
 */

use bevy::prelude::*;
use serde::Deserialize;

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
struct Colors {
    program_group_colors: [ColorRGB; 16],
}

#[derive(Debug, Resource)]
pub struct Config {
    pub program_group_colors: [Srgba; 16],
}

pub fn read_config() -> Config {
    let file_name = "config.toml";
    let contents = std::fs::read_to_string(file_name).unwrap();
    let colors: Colors = toml::from_str(&contents).unwrap();
    Config {
        program_group_colors:
            colors.program_group_colors.map(|color_rgb| {
                Srgba::from(color_rgb)
            }),
    }
}