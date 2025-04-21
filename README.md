![Image](https://github.com/user-attachments/assets/9a0f8d6e-c7b0-4fda-9b00-d52502d2be8a)

`gdrust-sprite3d-angle-animator` is a Rust library for animating 3D sprites based on the camera's viewing direction—similar to the classic Doom-style visuals. It allows for smooth sprite angle switching and animation control depending on where the player is looking.

# Animator trait

```rs
pub trait Animator<T: SidedAnimation + Copy> {
    //Can fail ❌
    fn change_animation(&mut self, animation: T) -> Result<(), AnimatorError>;
    fn update(&mut self) -> Result<(), AnimatorError>;
    fn play(&mut self) -> Result<(), AnimatorError>;
    fn pause(&mut self) -> Result<(), AnimatorError>;

    // Cannot fail ✅
    fn set_camera(&mut self, camera: &godot::obj::Gd<godot::classes::Camera3D>);
    fn get_animation(&self) -> &T;
    fn get_direction(&self) -> Direction;
}
```

> [!IMPORTANT]
> This crate was tested in godot environment with a few characters like this one on the preview bellow it's very powerfull and very fast.

![PREVIEW](https://github.com/Just-a-Jason/gdrust-sprite3d-angle-animator/blob/main/preview.gif)

> [!WARNING] > `To be able to use it you have to add all animation variants like for example by using this derive macro to generate all animation names at compile time`

```rs
#[derive(SidedAnimation)]
enum MyAnimations {
    Idle,
    Walk
}
```

> [!WARNING]
> You have to create all of those animations on your `3DAnimatedSprite` node.
>
> - `idle_front`
> - `idle_side`
> - `idle_back`
> - `walk_front`
> - `walk_side`
> - `walk_back` \
>   The derive macro `SidedAnimation` compiles `&'static str` reference to all of your animations with direction prefix.
>   The `Left/Right` direction compiles to => `{your animation name}_side` and then the `Animator` struct flips it.

## Example usage of SS3DAnimator (Single Sprite 3D Animator)

```rs
use gdrust_sprite3d_angle_animator::prelude::*;
use godot::{classes::AnimatedSprite3D, prelude::*};

const TIMER_MAX: f64 = 5.0;

// Must derive SidedAnimation and Copy trait
#[derive(SidedAnimation, Debug, Copy, Clone)]
pub enum CharacterAnimation {
    Idle,
    Walk,
}

// Example godot class
#[derive(GodotClass)]
#[class(base=Node)]
struct CharacterNativeAnimator {
    base: Base<Node>,

    // Our sprite exported to the editor
    #[export]
    sprite: Option<Gd<AnimatedSprite3D>>,

    // Single sprite animator struct
    animator: SS3DAnimator<CharacterAnimation>,
    timer: f64,
}

#[godot_api]
impl INode for CharacterNativeAnimator {
    fn init(base: Base<Node>) -> Self {
        CharacterNativeAnimator {
            base,
            sprite: None,
            // Initialize the animator
            animator: SS3DAnimator::new(CharacterAnimation::Idle),
            timer: TIMER_MAX,
        }
    }

    fn process(&mut self, delta: f64) {
        self.timer -= delta;

        if self.timer <= 0.0 {
            {
                let animator = &mut self.animator;

                match animator.get_animation() {
                    CharacterAnimation::Idle => {
                        animator.change_animation(CharacterAnimation::Walk).unwrap();
                    }
                    CharacterAnimation::Walk => {
                        animator.change_animation(CharacterAnimation::Idle).unwrap();
                    }
                }

                godot_print!("current animation: {:?}", animator.get_animation());
            }

            self.timer = TIMER_MAX;
        }

        /* update(&mut self) ->  Result<(), AnimatorError>
         If you set the sprite and camera you can safely unwrap it.
         */
        self.animator.update().unwrap();
    }

    fn ready(&mut self) {
        // Set the sprite on the animator
        self.animator.set_sprite(&self.sprite.as_mut().unwrap());

        // Set camera
        if let Some(viewport) = self.base().get_viewport() {
            if let Some(camera) = viewport.get_camera_3d() {
                self.animator.set_camera(&camera);
            }
        } else {
            godot_warn!("No viewport found!");
        }

        /* play(&mut self) -> Result<(), AnimatorError>
         If you set the sprite and camera you can safely unwrap it.
         */
        self.animator.play().unwrap();
    }
}
```

## Example usage of MS3DAnimator (Multi Sprite 3D Animator)

```rs
use gdrust_sprite3d_angle_animator::prelude::*;
use godot::{classes::AnimatedSprite3D, prelude::*};

const TIMER_MAX: f64 = 5.0;

// Must derive SidedAnimation and Copy trait
#[derive(SidedAnimation, Debug, Copy, Clone)]
pub enum CharacterAnimation {
    Idle,
    Walk,
}

// Example godot class
#[derive(GodotClass)]
#[class(base=Node)]
struct CharacterNativeAnimator {
    base: Base<Node>,

    // Our sprite exported to the editor
    #[export]
    sprites: Array<Gd<AnimatedSprite3D>>,

    // Single sprite animator struct
    animator: MS3DAnimator<CharacterAnimation>,
    timer: f64,
}

#[godot_api]
impl INode for CharacterNativeAnimator {
    fn init(base: Base<Node>) -> Self {
        CharacterNativeAnimator {
            base,
            sprites: Array::new(),
            // Initialize the animator
            animator: MS3DAnimator::new(CharacterAnimation::Idle),
            timer: TIMER_MAX,
        }
    }

    fn process(&mut self, delta: f64) {
        self.timer -= delta;

        if self.timer <= 0.0 {
            {
                let animator = &mut self.animator;

                match animator.get_animation() {
                    CharacterAnimation::Idle => {
                        animator.change_animation(CharacterAnimation::Walk).unwrap();
                    }
                    CharacterAnimation::Walk => {
                        animator.change_animation(CharacterAnimation::Idle).unwrap();
                    }
                }

                godot_print!("current animation: {:?}", animator.get_animation());
            }

            self.timer = TIMER_MAX;
        }

        /* update(&mut self) ->  Result<(), AnimatorError>
         If you set the sprite and camera you can safely unwrap it.
         */
        self.animator.update().unwrap();
    }

    fn ready(&mut self) {
        // Set the sprite on the animator
        self.animator.set_sprites(&self.sprites);

        // Set camera
        if let Some(viewport) = self.base().get_viewport() {
            if let Some(camera) = viewport.get_camera_3d() {
                self.animator.set_camera(&camera);
            }
        } else {
            godot_warn!("No viewport found!");
        }

        /* play(&mut self) -> Result<(), AnimatorError>
         If you set the sprite and camera you can safely unwrap it.
         */
        self.animator.play().unwrap();
    }
}
```

> [!IMPORTANT]
> Get current animation `enum <T: SidedAnimation>`

```rs
self.animator.get_animation();
```

> [!IMPORTANT]
> Get facing direction `enum Direction`

```rs
self.animator.get_direction()
```
