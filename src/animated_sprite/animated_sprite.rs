use crate::{
    Animation, AnimationEffectTrait, AnimationQueueEntry, EffectDuration, EffectTimeTarget,
    InternalEffectsState, Seconds, X, Y,
};
use glam::Vec2;
use macroquad::color::Color;
use macroquad::math::Rect;
use macroquad::texture::{draw_texture_ex, DrawTextureParams, Texture2D};
use macroquad::time::get_frame_time;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::hash::Hash;

/// AnimatedSprite is the core struct that allows for animating a single sprite using multiple Animations stored inside.
#[derive(Serialize, Deserialize, Clone)]
pub struct AnimatedSprite<K: Eq + Hash + Clone> {
    tile_width: f32,
    tile_height: f32,
    animations: HashMap<K, Animation>,
    default_animation_key: K,
    animation_queue: VecDeque<AnimationQueueEntry<K>>,
    current_frame: u32,
    current_animation_loop_time: f32, // Time within the current loop of the animation
    current_animation_time: f32,      // Total time the current animation has been playing
    current_queue_time: EffectDuration,
    playing_time: EffectDuration,
    paused: bool,
    current_animation_key: K,
    previous_animation_key: Option<K>,
    effects_state: InternalEffectsState,
}

impl<K: Eq + Hash + Clone> AnimatedSprite<K> {
    /// Create a new AnimatedSprite.
    /// default_animation_key: The key you would like to use to reference the default animation stored in the AnimatedSprite
    pub fn new(
        tile_width: f32,
        tile_height: f32,
        default_animation_key: K,
        default_animation: Animation,
    ) -> Self {
        let mut animations = HashMap::new();
        animations.insert(default_animation_key.clone(), default_animation);

        AnimatedSprite {
            tile_width,
            tile_height,
            animations,
            default_animation_key: default_animation_key.clone(),
            animation_queue: VecDeque::new(),
            current_frame: 0,
            current_animation_time: 0.0,
            current_animation_loop_time: 0.0,
            current_queue_time: 0.0,
            playing_time: 0.0,
            paused: false,
            current_animation_key: default_animation_key.clone(),
            previous_animation_key: None,
            effects_state: InternalEffectsState::new(),
        }
    }

    /// Internal method, starts a new animation, resetting relevant fields and initializing effects.
    fn start_new_animation(&mut self, key: K, animation_duration: Seconds) {
        self.previous_animation_key = Some(self.current_animation_key.clone());
        self.current_animation_key = key;
        self.current_frame = 0;
        self.current_animation_loop_time = 0.0;
        self.current_animation_time = 0.0;
        self.current_queue_time = 0.0;
        self.effects_state.reset();

        if let Some(new_animation) = self.animations.get(&self.current_animation_key) {
            if let Some((_, target)) = &new_animation.effect {
                match target {
                    EffectTimeTarget::Start(duration) => {
                        let capped_duration = duration.min(animation_duration);
                        self.effects_state.current_effect_duration = capped_duration;
                        self.effects_state.is_active = true;
                        self.effects_state.effect_start_time = 0.0;
                    }
                    EffectTimeTarget::End(duration) => {
                        let capped_duration = duration.min(animation_duration);
                        self.effects_state.current_effect_duration = capped_duration;
                        self.effects_state.is_active = false;
                        self.effects_state.effect_start_time = animation_duration - capped_duration;
                    }
                }
            } else {
                self.effects_state.is_active = false;
                self.effects_state.current_effect_duration = 0.0;
                self.effects_state.effect_start_time = 0.0;
            }
        }
    }

    /// Sets the default animation of the sprite, referencing a previously registered Animation.
    pub fn set_default_animation(&mut self, key: K) -> Option<&mut Self> {
        if self.animations.contains_key(&key) {
            self.default_animation_key = key.clone();
            if self.animation_queue.is_empty() {
                self.start_new_animation(key, f32::MAX);
            }
            Some(self)
        } else {
            None
        }
    }

    /// Registers an animation in the sprite which can later be used as either the default, or part of the animation queue.
    /// Of note, registering another animation under the same key will replace the old one.
    pub fn register_animation(&mut self, key: K, animation: Animation) -> &mut Self {
        self.animations.insert(key, animation);
        self
    }

    /// Deletes a registered animation from the sprite by its key.
    pub fn delete_animation(&mut self, key: &K) -> &mut Self {
        self.animations.remove(key);
        self
    }

    /// Adds an animation to the queue. This will queue it up to be played for a `duration` number of seconds automatically.
    pub fn add_animation_to_queue(&mut self, key: K, duration: Seconds) -> Option<&mut Self> {
        if self.animations.contains_key(&key) {
            self.animation_queue.push_back((key.clone(), duration));

            if self.animation_queue.len() == 1 {
                self.start_new_animation(key, duration);
            }
            Some(self)
        } else {
            None
        }
    }

