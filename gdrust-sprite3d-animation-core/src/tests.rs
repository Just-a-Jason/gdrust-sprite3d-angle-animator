#[cfg(test)]
mod tests {
    use crate::*;
    use gdrust_sprite3d_animation_derive::SidedAnimation;

    #[derive(SidedAnimation, Debug, PartialEq, Copy, Clone)]
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
    fn test() {
        let mut animator = SS3DAnimator::new(Animations::Idle);

        animator.change_animation(Animations::Attack);
        assert_eq!(animator.get_current_animation(), &Animations::Attack);
    }
}
