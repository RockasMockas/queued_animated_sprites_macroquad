use crate::{EffectColor, FlipDirection, SlideDirection, X, Y};
use macroquad::prelude::*;
use macroquad::texture::DrawTextureParams;
use macroquad::{color::Color, rand::rand};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

/// An internal trait used for allowing custom animation effects to be possible
pub trait AnimationEffectTrait: Debug + Send + Sync {
    fn apply(
        &self,
        progress: f32,
        color: &mut Color,
        params: &mut DrawTextureParams,
        x_pos: &mut X,
        y_pos: &mut Y,
        tile_width: f32,
        tile_height: f32,
    );
    fn clone_box(&self) -> Box<dyn AnimationEffectTrait>;
}

/// AnimationEffects provide a variety of baked-in options for enhancing how your AnimatedSprite is drawn.
#[derive(Debug, Serialize, Deserialize)]
pub enum AnimationEffect {
    FadeIn,
    FadeOut,
    SlideIn(SlideDirection),
    SlideOut(SlideDirection),
    Spin,
    /// Maximum size to grow during pulse, 1.0 = 100%
    Pulse(f32),
    /// EffectColor to blink, number of blinks
    Blinking(EffectColor, u32),
    /// Intensity of the shake
    Shake(f32),
    /// Intensity of the wobble
    Wobble(f32),
    /// Height of the bounce, Number of bounces
    Bounce(f32, u32),
    BasicFlip(FlipDirection), // Direction to flip
    // Glitch intensity
    Glitch(f32),
    // Intensity
    ShearLeft(f32),
    // Intensity
    ShearRight(f32),
    /// Intensity of the squash
    SquashFlipVertical(f32),
    /// Intensity of the squash
    SquashFlipHorizontal(f32),
    /// New effect colors
    ColorCycle(Vec<EffectColor>),
    #[cfg(feature = "custom_effects")]
    #[serde(skip)]
    Custom(Box<dyn AnimationEffectTrait>),
}

impl Clone for AnimationEffect {
    fn clone(&self) -> Self {
        match self {
            AnimationEffect::FadeIn => AnimationEffect::FadeIn,
            AnimationEffect::FadeOut => AnimationEffect::FadeOut,
            AnimationEffect::SlideIn(from) => AnimationEffect::SlideIn(from.clone()),
            AnimationEffect::SlideOut(from) => AnimationEffect::SlideOut(from.clone()),
            AnimationEffect::Spin => AnimationEffect::Spin,
            AnimationEffect::Pulse(scale) => AnimationEffect::Pulse(*scale),
            AnimationEffect::Blinking(color, blinks) => {
                AnimationEffect::Blinking(color.clone(), *blinks)
            }
            AnimationEffect::Shake(intensity) => AnimationEffect::Shake(*intensity),
            AnimationEffect::Wobble(intensity) => AnimationEffect::Wobble(*intensity),
            AnimationEffect::Bounce(height, bounces) => AnimationEffect::Bounce(*height, *bounces),
            AnimationEffect::BasicFlip(direction) => AnimationEffect::BasicFlip(direction.clone()),
            AnimationEffect::Glitch(intensity) => AnimationEffect::Glitch(*intensity),
            AnimationEffect::ShearLeft(intensity) => AnimationEffect::ShearLeft(*intensity),
            AnimationEffect::ShearRight(intensity) => AnimationEffect::ShearRight(*intensity),
            AnimationEffect::SquashFlipVertical(scale) => {
                AnimationEffect::SquashFlipVertical(*scale)
            }
            AnimationEffect::SquashFlipHorizontal(scale) => {
                AnimationEffect::SquashFlipHorizontal(*scale)
            }
            AnimationEffect::ColorCycle(colors) => AnimationEffect::ColorCycle(colors.clone()),
            #[cfg(feature = "custom_effects")]
            AnimationEffect::Custom(effect) => AnimationEffect::Custom(effect.clone_box()),
        }
    }
}

impl AnimationEffectTrait for AnimationEffect {
    /// Clones the current AnimationEffect as a Box<dyn AnimationEffectTrait>
    fn clone_box(&self) -> Box<dyn AnimationEffectTrait> {
        Box::new(self.clone())
    }

