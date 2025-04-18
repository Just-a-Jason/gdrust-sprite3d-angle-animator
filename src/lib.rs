mod tests;

pub mod prelude;

pub mod core {
    pub use gdrust_sprite3d_animation_core::{SS3DAnimator, MS3DAnimator, Direction, SidedAnimation};
}

pub mod derive {
    pub use gdrust_sprite3d_animation_derive::SidedAnimation;
}