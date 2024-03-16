//this file will be a place where global constants for color are. in the future this will be edited to tweak the graphics not
//the actual gamecode
use crate::prelude::*;
///Used to represent the color of an entity when rendering, it allows the game to decide the color of the object
///dynamically depending on ingame lighting conditions.
pub struct DynamicColor {
    pub dark: (u8, u8, u8),
    pub dim: (u8, u8, u8),
    pub bright: (u8, u8, u8),
}

pub const DYN_BG: DynamicColor = DynamicColor {
    dark: (25, 25, 25),
    dim: (50, 50, 50),
    bright: (75, 75, 75),
};

pub const DYN_RED: DynamicColor = DynamicColor {
    dark: DARK_RED,
    dim: (190, 0, 0),
    bright: RED,
};

pub const DYN_BLUE: DynamicColor = DynamicColor {
    dark: (0, 0, 130),
    dim: (0, 0, 190),
    bright: (0, 0, 255),
};

pub const DYN_YELLOW: DynamicColor = DynamicColor {
    dark: (130, 130, 0),
    dim: (190, 190, 0),
    bright: (255, 255, 0),
};
pub const DYN_GREEN: DynamicColor = DynamicColor {
    dark: (0, 130, 0),
    dim: (0, 190, 0),
    bright: (0, 255, 0),
};