    /// Applies the current AnimationEffect to the given parameters
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
        match self {
            AnimationEffect::FadeIn => apply_fade_in(progress, color),
            AnimationEffect::FadeOut => apply_fade_out(progress, color),
            AnimationEffect::SlideIn(direction) => {
                apply_slide_in(progress, x_pos, y_pos, tile_width, tile_height, direction)
            }
            AnimationEffect::SlideOut(direction) => {
                apply_slide_out(progress, x_pos, y_pos, tile_width, tile_height, direction)
            }
            AnimationEffect::Spin => apply_spin(progress, params),
            AnimationEffect::Pulse(max_scale) => {
                apply_pulse(progress, params, x_pos, y_pos, *max_scale)
            }
            AnimationEffect::Blinking(blink_color, blinks) => {
                apply_blinking(progress, color, blink_color, *blinks)
            }
            AnimationEffect::Shake(intensity) => apply_shake(progress, x_pos, y_pos, *intensity),
            AnimationEffect::Wobble(intensity) => apply_wobble(progress, params, *intensity),
            AnimationEffect::Bounce(height, bounces) => {
                apply_bounce(progress, y_pos, *height, *bounces)
            }
            AnimationEffect::BasicFlip(direction) => apply_basic_flip(params, direction),
            AnimationEffect::Glitch(intensity) => {
                apply_glitch(progress, color, params, x_pos, y_pos, *intensity)
            }
            AnimationEffect::ShearLeft(intensity) => {
                apply_shear_left(progress, params, *intensity);
            }
            AnimationEffect::ShearRight(intensity) => {
                apply_shear_right(progress, params, x_pos, *intensity);
            }
            AnimationEffect::SquashFlipVertical(intensity) => {
                apply_squash_vertical(progress, params, y_pos, *intensity, tile_height)
            }
            AnimationEffect::SquashFlipHorizontal(intensity) => {
                apply_squash_horizontal(progress, params, x_pos, *intensity, tile_width)
            }
            AnimationEffect::ColorCycle(palette) => apply_color_cycle(progress, color, palette),
            #[cfg(feature = "custom_effects")]
            AnimationEffect::Custom(effect) => effect.apply(
                progress,
                color,
                params,
                x_pos,
                y_pos,
                tile_width,
                tile_height,
            ),
        }
    }
}

/// Applies the FadeIn effect
fn apply_fade_in(progress: f32, color: &mut Color) {
    color.a = progress;
}

/// Applies the FadeOut effect
fn apply_fade_out(progress: f32, color: &mut Color) {
    color.a = 1.0 - progress;
}

/// Applies the SlideIn effect
fn apply_slide_in(
    progress: f32,
    x_pos: &mut X,
    y_pos: &mut Y,
    tile_width: f32,
    tile_height: f32,
    direction: &SlideDirection,
) {
    let (start_x, start_y) = SlideDirection::get_slide_target_position(
        direction,
        *x_pos,
        *y_pos,
        tile_width,
        tile_height,
    );
    *x_pos = start_x + (*x_pos - start_x) * progress;
    *y_pos = start_y + (*y_pos - start_y) * progress;
}

/// Applies the SlideOut effect
fn apply_slide_out(
    progress: f32,
    x_pos: &mut X,
    y_pos: &mut Y,
    tile_width: f32,
    tile_height: f32,
    direction: &SlideDirection,
) {
    let (end_x, end_y) = SlideDirection::get_slide_target_position(
        direction,
        *x_pos,
        *y_pos,
        tile_width,
        tile_height,
    );
    *x_pos = *x_pos + (end_x - *x_pos) * progress;
    *y_pos = *y_pos + (end_y - *y_pos) * progress;
}

/// Applies the Spin effect
fn apply_spin(progress: f32, params: &mut DrawTextureParams) {
    let max_rotation = 10.0 * std::f32::consts::PI; // 5 full rotations
    let rotation = max_rotation * (1.0 - progress);
    params.rotation = rotation;
}

/// Applies the Pulse effect
fn apply_pulse(
    progress: f32,
    params: &mut DrawTextureParams,
    x_pos: &mut X,
    y_pos: &mut Y,
    max_scale: f32,
) {
    let scale = 1.0 + (max_scale - 1.0) * (2.0 * std::f32::consts::PI * progress).sin().abs();

    if let Some(mut size) = params.dest_size {
        let delta_width = size.x * (scale - 1.0);
        let delta_height = size.y * (scale - 1.0);

        *x_pos -= delta_width / 2.0;
        *y_pos -= delta_height / 2.0;

        size.x *= scale;
        size.y *= scale;
        params.dest_size = Some(size);
    }
}