    /// Immediately moves to the next animation in the queue, dropping the current one even if the duration has not finished.
    pub fn next_in_queue(&mut self) -> &mut Self {
        self.animation_queue.pop_front();
        self.current_frame = 0;
        self.current_animation_time = 0.0;
        self
    }

    /// Resets the animation queue, deleting everything queued immediately, and defaulting to the default animation.
    pub fn reset_queue(&mut self) -> &mut Self {
        self.animation_queue.clear();
        self.current_frame = 0;
        self.current_animation_time = 0.0;
        self
    }

    /// If the AnimatedSprite is paused, this starts it animating once again.
    pub fn play(&mut self) -> &mut Self {
        self.paused = false;
        self
    }

    /// If the AnimatedSprite is not paused, this pauses it from animating. Any calls to the .update()
    /// method cause no frame changes when paused, meaning drawing will always draw the same frame.
    pub fn pause(&mut self) -> &mut Self {
        self.paused = true;
        self
    }

    /// Checks if the AnimatedSprite is paused.
    pub fn is_paused(&self) -> bool {
        self.paused
    }

    /// Checks if the animation queue is empty.
    pub fn is_queue_empty(&self) -> bool {
        self.animation_queue.is_empty()
    }

    /// Gets the current animation's key.
    pub fn get_current_animation_key(&self) -> &K {
        self.animation_queue
            .front()
            .map(|(k, _)| k)
            .unwrap_or(&self.default_animation_key)
    }

    /// Gets the current animation.
    pub fn get_current_animation(&self) -> Option<Animation> {
        self.animations
            .get(self.get_current_animation_key())
            .cloned()
    }

    /// Sets the current frame of the animation, potentially missing multiple frames and thus having a jarring visual effect.
    pub fn set_frame(&mut self, frame: u32) -> Option<&mut Self> {
        let animation = self.get_current_animation()?;
        self.current_frame = frame % animation.total_frames();
        Some(self)
    }

    /// Checks if the current frame is the last frame of the animation.
    pub fn is_last_frame(&self) -> bool {
        if let Some(animation) = self.get_current_animation() {
            self.current_frame == animation.total_frames() - 1
        } else {
            false
        }
    }

    /// Update must be called continuously by your application to ensure your AnimatedSprite changes frames/animates.
    /// This handles the internal logic for dealing with the animation queue and providing the draw methods with the correct frame.
    pub fn update(&mut self) -> &mut Self {
        if self.paused {
            return self;
        }

        let dt = get_frame_time();
        self.playing_time += dt;
        self.current_animation_loop_time += dt;
        self.current_animation_time += dt;
        self.current_queue_time += dt;

        let mut switch_animation = false;

        // Check if current animation is finished
        if let Some((_, duration)) = self.animation_queue.front() {
            if self.current_queue_time >= *duration {
                switch_animation = true;
            }
        }

        if let Some(animation) = self.animations.get(&self.current_animation_key) {
            // Handle effect activation
            if let Some((_, target)) = &animation.effect {
                match target {
                    EffectTimeTarget::Start(_) => {
                        if !self.effects_state.is_active && !self.effects_state.has_played {
                            self.effects_state.is_active = true;
                            self.effects_state.effect_time = 0.0;
                        }
                    }
                    EffectTimeTarget::End(_) => {
                        if !self.effects_state.is_active
                            && !self.effects_state.has_played
                            && self.current_animation_time >= self.effects_state.effect_start_time
                        {
                            self.effects_state.is_active = true;
                            self.effects_state.effect_time = 0.0;
                        }
                    }
                }
            }

            // Update effect state
            if self.effects_state.is_active {
                self.effects_state.effect_time += dt;
                if self.effects_state.effect_time >= self.effects_state.current_effect_duration {
                    self.effects_state.is_active = false;
                    self.effects_state.has_played = true;
                }
            }

            // Handle frame update
            let frame_duration = 1.0 / animation.fps as f32;
            while self.current_animation_loop_time >= frame_duration {
                self.current_frame = (self.current_frame + 1) % animation.total_frames();
                self.current_animation_loop_time -= frame_duration;
            }

            // Check if we've reached the end of the queued duration
            if self.current_queue_time
                >= self
                    .animation_queue
                    .front()
                    .map(|(_, d)| *d)
                    .unwrap_or(f32::MAX)
            {
                switch_animation = true;
            }
        }

        if switch_animation && !self.effects_state.is_active {
            self.animation_queue.pop_front();
            if let Some((next_key, duration)) = self.animation_queue.front() {
                self.start_new_animation(next_key.clone(), *duration);
            } else {
                // If queue is empty, switch to default animation
                self.start_new_animation(self.default_animation_key.clone(), f32::MAX);
            }
        }

        self
    }
    /// Draws the current frame of the animation on screen using extra params.
    pub fn draw_animation_ex(
        &self,
        texture: &Texture2D,
        x_pos: X,
        y_pos: Y,
        color: Color,
        mut params: DrawTextureParams,
    ) {
        if let Some(animation) = self.animations.get(&self.current_animation_key) {
            if animation.fps == 0 {
                return; // Don't draw if fps is 0
            }

            let (row, frame, _) = animation.get_row_and_frame_and_fps(self.current_frame);
            let current_frame_rect = self._get_current_frame_rect(row, frame);
            params.source = current_frame_rect;

            let mut final_color = color;
            let mut adjusted_x = x_pos;
            let mut adjusted_y = y_pos;

            if let Some((effect, _)) = &animation.effect {
                if self.effects_state.is_active {
                    let progress = self.effects_state.progress();
                    effect.apply(
                        progress,
                        &mut final_color,
                        &mut params,
                        &mut adjusted_x,
                        &mut adjusted_y,
                        self.tile_width,
                        self.tile_height,
                    );
                }
            }

            draw_texture_ex(&texture, adjusted_x, adjusted_y, final_color, params);
        }
    }

