![Image](https://github.com/user-attachments/assets/9a0f8d6e-c7b0-4fda-9b00-d52502d2be8a)

`gdrust-sprite3d-angle-animator` is a Rust library for animating 3D sprites based on the camera's viewing direction—similar to the classic Doom-style visuals. It allows for smooth sprite angle switching and animation control depending on where the player is looking.

## Example usage

```rs
use godot::{
    classes::{AnimatedSprite3D, INode2D, Node2D},
    global::godot_warn,
    obj::{Base, Gd, WithBaseField},
    prelude::{godot_api, GodotClass},
};

#[derive(SidedAnimation)]
pub enum CharacterAnimations {
    Idle,
    Walk,
}

#[derive(GodotClass)]
#[class(base=Node2D)]
struct CharacterNativeAnimator {
    base: Base<Node2D>,

    #[export]
    sprite: Option<Gd<AnimatedSprite3D>>,
    animator: Animator<CharacterAnimations>,
}

#[godot_api]
impl INode2D for CharacterNativeAnimator {
    fn init(base: Base<Node2D>) -> Self {
        CharacterNativeAnimator {
            base,
            sprite: None,
            // Create a new instance of Animator<CharacterAnimations> struct with a default animation.
            animator: Animator::new(CharacterAnimations::Idle),
        }
    }

    fn process(&mut self, _delta: f64) {
        // Updates the animator every frame.
        self.animator.update();
    }

    fn ready(&mut self) {
        // Assign and play the `default` animation.
        self.animator.assign_sprite(self.sprite.as_ref().cloned());
        self.animator.play();

        // Assign the viewport from which the animator will calculate the direction from.
        if let Some(viewport) = self.base().get_viewport() {
            self.animator.assign_camera(viewport.get_camera_3d());
        } else {
            godot_warn!("No viewport found!");
        }
    }
}
```

## Example result

![Image](https://github.com/Just-a-Jason/gdrust-sprite3d-angle-animator/blob/main/preview.gif)
