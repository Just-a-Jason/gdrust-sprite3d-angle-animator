use crate::{Direction, SidedAnimation};
use godot::{
    builtin::Vector3,
    classes::{AnimatedSprite3D, Camera3D},
    obj::Gd,
};
use std::cell::RefCell;

const SIDE_ANGLE: f32 = 155.0;
const BACK_ANGLE: f32 = 65.0;

#[derive(Debug)]
pub struct Animator<T: SidedAnimation> {
    current_animation: T,
    current_direction: Direction,
    last_direction: Direction,
    camera: RefCell<Option<Gd<Camera3D>>>,
    sprite: RefCell<Option<Gd<AnimatedSprite3D>>>,
}

impl<T: SidedAnimation> Animator<T> {
    pub fn new(default_animation: T) -> Animator<T> {
        Animator {
            current_animation: default_animation,
            current_direction: Direction::default(),
            last_direction: Direction::default(),
            camera: RefCell::new(None),
            sprite: RefCell::new(None),
        }
    }
}

impl<T: SidedAnimation> Animator<T> {
    pub fn assign_sprite(&mut self, sprite: Option<Gd<AnimatedSprite3D>>) {
        *self.sprite.borrow_mut() = sprite;
    }

    pub fn assign_camera(&mut self, camera: Option<Gd<Camera3D>>) {
        *self.camera.borrow_mut() = camera;
    }

    pub fn change_animation(&mut self, animation: T) {
        self.current_animation = animation;
        self.update_animation();
    }

    pub fn get_current_dir(&self) -> Direction {
        self.current_direction
    }

    pub fn get_current_animation(&self) -> &T {
        &self.current_animation
    }

    pub fn play(&self) {
        if let Some(sprite_ref) = self.sprite.borrow_mut().as_mut() {
            if sprite_ref.is_playing() {
                return;
            }

            sprite_ref.play();
        }
    }

    pub fn pause(&self) {
        if let Some(sprite_ref) = self.sprite.borrow_mut().as_mut() {
            if !sprite_ref.is_playing() {
                return;
            }

            sprite_ref.pause();
        }
    }

    pub fn is_playing(&self) -> bool {
        if let Some(sprite) = self.sprite.borrow().as_deref() {
            return sprite.is_playing();
        }

        false
    }

    pub fn update_animation(&self) {
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

    pub fn update(&mut self) {
        let facing_dir = self.get_facing_direction();

        if self.last_direction == facing_dir {
            return;
        }

        self.current_direction = facing_dir;
        self.update_animation();
        self.last_direction = facing_dir;
    }
}

impl<T: SidedAnimation> Animator<T> {
    fn get_facing_direction(&self) -> Direction {
        if let Some(camera) = self.camera.borrow_mut().as_mut() {
            let forward = camera.get_global_transform().basis.col_c();
            let cam_forward = Vector3::new(forward.x, 0.0, forward.z).normalized();

            if let Some(sprite) = self.sprite.borrow_mut().as_mut() {
                let forward = sprite.get_global_transform().basis.col_c();
                let sprite_forward = Vector3::new(forward.x, 0.0, forward.z).normalized();

                let signed_angle = cam_forward
                    .signed_angle_to(sprite_forward, Vector3::UP)
                    .to_degrees();
                let angle = signed_angle.abs();

                if angle < BACK_ANGLE {
                    return Direction::Back;
                }

                return match angle {
                    a if a < BACK_ANGLE => Direction::Back,
                    a if a < SIDE_ANGLE => {
                        return if signed_angle > 0.0 {
                            Direction::Right
                        } else {
                            Direction::Left
                        }
                    }
                    _ => Direction::Front,
                };
            }
        }
        Direction::Front
    }
}
