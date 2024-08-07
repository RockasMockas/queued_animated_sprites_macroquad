use crate::{AnimationEffect, AnimationEffectTrait, X, Y};
use macroquad::color::Color;
use macroquad::texture::{draw_texture_ex, DrawTextureParams, Image, Texture2D};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::sync::Arc;

#[cfg(feature = "custom_effects")]
pub struct CustomEffect {
    effect: Arc<
        dyn Fn(f32, &mut Color, &mut DrawTextureParams, &mut X, &mut Y, f32, f32) + Send + Sync,
    >,
}

#[cfg(feature = "custom_effects")]
impl Clone for CustomEffect {
    fn clone(&self) -> Self {
        CustomEffect {
            effect: Arc::clone(&self.effect),
        }
    }
}

#[cfg(feature = "custom_effects")]
impl Debug for CustomEffect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CustomEffect")
    }
}

#[cfg(feature = "custom_effects")]
impl AnimationEffectTrait for CustomEffect {
    fn apply(
        &self,
        progress: f32,
        color: &mut Color,
        params: &mut DrawTextureParams,
        x_pos: &mut X,
        y_pos: &mut Y,
        tile_width: f32,
        tile_height: f32,
    ) {
        (self.effect)(
            progress,
            color,
            params,
            x_pos,
            y_pos,
            tile_width,
            tile_height,
        );
    }

    fn clone_box(&self) -> Box<dyn AnimationEffectTrait> {
        Box::new(self.clone())
    }
}

#[cfg(feature = "custom_effects")]
impl AnimationEffect {
    /// Creates a new custom animation efefct
    #[cfg(feature = "custom_effects")]
    pub fn new_custom<F>(f: F) -> AnimationEffect
    where
        F: Fn(f32, &mut Color, &mut DrawTextureParams, &mut X, &mut Y, f32, f32)
            + Send
            + Sync
            + 'static,
    {
        AnimationEffect::Custom(Box::new(CustomEffect {
            effect: Arc::new(f),
        }))
    }
}