    /// Draws the current frame of the animation on screen with deafault params, but a specified output dest_size and no other special params.
    /// This or one of the other draw methods must be continously called by your application.
    pub fn draw_animation_dest_sized(
        &self,
        texture: &Texture2D,
        x_pos: f32,
        y_pos: f32,
        color: Color,
        dest_size_x: f32,
        dest_size_y: f32,
    ) {
        let mut draw_params = DrawTextureParams::default();
        draw_params.dest_size = Some(Vec2::new(dest_size_x, dest_size_y));

        self.draw_animation_ex(texture, x_pos, y_pos, color, draw_params);
    }

    /// Draws the current frame of the animation on screen with deafault params.
    /// This or one of the other draw methods must be continously called by your application.
    pub fn draw_animation(&self, texture: &Texture2D, x_pos: f32, y_pos: f32, color: Color) {
        self.draw_animation_ex(texture, x_pos, y_pos, color, DrawTextureParams::default());
    }

    /// Updates the AnimatedSprite<EntityAnimationType>, and calls the default draw method on it back-to-back.
    /// This must be continously called by your application (or one of the other update, and one of the other draw methods).
    pub fn update_and_draw_animation(
        &mut self,
        texture: &Texture2D,
        x_pos: f32,
        y_pos: f32,
        color: Color,
    ) {
        self.update();
        self.draw_animation(texture, x_pos, y_pos, color);
    }

    /// Updates the AnimatedSprite<EntityAnimationType>, and calls the default draw method on it back-to-back.
    /// This must be continously called by your application (or one of the other update, and one of the other draw methods).
    pub fn update_and_draw_animation_ex(
        &mut self,
        texture: &Texture2D,
        x_pos: f32,
        y_pos: f32,
        color: Color,
        params: DrawTextureParams,
    ) {
        self.update();
        self.draw_animation_ex(texture, x_pos, y_pos, color, params);
    }

    /// Gets the current frame rectangle dimensions.
    pub fn get_current_frame_rect(&self) -> Option<Rect> {
        let animation = self.get_current_animation()?;
        let (row, frame, _) = animation.get_row_and_frame_and_fps(self.current_frame);
        self._get_current_frame_rect(row, frame)
    }

    /// Internal, gets the current frame rectangle dimensions with the provided row and frame.
    fn _get_current_frame_rect(&self, row: u32, frame: u32) -> Option<Rect> {
        Some(Rect::new(
            self.tile_width * frame as f32,
            self.tile_height * row as f32,
            self.tile_width,
            self.tile_height,
        ))
    }

    /// Gets the length of the animation queue.
    pub fn get_queue_length(&self) -> usize {
        self.animation_queue.len()
    }

    /// Clears the animation queue.
    pub fn clear_queue(&mut self) -> &mut Self {
        self.animation_queue.clear();
        self
    }

    /// Resets the sprite.
    pub fn reset(&mut self) -> &mut Self {
        self.current_frame = 0;
        self.current_animation_time = 0.0;
        self.playing_time = 0.0;
        self.clear_queue()
    }

    /// Returns the number of seconds that this sprite has been animating for in total.
    pub fn get_animation_playing_time(&self) -> Seconds {
        self.playing_time as f32 / 1000.0
    }

    /// Returns the number of seconds that this current animation has been playing for.
    pub fn get_current_animation_time(&self) -> Seconds {
        self.current_animation_time as f32 / 1000.0
    }
}
