#[cfg(test)]
mod tests {
    use crate::*;
    use gdrust_sprite3d_animation_derive::SidedAnimation;
    use godot::{classes::AnimatedSprite3D, obj::NewAlloc};

    #[derive(SidedAnimation, Debug, PartialEq)]
    enum Animations {
        Idle,
        Walk,
        Attack,
    }

    #[test]
    fn test_changing_dir() {
        let mut animator = SS3DAnimator::new(Animations::Idle);
        animator.change_animation(Animations::Attack);

        assert_eq!(animator.get_current_animation(), &Animations::Attack);
    }

    #[test]
    fn test_camera_error() {
        let mut animator = SS3DAnimator::new(Animations::Idle);

        let result = animator.update();
        let expected = AnimatorError::CameraNotSetError(
            "⚠️ SS3DAnimator: Cannot update animation — camera is not set. 
            Make sure to call `set_camera()` before calling `update()`.",
        );

        assert_eq!(result, Err(expected));
    }
}
