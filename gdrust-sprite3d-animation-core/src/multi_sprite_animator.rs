use {godot::prelude::*, godot::classes::AnimatedSprite3D};
use crate::{utils::ArrayIterator, Animator, Direction, SidedAnimation};
use std::cell::RefCell;

pub struct MS3DAnimator <T: SidedAnimation> {
    current_animation: T,
    freeze_rotation: bool,
    current_direction: Direction,
    last_direction: Direction,
    camera: RefCell<Option<Gd<Camera3D>>>,
    sprites: RefCell<ArrayIterator<Gd<AnimatedSprite3D>>>,
}

impl<T: SidedAnimation> MS3DAnimator<T> {
    pub fn new(default_animation: T) -> Self {
        MS3DAnimator {
            current_animation: default_animation,
            freeze_rotation: false,
            current_direction: Direction::default(),
            last_direction: Direction::default(),
            camera: RefCell::new(None),
            sprites: RefCell::new(ArrayIterator::new(array![])),
        }
    }
} 

impl<T:SidedAnimation + Copy> Animator<T> for MS3DAnimator<T> {
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

    fn freeze_dir_until_finished(&mut self) {
        self.freeze_rotation = true;
    }

    fn get_current_animation(&self) -> T {
        self.current_animation
    }

    fn is_playing(&self) -> bool {
        if let Some(sprite) = self.sprites.borrow().get(0) {
            return sprite.is_playing();
        }
        false
    }

    fn update_animation(&self) {
        let mut sprites = self.sprites.borrow_mut();
        let animation = self.current_animation.to_sided(self.current_direction);

        for mut sprite in sprites.by_ref() {
            sprite.set_animation(animation);

            if sprite.is_playing() { continue; }

            sprite.play();
        }
    }

    fn update(&mut self) {
        use crate::core::calculate_dir;

        if self.freeze_rotation {
            return;
        }

        let camera_ref = self.camera.borrow();
        let camera = camera_ref.as_ref().expect("Camera must be set");

        let sprite_ref = self.sprites.borrow().get(0);
        let sprite = sprite_ref.as_ref().expect("Sprite must be set");

        let facing_dir = calculate_dir(&camera, &sprite);

        if self.last_direction == facing_dir {
            return;
        }

        self.current_direction = facing_dir;
        self.update_animation();
        self.last_direction = facing_dir;
    }

    fn play(&self) {
        let mut sprites = self.sprites.borrow_mut();

        for mut sprite in sprites.by_ref() {
            sprite.play();
        }
    }

    fn pause(&self) {
        let mut sprites = self.sprites.borrow_mut();
        
        for mut sprite in sprites.by_ref() {
            sprite.pause();
        }
    }

    fn take_sprite(&self) -> Option<Gd<AnimatedSprite3D>> {
        self.sprites.borrow().get(0)      
    }
    
    fn take_camera(&self) -> Option<godot::obj::Gd<godot::classes::Camera3D>> {
        self.camera.borrow_mut().take()
    }
}


impl<T:crate::SidedAnimation + Copy> From<crate::SS3DAnimator<T>> for MS3DAnimator<T> {
    fn from(value: crate::SS3DAnimator<T>) -> Self {
        let sprite = value.take_sprite().expect("To convert From<SS3DAnimator> to MS3DAnimator struct you need to have the sprite alredy initialized!");


        MS3DAnimator {
            sprites: RefCell::new(ArrayIterator::new(array![&sprite])),
            camera:RefCell::new(value.take_camera()),
            current_animation: value.get_current_animation(),  
            current_direction:value.get_current_dir(),
            last_direction: value.get_current_dir(),
            freeze_rotation: false
        }
    }
}