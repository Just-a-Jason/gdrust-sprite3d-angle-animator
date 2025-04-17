use godot::prelude::*;

pub struct MultiSpriteAnimator <T: SidedAnimation> {
    current_animation: T,
    freeze_rotation: bool,
    current_direction: Direction,
    last_direction: Direction,
    camera: RefCell<Option<Gd<Camera3D>>>,
    sprites: RefCell<Option<Array<Gd<AnimatedSprite3D>>>>,
}


impl<T: SidedAnimation> MultiSpriteAnimator<T> {
    pub fn new(default_animation: T) -> Self {
        MultiSpriteAnimator {
            current_animation: default_animation,
            freeze_rotation: false,
            current_direction: Direction::default(),
            last_direction: Direction::default(),
            camera: RefCell::new(None),
            sprites: RefCell::new(None),
        }
    }
} 


impl<T:SidedAnimation> Animator for MultiSpriteAnimator {

}
