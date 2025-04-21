use crate::{Animator, AnimatorError, Direction, SS3DAnimator, SidedAnimation};
use godot::{
    builtin::Array,
    classes::{AnimatedSprite3D, Camera3D},
    obj::Gd,
};

pub struct MS3DAnimator<T: SidedAnimation> {
    freeze_animation: bool,
    camera: Option<Gd<Camera3D>>,
    current_animation: T,
    animators: Vec<SS3DAnimator<T>>,
}

impl<T: SidedAnimation + Copy> Animator<T> for MS3DAnimator<T> {
    fn change_animation(&mut self, animation: T) -> Result<(), AnimatorError> {
        self.animators
            .iter_mut()
            .try_for_each(|animator| animator.change_animation(animation))
    }

    fn update(&mut self) -> Result<(), AnimatorError> {
        self.animators
            .iter_mut()
            .try_for_each(|animator| animator.update())
    }

    fn play(&mut self) -> Result<(), AnimatorError> {
        self.freeze_animation = false;

        self.animators
            .iter_mut()
            .try_for_each(|animator| animator.play())
    }

    fn pause(&mut self) -> Result<(), AnimatorError> {
        self.freeze_animation = true;

        self.animators
            .iter_mut()
            .try_for_each(|animator| animator.pause())
    }

    fn set_camera(&mut self, camera: &Gd<Camera3D>) {
        self.camera = Some(camera.clone());

        self.animators
            .iter_mut()
            .for_each(|animator| animator.set_camera(camera));
    }

    fn get_animation(&self) -> &T {
        &self.current_animation
    }

    fn get_direction(&self) -> Direction {
        let animator = self.animators.get(0);

        match animator {
            Some(animator) => animator.get_direction(),
            None => Direction::default(),
        }
    }
}

impl<T: SidedAnimation + Copy> MS3DAnimator<T> {
    pub fn new(default_animation: T) -> Self {
        MS3DAnimator {
            camera: None,
            current_animation: default_animation,
            freeze_animation: false,
            animators: Vec::new(),
        }
    }
}

impl<T: SidedAnimation + Copy> MS3DAnimator<T> {
    pub fn get_sprites(&self) -> Vec<Gd<AnimatedSprite3D>> {
        self.animators
            .iter()
            .filter_map(|animator| animator.get_sprite())
            .collect()
    }

    pub fn get_animators(&self) -> &Vec<SS3DAnimator<T>> {
        &self.animators
    }

    pub fn set_sprites(&mut self, sprites: &Array<Gd<AnimatedSprite3D>>) {
        self.animators = sprites
            .iter_shared()
            .filter_map(|item| item.try_cast::<AnimatedSprite3D>().ok())
            .map(|sprite| {
                let mut animator = SS3DAnimator::new(self.current_animation);
                animator.set_sprite(&sprite);
                animator
            })
            .collect();
    }
}
