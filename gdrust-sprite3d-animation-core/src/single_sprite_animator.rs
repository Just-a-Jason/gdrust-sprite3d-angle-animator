use crate::{Animator, Direction, SidedAnimation};
use std::cell::RefCell;
use {
    godot::classes::{AnimatedSprite3D, Camera3D},
    godot::prelude::*,
};

#[derive(Debug)]
pub struct SS3DAnimator<T: SidedAnimation> {
    freeze_rotation: bool,
    current_animation: T,
    current_direction: Direction,
    last_direction: Direction,
    camera: RefCell<Option<Gd<Camera3D>>>,
    sprite: RefCell<Option<Gd<AnimatedSprite3D>>>,
}

impl<T: SidedAnimation> SS3DAnimator<T> {
    pub fn new(default_animation: T) -> SS3DAnimator<T> {
        SS3DAnimator {
            freeze_rotation: false,
            current_animation: default_animation,
            current_direction: Direction::default(),
            last_direction: Direction::default(),
            camera: RefCell::new(None),
            sprite: RefCell::new(None),
        }
    }
}

impl<T: SidedAnimation + Copy> Animator<T> for SS3DAnimator<T> {
    fn assign_camera(&mut self, camera: Gd<Camera3D>) {
        *self.camera.borrow_mut() = Some(camera);
    }

    fn change_animation(&mut self, animation: T) {
        self.current_animation = animation;
        self.update_animation();
    }

    fn get_current_dir(&self) -> Direction {
        self.current_direction
    }

    fn get_current_animation(&self) -> T {
        self.current_animation
    }

    fn play(&self) {
        if let Some(sprite_ref) = self.sprite.borrow_mut().as_mut() {
            if sprite_ref.is_playing() {
                return;
            }

            sprite_ref.play();
        }
    }

    fn pause(&self) {
        if let Some(sprite_ref) = self.sprite.borrow_mut().as_mut() {
            if !sprite_ref.is_playing() {
                return;
            }

            sprite_ref.pause();
        }
    }

    fn is_playing(&self) -> bool {
        if let Some(sprite) = self.sprite.borrow().as_deref() {
            return sprite.is_playing();
        }

        false
    }

    fn update_animation(&self) {
        if let Some(sprite_ref) = self.sprite.borrow_mut().as_mut() {
            match self.current_direction {
                Direction::Right => {
                    sprite_ref.set_flip_h(false);
                }
                Direction::Left => sprite_ref.set_flip_h(true),
                _ => (),
            }

            let animation = self.current_animation.to_sided(self.current_direction);
            sprite_ref.set_animation(animation);
            sprite_ref.play();
        }
    }

    fn update(&mut self) {
        use crate::core::calculate_dir;

        if self.freeze_rotation {
            return;
        }

        let camera_ref = self.camera.borrow();
        let camera = camera_ref.as_ref().expect("Camera must be set");

        let sprite_ref = self.sprite.borrow();
        let sprite = sprite_ref.as_ref().expect("Sprite must be set");

        let facing_dir = calculate_dir(&camera, &sprite);

        if self.last_direction == facing_dir {
            return;
        }

        self.current_direction = facing_dir;
        self.update_animation();
        self.last_direction = facing_dir;
    }

    fn freeze_dir_until_finished(&mut self) {
        self.freeze_rotation = true;
        // !TODO To implement leater
    }

    fn take_sprite(&self) -> Option<Gd<AnimatedSprite3D>> {
        self.sprite.borrow_mut().take()
    }

    fn take_camera(&self) -> Option<godot::obj::Gd<godot::classes::Camera3D>> {
        self.camera.borrow_mut().take()   
    }
}

impl<T: SidedAnimation> SS3DAnimator<T> {
    pub fn assign_sprite(&mut self, sprite: Option<Gd<AnimatedSprite3D>>) {
        *self.sprite.borrow_mut() = sprite;
    }
}
