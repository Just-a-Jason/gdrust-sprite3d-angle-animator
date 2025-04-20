use crate::{Animator, AnimatorError, Direction, SidedAnimation};
use {
    godot::classes::{AnimatedSprite3D, Camera3D},
    godot::prelude::*,
};

#[derive(Debug)]
pub struct SS3DAnimator<T: SidedAnimation> {
    freeze_animation: bool,
    current_animation: T,
    current_direction: Direction,
    last_direction: Direction,
    camera: Option<Gd<Camera3D>>,
    sprite: Option<Gd<AnimatedSprite3D>>,
}

impl<T: SidedAnimation> SS3DAnimator<T> {
    pub fn new(default_animation: T) -> Self {
        SS3DAnimator {
            freeze_animation: false,
            current_animation: default_animation,
            current_direction: Direction::default(),
            last_direction: Direction::default(),
            camera: None,
            sprite: None,
        }
    }
}

impl<T: SidedAnimation> Animator<T> for SS3DAnimator<T> {
    fn update(&mut self) -> Result<(), AnimatorError> {
        use crate::core::calculate_dir;

        if self.camera.is_none() {
            let text: &'static str =
                "⚠️ SS3DAnimator: Cannot update animation — camera is not set. 
            Make sure to call `set_camera()` before calling `update()`.";

            godot_warn!("{}", text);

            return Err(AnimatorError::CameraNotSetError(text));
        } else if self.sprite.is_none() {
            let text: &'static str = "⚠️ SS3DAnimator: Cannot update animation — sprite is not set.
                Make sure to call `set_sprite()` before calling `update()`.";

            godot_warn!("{}", text);

            return Err(AnimatorError::SpriteNotSetError(text));
        }

        let camera = self.camera.as_ref().unwrap();
        let sprite = self.sprite.as_ref().unwrap();

        let dir = calculate_dir(&camera, &sprite);

        if dir == self.last_direction {
            return Ok(());
        }

        match self.update_animation() {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
        self.last_direction = dir;

        Ok(())
    }

    fn change_animation(&mut self, animation: T) {
        self.current_animation = animation;
    }

    fn play(&mut self) -> Result<(), AnimatorError> {
        if self.freeze_animation {
            self.freeze_animation = false;
        }

        match self.sprite.as_mut() {
            Some(sprite) => sprite.play(),
            None => {
                let text: &'static str = "⚠️ Cannot play animation on a 'None' value: sprite is not set in SS3DAnimator.";
                godot_warn!("{}", text);
                return Err(AnimatorError::SpriteNotSetError(text));
            }
        }

        Ok(())
    }

    fn pause(&mut self) -> Result<(), AnimatorError> {
        self.freeze_animation = true;

        match self.sprite.as_mut() {
            Some(sprite) => sprite.pause(),
            None => godot_warn!(
                "⚠️ Cannot pause animation on a 'None' value: sprite is not set in SS3DAnimator."
            ),
        }

        Ok(())
    }

    fn set_camera(&mut self, camera: &Gd<Camera3D>) {
        self.camera = Some(camera.clone());
    }

    fn get_animation(&self) -> &T {
        &self.current_animation
    }

    fn get_direction(&self) -> Direction {
        self.current_direction
    }
}

/*
    ? Gd<T> smart-pointer is cheap to clone. The clone works for reference only.
    ? It does not clone the full struct.
    ? It works very similar to Rc<T> smart-pointer. - (Just for me to remember.) ~ Jason.json
*/
impl<T: SidedAnimation> SS3DAnimator<T> {
    pub fn set_sprite(&mut self, sprite: &Gd<AnimatedSprite3D>) {
        self.sprite = Some(sprite.clone());
    }

    fn update_animation(&mut self) -> Result<(), AnimatorError> {
        let sprite = match self.sprite.as_mut() {
            Some(sprite) => sprite,
            None => return Err(AnimatorError::SpriteNotSetError(
                "⚠️ Cannot update animation on a 'None' value: sprite is not set in SS3DAnimator.",
            )),
        };

        let animation = self.current_animation.to_sided(self.current_direction);

        sprite.set_animation(animation);

        if !sprite.is_playing() {
            sprite.play();
        }

        Ok(())
    }
}
