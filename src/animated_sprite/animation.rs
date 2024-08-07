use serde::{Deserialize, Serialize};
use std::fmt::Debug;

use crate::{AnimationEffect, EffectTimeTarget, Seconds};

/// Represents one of the animations part of the spritesheet used by the AnimatedSprite.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Animation {
    pub rows: Vec<u32>,
    pub frames_per_row: u32,
    pub fps: u32,
    pub effect: Option<(AnimationEffect, EffectTimeTarget)>,
}

impl Animation {
    /// Create a new Animation from a single row on the spritesheet.
    pub fn new(row: u32, frames: u32, fps: u32) -> Self {
        Animation {
            rows: vec![row.max(0)],
            frames_per_row: frames.max(1),
            fps: fps.max(0),
            effect: None,
        }
    }

    /// Create a new Animation from multiple rows put together on the spritesheet.
    pub fn new_multi_row(rows: Vec<u32>, frames_per_row: u32, fps: u32) -> Self {
        Animation {
            rows: if rows.is_empty() { vec![1] } else { rows },
            frames_per_row: frames_per_row.max(1),
            fps: fps.max(0),
            effect: None,
        }
    }

    /// Add a start animation effect that begins at the start of the animation. Duration is represented in seconds from the start.
    pub fn with_start_effect(mut self, effect: AnimationEffect, duration: Seconds) -> Self {
        self.effect = Some((effect, EffectTimeTarget::Start(duration)));
        self
    }

    /// Add an end animation effect that ends with the animation.  Duration is represented in seconds from the end.
    pub fn with_end_effect(mut self, effect: AnimationEffect, duration: Seconds) -> Self {
        self.effect = Some((effect, EffectTimeTarget::End(duration)));
        self
    }

    /// Returns an empty Animation with row/frames/fps set to 0.
    /// This means this animation will draw nothing no matter what texture is provided to AnimatedSprite draw methods.
    /// Can be used in between other animations in the queue, or even set as default animation to guarantee nothing is drawn when queue is finished.
    pub fn empty() -> Self {
        Animation::new(0, 0, 0)
    }

    /// Calculates the row and frame based on the current frame.
    pub fn get_row_and_frame_and_fps(&self, current_frame: u32) -> (u32, u32, u32) {
        let total_frames = self.rows.len() as u32 * self.frames_per_row;
        if total_frames == 0 && self.fps == 0 {
            return (0, 0, 0);
        }

        let adjusted_frame = current_frame % total_frames;
        let row_index = (adjusted_frame / self.frames_per_row) as usize;
        let frame = adjusted_frame % self.frames_per_row;
        (self.rows[row_index], frame, self.fps)
    }

    /// Returns the total number of frames in the animation, accounting for all rows.
    pub fn total_frames(&self) -> u32 {
        self.rows.len() as u32 * self.frames_per_row
    }
}
