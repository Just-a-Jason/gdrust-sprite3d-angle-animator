use {godot::prelude::*, godot::classes::AnimatedSprite3D};
use crate::{Direction, SidedAnimation, Animator};
use std::cell::RefCell;


pub struct MultiSpriteAnimator <T: SidedAnimation> {
    current_animation: T,
    freeze_rotation: bool,
    current_direction: Direction,
    last_direction: Direction,
    camera: RefCell<Option<Gd<Camera3D>>>,
    sprites: RefCell<Option<Array<Option<Gd<AnimatedSprite3D>>>>>,
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


impl<T:SidedAnimation + Copy> Animator<T> for MultiSpriteAnimator<T> {
    fn assign_camera(&mut self, camera: Option<Gd<Camera3D>>) {
        todo!()
    }

    fn change_animation(&mut self, animation: T) {
        todo!()
    }

    fn get_current_dir(&self) -> Direction {
        self.current_direction
    }

    fn freeze_dir_until_finished(&mut self) {
        todo!()
    }

    fn get_current_animation(&self) -> T {
        self.current_animation
    }

    fn is_playing(&self) -> bool {
        todo!()
    }

    fn update_animation(&self) {
        todo!()
    }

    fn update(&mut self) {
        todo!()
    }

    fn play(&self) {
        todo!()
    }

    fn pause(&self) {
        todo!()
    }

    fn get_sprite(&self) -> Option<Gd<AnimatedSprite3D>> {
        if let Some(array) = self.sprites.borrow().as_ref() {
            match array.get(0) {
                Some(sprite) => {
                    return sprite;
                }
                None => return None
            }
        }
        None
    }
}


impl<T:crate::SidedAnimation + Copy> From<crate::SingleSpriteAnimator<T>> for MultiSpriteAnimator<T> {
    fn from(value: crate::SingleSpriteAnimator<T>) -> Self {
        MultiSpriteAnimator {
            sprites: RefCell::new(None),
            camera:RefCell::new(None),
            current_animation: value.get_current_animation(),  
            current_direction:value.get_current_dir(),
            last_direction: value.get_current_dir(),
            freeze_rotation: false
        }
    }
}
