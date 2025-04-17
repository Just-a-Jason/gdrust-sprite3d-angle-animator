mod single_sprite_animator;
mod tests;

pub use single_sprite_animator::SingleSpriteAnimator;
use godot::{classes::Camera3D, obj::Gd};

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
    fn assign_camera(&mut self, camera: Option<Gd<Camera3D>>);
    fn change_animation(&mut self, animation: T);
    fn get_current_dir(&self) -> Direction;
    fn freeze_dir_until_finished(&mut self);
    fn get_current_animation(&self) -> &T;
    fn is_playing(&self) -> bool;
    fn update_animation(&self);
    fn update(&mut self);
    fn play(&self);
    fn pause(&self);
}
