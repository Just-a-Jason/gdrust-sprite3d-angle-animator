#[cfg(test)]
mod tests {
    use animation_core::*;
    use sided_animation_derive::SidedAnimation;

    #[derive(SidedAnimation)]
    enum CharacterAnimation {
        Idle,
        Walking,
        Running,
    }

    #[test]
    pub fn test_sided_animation_macro() {
        assert_eq!(
            "idle_front",
            CharacterAnimation::Idle.to_sided(Direction::Front)
        );
        assert_eq!(
            "walking_side",
            CharacterAnimation::Walking.to_sided(Direction::Left)
        );
        assert_eq!(
            "running_back",
            CharacterAnimation::Running.to_sided(Direction::Back)
        );
    }
}