/// Applies the Blinking effect
fn apply_blinking(progress: f32, color: &mut Color, blink_color: &EffectColor, blinks: u32) {
    let blink_duration = 1.0 / (blinks as f32);
    let blink_progress = (progress / blink_duration) % 1.0;

    // Adjust these values to control the sharpness and duration of the blink
    let rise_time = 0.1; // Time to transition to blink color
    let hold_time = 0.6; // Time to hold at blink color
    let fall_time = 0.3; // Time to transition back to original color

    let blink_intensity = if blink_progress < rise_time {
        // Sharp rise to blink color
        blink_progress / rise_time
    } else if blink_progress < rise_time + hold_time {
        // Hold at blink color
        1.0
    } else if blink_progress < rise_time + hold_time + fall_time {
        // Fall back to original color
        1.0 - (blink_progress - rise_time - hold_time) / fall_time
    } else {
        // Stay at original color for the remainder
        0.0
    };

    let target_color = blink_color.to_color();
    color.r = color.r * (1.0 - blink_intensity) + target_color.r * blink_intensity;
    color.g = color.g * (1.0 - blink_intensity) + target_color.g * blink_intensity;
    color.b = color.b * (1.0 - blink_intensity) + target_color.b * blink_intensity;
}

/// Applies the Shake effect
fn apply_shake(progress: f32, x_pos: &mut X, y_pos: &mut Y, intensity: f32) {
    let shake_amount = intensity * (1.0 - progress); // Decrease shake over time
    let angle = progress * std::f32::consts::PI * 10.0; // Arbitrary multiplier for quicker shaking
    *x_pos += shake_amount * angle.sin();
    *y_pos += shake_amount * angle.cos();
}

/// Applies the Wobble effect
fn apply_wobble(progress: f32, params: &mut DrawTextureParams, intensity: f32) {
    let wobble_amount = intensity * (1.0 - progress.powf(2.0)); // Decrease wobble over time
    let angle = progress * std::f32::consts::PI * 8.0; // Frequency of wobble

    // Calculate x and y offsets for wobble
    let offset_x = wobble_amount * angle.sin();
    let offset_y = wobble_amount * (angle * 2.0).sin(); // Double frequency for y to create more complex motion

    // Apply offsets to the sprite's position
    if let Some(mut dest_size) = params.dest_size {
        dest_size.x += offset_x;
        dest_size.y += offset_y;
        params.dest_size = Some(dest_size);
    }
}

/// Applies the Bounce effect
fn apply_bounce(progress: f32, y_pos: &mut Y, height: f32, bounces: u32) {
    let bounce_progress = (progress * std::f32::consts::PI * bounces as f32).sin();
    *y_pos -= height * bounce_progress.abs() * (1.0 - progress.powf(0.5)); // Adjust bounce decay
}

/// Applies the BasicFlip effect
fn apply_basic_flip(params: &mut DrawTextureParams, direction: &FlipDirection) {
    match direction {
        FlipDirection::Horizontal => {
            params.flip_x = !params.flip_x;
        }
        FlipDirection::Vertical => {
            params.flip_y = !params.flip_y;
        }
    }
}

/// Applies the Glitch effect
fn apply_glitch(
    progress: f32,
    color: &mut Color,
    params: &mut DrawTextureParams,
    x_pos: &mut X,
    y_pos: &mut Y,
    intensity: f32,
) {
    let glitch_amount = intensity * (1.0 - progress);

    if let Some(original_source) = params.source {
        // Number of horizontal and vertical divisions
        let h_divisions = (glitch_amount * 3.0).round().max(1.0) as i32;
        let v_divisions = (glitch_amount * 3.0).round().max(1.0) as i32;

        // Size of each piece
        let piece_width = original_source.w / h_divisions as f32;
        let piece_height = original_source.h / v_divisions as f32;

        // Create and shuffle pieces
        let mut pieces = Vec::new();
        for i in 0..h_divisions {
            for j in 0..v_divisions {
                let mut piece = Rect::new(
                    original_source.x + i as f32 * piece_width,
                    original_source.y + j as f32 * piece_height,
                    piece_width,
                    piece_height,
                );

                // Random offset for each piece
                let offset_x = rand::gen_range(-0.5, 0.5) * glitch_amount * piece_width;
                let offset_y = rand::gen_range(-0.5, 0.5) * glitch_amount * piece_height;

                piece.x += offset_x;
                piece.y += offset_y;

                pieces.push(piece);
            }
        }

        // Randomly select one piece to display
        if !pieces.is_empty() {
            let selected_piece = pieces[rand::gen_range(0, pieces.len())];
            params.source = Some(selected_piece);

            // Adjust the position to account for the piece's offset
            *x_pos += selected_piece.x - original_source.x;
            *y_pos += selected_piece.y - original_source.y;
        }

        // Occasionally flip the texture
        if rand::gen_range(0.0, 1.0) < glitch_amount {
            params.flip_x = !params.flip_x;
        }
        if rand::gen_range(0.0, 1.0) < glitch_amount {
            params.flip_y = !params.flip_y;
        }

        // Apply random color glitch effect
        if rand::gen_range(0.0, 1.0) < glitch_amount * 0.5 {
            // Randomly choose between color shift and color inversion
            if rand::gen_range(0.0, 1.0) < 0.5 {
                // Color shift
                let random_color = Color::new(
                    rand::gen_range(0.0, 1.0),
                    rand::gen_range(0.0, 1.0),
                    rand::gen_range(0.0, 1.0),
                    1.0,
                );
                let color_shift = rand::gen_range(0.3, 1.0) * glitch_amount;
                color.r = color.r * (1.0 - color_shift) + random_color.r * color_shift;
                color.g = color.g * (1.0 - color_shift) + random_color.g * color_shift;
                color.b = color.b * (1.0 - color_shift) + random_color.b * color_shift;
            } else {
                // Color inversion
                color.r = 1.0 - color.r;
                color.g = 1.0 - color.g;
                color.b = 1.0 - color.b;
            }
        }

        // Occasionally add color channel splitting
        if rand::gen_range(0.0, 1.0) < glitch_amount * 0.3 {
            let channel = rand::gen_range(0, 3);
            let shift = rand::gen_range(-0.2, 0.2) * glitch_amount;
            match channel {
                0 => color.r = (color.r + shift).clamp(0.0, 1.0),
                1 => color.g = (color.g + shift).clamp(0.0, 1.0),
                _ => color.b = (color.b + shift).clamp(0.0, 1.0),
            }
        }
    }
}

