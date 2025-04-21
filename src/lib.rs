mod tests;

pub mod prelude;

pub mod core {
    pub use gdrust_sprite3d_animation_core::Direction;
}

pub mod animators {
    pub use gdrust_sprite3d_animation_core::{Animator, MS3DAnimator, SS3DAnimator};
}

pub mod derive {
    pub use gdrust_sprite3d_animation_derive::SidedAnimation;
}
