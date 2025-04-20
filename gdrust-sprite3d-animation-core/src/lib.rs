mod core;
mod errors;
mod single_sprite_animator;
mod tests;
mod utils;

pub use errors::*;
pub use single_sprite_animator::SS3DAnimator;

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

pub trait Animator<T: SidedAnimation> {
    // ? Can fail
    fn update(&mut self) -> Result<(), AnimatorError>;
    fn play(&mut self) -> Result<(), AnimatorError>;
    fn pause(&mut self) -> Result<(), AnimatorError>;

    // ? Bullet proof
    fn set_camera(&mut self, camera: &godot::obj::Gd<godot::classes::Camera3D>);
    fn change_animation(&mut self, animation: T);
    fn get_current_animation(&self) -> &T;
}
