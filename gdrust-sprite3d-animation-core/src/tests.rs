#[cfg(test)]
mod tests {
    use crate::*;
    use gdrust_sprite3d_animation_derive::SidedAnimation;

    #[derive(SidedAnimation, Debug, PartialEq)]
    enum Animations {
        Idle,
        Walk,
        Attack,
    }

    #[test]
    fn test_changing_dir() {
        let mut animator = Animator::new(Animations::Idle);
        animator.change_animation(Animations::Attack);

        assert_eq!(animator.get_current_animation(), &Animations::Attack);
    }
}
