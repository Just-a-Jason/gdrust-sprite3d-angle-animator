mod single_sprite_animator;
mod multi_sprite_animator;
mod tests;
mod utils;
mod core;

pub use single_sprite_animator::SS3DAnimator;
pub use multi_sprite_animator::MS3DAnimator;

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
    // Used for converion between SP3DAnimator
    fn take_sprite(&self) -> Option<godot::obj::Gd<godot::classes::AnimatedSprite3D>>;
    fn take_camera(&self) -> Option<godot::obj::Gd<godot::classes::Camera3D>>;

    fn assign_camera(&mut self, camera: godot::obj::Gd<godot::classes::Camera3D>);
    fn change_animation(&mut self, animation: T);
    fn get_current_dir(&self) -> Direction;
    fn freeze_dir_until_finished(&mut self);
    fn get_current_animation(&self) -> T;
    fn is_playing(&self) -> bool;
    fn update_animation(&self);
    fn update(&mut self);
    fn play(&self);
    fn pause(&self);
}
