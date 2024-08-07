use serde::{Deserialize, Serialize};

use crate::EffectDuration;

/// A struct that holds the internal state related to processing AnimationEffects (for an AnimatedSprite )
#[derive(Serialize, Deserialize, Clone)]
pub struct InternalEffectsState {
    pub effect_time: EffectDuration,
    pub current_effect_duration: EffectDuration,
    pub effect_start_time: EffectDuration,
    pub is_active: bool,
    pub has_played: bool,
}

impl InternalEffectsState {
    /// Creates a new InternalEffectsState
    pub fn new() -> Self {
        InternalEffectsState {
            effect_time: 0.0,
            current_effect_duration: 0.0,
            effect_start_time: 0.0,
            is_active: false,
            has_played: false,
        }
    }

    /// Resets the state of the internal effects state
    pub fn reset(&mut self) {
        self.effect_time = 0.0;
        self.current_effect_duration = 0.0;
        self.effect_start_time = 0.0;
        self.is_active = false;
        self.has_played = false;
    }

    /// Returns the progress of the current effect
    pub fn progress(&self) -> f32 {
        if self.current_effect_duration > 0.0 {
            (self.effect_time / self.current_effect_duration).min(1.0)
        } else {
            1.0
        }
    }

    // fn update(&mut self, dt: f32) {
    //     if self.is_active {
    //         self.effect_time += dt;
    //         if self.effect_time >= self.current_effect_duration {
    //             self.is_active = false;
    //             self.has_played = true;
    //         }
    //     }
    // }
}