/// Applies the ShearRight effect
fn apply_shear_right(
    progress: f32,
    params: &mut DrawTextureParams,
    x_pos: &mut f32,
    intensity: f32,
) {
    let shear_amount = -intensity * (1.0 - progress) * 0.5; // Negative for left shear, max 50% shear
    if let Some(mut dest_size) = params.dest_size {
        let shear_x = dest_size.x * shear_amount.abs();
        params.rotation = shear_amount * std::f32::consts::PI / 4.0; // Rotate to create shear effect
        dest_size.x += shear_x; // Increase width to accommodate shear
        params.dest_size = Some(dest_size);

        // Adjust x position to keep the right edge of the sprite fixed
        *x_pos -= shear_x;
    }
}

/// Applies the ShearLeft effect
fn apply_shear_left(progress: f32, params: &mut DrawTextureParams, intensity: f32) {
    let shear_amount = intensity * (1.0 - progress) * 0.5; // Positive for right shear, max 50% shear
    if let Some(mut dest_size) = params.dest_size {
        let shear_x = dest_size.x * shear_amount;
        params.rotation = shear_amount * std::f32::consts::PI / 4.0; // Rotate to create shear effect
        dest_size.x += shear_x; // Increase width to accommodate shear
        params.dest_size = Some(dest_size);

        // No need to adjust x position for right shear, as the left edge remains fixed
    }
}

/// Applies the squash effect vertically
fn apply_squash_vertical(
    progress: f32,
    params: &mut DrawTextureParams,
    y_pos: &mut Y,
    intensity: f32,
    tile_height: f32,
) {
    let squash_amount = intensity * (1.0 - progress.powf(2.0)); // Decrease squash over time
    if let Some(mut dest_size) = params.dest_size {
        let original_height = dest_size.y;
        dest_size.y *= 1.0 - squash_amount;
        params.dest_size = Some(dest_size);

        // Adjust y position to keep the sprite centered
        *y_pos += (original_height - dest_size.y) / 2.0;
    }
}

/// Applies the squash effect horizontally
fn apply_squash_horizontal(
    progress: f32,
    params: &mut DrawTextureParams,
    x_pos: &mut X,
    intensity: f32,
    tile_width: f32,
) {
    let squash_amount = intensity * (1.0 - progress.powf(2.0)); // Decrease squash over time
    if let Some(mut dest_size) = params.dest_size {
        let original_width = dest_size.x;
        dest_size.x *= 1.0 - squash_amount;
        params.dest_size = Some(dest_size);

        // Adjust x position to keep the sprite centered
        *x_pos += (original_width - dest_size.x) / 2.0;
    }
}

/// Applies a color cycle effect
fn apply_color_cycle(progress: f32, color: &mut Color, palette: &[EffectColor]) {
    if palette.is_empty() {
        return;
    }
    let index = (progress * palette.len() as f32) as usize % palette.len();
    let next_index = (index + 1) % palette.len();
    let sub_progress = (progress * palette.len() as f32) % 1.0;

    let current_color = palette[index].to_color();
    let next_color = palette[next_index].to_color();

    *color = Color::new(
        lerp(current_color.r, next_color.r, sub_progress),
        lerp(current_color.g, next_color.g, sub_progress),
        lerp(current_color.b, next_color.b, sub_progress),
        color.a, // Preserve original alpha
    );
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}
