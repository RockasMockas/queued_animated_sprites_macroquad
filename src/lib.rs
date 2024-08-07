//! # Queued Animated Sprites Macroquad
//!
//! The `queued_animated_sprites_macroquad` Rust crate offers an easy interface for animated sprite drawing
//! focused around a queue-based design. It is built on top of [macroquad](https://crates.io/crates/macroquad) and provides flexibility with its animation effect system that enhances how your sprites are drawn with little extra work.
//!
//! ## Features
//!
//! - Easy creation and management of sprite animations
//! - Support for multiple animations per sprite stored using a generic key type to trivially fit into your project (ex. use your own enum keys)
//! - Serialization for easy saving and loading of sprite states
//! - A queue based system, where animations automatically-chain based on duration, providing a great interface for advanced animation combining
//! - An animation effect system, which enables applying unique effects to your sprites with no extra work (fade in/out, slide in/out, spin, pulse, etc.)
//! - Optional custom effects crate feature, which allows anyone to implement their own new effects instantly
//!
//!
//! ## Built-in Effects
//!
//! The library provides a number of built-in effects such as:
//!
//! - **Blinking(EffectColor, u32)**: Make the sprite blink with the specified color and number of blinks (used for damage or low health effects)
//! - **FadeIn** and **FadeOut**: Gradually changes the opacity of the sprite.
//! - **SlideIn(SlideDirection)** and **SlideOut(SlideDirection)**: Move the sprite in or out of the screen.
//! - **Pulse(f32)**: Scale the sprite up and down, centered on its origin. The f32 parameter determines the maximum scale factor.
//! - **Shake(f32)**: Apply a shaking effect to the sprite. The f32 parameter determines the intensity of the shake.
//! - **Wobble(f32)**: Apply a wobbling effect to the sprite. The f32 parameter determines the intensity of the wobble.
//! - **Bounce(f32, u32)**: Make the sprite bounce. The f32 parameter determines the height of the bounce, and the u32 parameter specifies the number of bounces.
//! - **BasicFlip(FlipDirection)**: Flip the sprite either horizontally or vertically.
//! - **Spin**: Rotates the sprite.
//! - **Glitch(f32)**: Apply a glitch effect to the sprite. The f32 parameter determines the intensity of the glitch.
//! - **ShearLeft(f32)** and **ShearRight(f32)**: Apply a shearing effect to the sprite. The f32 parameter determines the intensity of the shear.
//! - **SquashFlipVertical(f32)** and **SquashFlipVertical(f32)**: Squash + flip the sprite either vertically or horizontally. The f32 parameter determines the intensity of the squash.
//! - **ColorCycle(Vec<EffectColor>)**: Cycle through a palette of colors.
//!
//! ## Basic Usage
//!
//!  At the core of the library is the `AnimatedSprite` struct where the majority of functionality in this library takes place.
//!
//! Here's a quick example of how to create and use an animated sprite:
//!
//! ```rust
//! use queued_animated_sprites_macroquad::{AnimatedSprite, Animation, AnimationEffect, SlideFrom};
//! use macroquad::prelude::*;
//!
//! #[macroquad::main("Queued Animated Sprites Demo")]
//! async fn main() {
//!     // Load the spritesheet
//!     let texture = Texture2D::from_file_with_format(
//!         include_bytes!("slime.png"),
//!         None
//!     );
//!
//!     // Create the animated sprite
//!     let mut slime = AnimatedSprite::new(
//!         32.0,  // Width of each sprite on the spritesheet
//!         32.0,  // Height of each sprite on the spritesheet
//!         "idle", // Default key you want to have for your default animation
//!         Animation::new(0, 4, 6)  // Default animation, uses the first row on the spritesheet, with 4 sprites on the row, running at 6fps
//!     );
//!
//!     // Add an attack animation
//!     slime.register_animation(
//!         "attack",
//!         Animation::new(1, 6, 12)  // Targets spritesheet's first row (1), takes the first 6 frames on the row, plays them at 12 fps
//!     );
//!
//!     // Using the added attack animation, queue it up and set its duration for 1.5 seconds
//!     slime.add_animation_to_queue("attack", 1.5);
//!
//!     loop {
//!         clear_background(WHITE);
//!
//!         // Update and draw the sprite
//!         slime.update();
//!         slime.draw_animation(&texture, 400.0, 300.0, WHITE); // spritesheet texture, x, y, color
//!
//!         next_frame().await
//!     }
//! }
//! ```
//!
//! This example creates a slime sprite with an idle animation and an attack animation.
//! The sprite performs the attack animation for 1.5 seconds before returning to idle (default animation).
//!
//! ## Animation Effects
//!
//! This library includes a built-in animation effects system which can be applied to your sprites.
//! These allow you to perform things like have a slime monster fade in while spawning, have your sprites slide off the screen
//! when moving in/out of a battle, have a UI element pulse when it needs to be seen, and more.
//!
//! Here's an example of how to use animation effects:
//!
//! ```rust
//! use queued_animated_sprites_macroquad::{AnimatedSprite, Animation, AnimationEffect, SlideFrom};
//!
//! // ... (previous setup code)
//!
//! // Add a spawn animation with fade-in effects
//! slime.register_animation(
//!     "spawn",
//!     Animation::new(2, 4, 8)  // row 2, 4 frames, 8 fps
//!         .with_start_effect(AnimationEffect::FadeIn, 1) // fade in effect has a duration of 1s from the start of the spawn animation
//! );
//!
//! // Add a despawn animation with fade-out
//! slime.register_animation(
//!     "despawn",
//!     Animation::new(3, 4, 8)  // row 3, 4 frames, 8 fps
//!         .with_end_effect(AnimationEffect::FadeOut, 0.5) // fade out effect has a duration starting 0.5s before the end of the despawn animation
//! );
//!
//! // Queue the spawn animation which will play for 1 second
//! slime.add_animation_to_queue("spawn", 1.0);
//! ```
//!
//! In this example, we've added a "spawn" animation that fades in, and a "despawn" animation that fades out.
//! Of note, you can use `Animation::empty()` to have the sprite skip drawing entirely. This means it's possible to set empty as the default
//! (useful for visual effects or otherwise that are triggered at specific moments), or be added into the queue to stop drawing the sprite temporarily.
//!
//!
//! ## Custom Effects (Optional Feature)
//!
//! For more advanced use cases, `queued_animated_sprites_macroquad` also supports custom animation effects.
//! This is an optional feature that can be enabled in your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! queued_animated_sprites_macroquad = { version = "0.1.0", features = ["custom_effects"] }
//! ```
//!
//! Custom effects allow you to define your own effect behaviors by specifying a single function.
//! The one tradeoff is that custom effects are not serializable (thus less plug-and-play), so this
//! is an optional feature for advanced users.
//!
//! Here's an example of creating and using a custom "color cycle" effect:
//!
//! ```rust
//! use queued_animated_sprites_macroquad::{AnimatedSprite, Animation, AnimationEffect};
//! use macroquad::prelude::*;
//!
//! // Define the color cycle effect
//! fn color_cycle_effect(
//!     progress: f32,
//!     color: &mut Color,
//!     _params: &mut DrawTextureParams,
//!     _x_pos: &mut f32,
//!     _y_pos: &mut f32,
//!     _tile_width: f32,
//!     _tile_height: f32,
//! ) {
//!     color.r = (progress * std::f32::consts::PI * 2.0).sin() * 0.5 + 0.5;
//!     color.g = (progress * std::f32::consts::PI * 2.0 + 2.0).sin() * 0.5 + 0.5;
//!     color.b = (progress * std::f32::consts::PI * 2.0 + 4.0).sin() * 0.5 + 0.5;
//! }
//!
//! // ... (previous setup code)
//!
//! // Add an idle animation with the color cycle effect
//! let color_cycle_animation = Animation::new(0, 4, 6)
//!     .with_start_effect(AnimationEffect::new_custom(color_cycle_effect), 2.0);
//!
//! slime.register_animation("idle_color_cycle", color_cycle_animation);
//!
//! // Queue the color cycling idle animation, which will play for 3.0 seconds
//! slime.add_animation_to_queue("idle_color_cycle", 3.0);
//! ```
//!
//! This example creates a custom color cycle effect that changes the color of the sprite over time.
//! The effect is applied to an "idle_color_cycle" animation that lasts for 3 seconds.

pub mod animated_sprite;
pub mod effects;

pub use animated_sprite::*;
pub use effects::*;

type AnimationQueueEntry<K> = (K, EffectDuration); // (key, duration)
pub type X = f32;
pub type Y = f32;
pub type EffectDuration = Seconds;
pub type Seconds = f32;
pub type Red = f32;
pub type Green = f32;
pub type Blue = f32;
