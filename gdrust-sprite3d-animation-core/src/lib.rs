mod animators;
mod core;
mod errors;
mod tests;

pub use animators::{muti_sprite_animator::MS3DAnimator, single_sprite_animator::SS3DAnimator};
pub use errors::*;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    #[default]
    Front,
    Back,
    Left,
    Right,
}

pub trait SidedAnimation {
    fn to_sided(&self, dir: Direction) -> &'static str;
}

pub trait Animator<T: SidedAnimation + Copy> {
    // ? Can fail ❌
    fn change_animation(&mut self, animation: T) -> Result<(), AnimatorError>;
    fn update(&mut self) -> Result<(), AnimatorError>;
    fn play(&mut self) -> Result<(), AnimatorError>;
    fn pause(&mut self) -> Result<(), AnimatorError>;

    // ? Cannot fail ✅
    fn set_camera(&mut self, camera: &godot::obj::Gd<godot::classes::Camera3D>);
    fn get_animation(&self) -> &T;
    fn get_direction(&self) -> Direction;
}
