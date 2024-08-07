use crate::{Blue, Green, Red, Seconds, X, Y};
use macroquad::{
    color::Color,
    window::{screen_height, screen_width},
};
use serde::{Deserialize, Serialize};

/// An internally used type for keeping track of when to start an effect
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum EffectTimeTarget {
    Start(Seconds),
    End(Seconds),
}

/// Represents the direction to slide from/to for the slide animation effects
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SlideDirection {
    Left,
    Right,
    Top,
    Bottom,
    Custom(X, Y),
}

impl SlideDirection {
    /// Returns the position to start/end for the slide effects
    pub fn get_slide_target_position(
        direction: &SlideDirection,
        x_pos: X,
        y_pos: Y,
        tile_width: f32,
        tile_height: f32,
    ) -> (X, Y) {
        match direction {
            SlideDirection::Left => (-tile_width, y_pos),
            SlideDirection::Right => (screen_width(), y_pos),
            SlideDirection::Top => (x_pos, -tile_height),
            SlideDirection::Bottom => (x_pos, screen_height()),
            SlideDirection::Custom(custom_x, custom_y) => (*custom_x, *custom_y),
        }
    }
}

/// A basic color color struct which is fully serializable, and allows specifying an rgb without alpha (important for effects that apply)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EffectColor {
    Red,
    Green,
    Blue,
    Yellow,
    Magenta,
    Cyan,
    White,
    Black,
    Custom(f32, f32, f32),
}

impl EffectColor {
    /// Converts the EffectColor to macroquad's Color struct
    pub fn to_color(&self) -> Color {
        match self {
            EffectColor::Red => Color::new(1.0, 0.0, 0.0, 1.0),
            EffectColor::Green => Color::new(0.0, 1.0, 0.0, 1.0),
            EffectColor::Blue => Color::new(0.0, 0.0, 1.0, 1.0),
            EffectColor::Yellow => Color::new(1.0, 1.0, 0.0, 1.0),
            EffectColor::Magenta => Color::new(1.0, 0.0, 1.0, 1.0),
            EffectColor::Cyan => Color::new(0.0, 1.0, 1.0, 1.0),
            EffectColor::White => Color::new(1.0, 1.0, 1.0, 1.0),
            EffectColor::Black => Color::new(0.0, 0.0, 0.0, 1.0),
            EffectColor::Custom(r, g, b) => Color::new(*r, *g, *b, 1.0),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FlipDirection {
    Horizontal,
    Vertical,
}
