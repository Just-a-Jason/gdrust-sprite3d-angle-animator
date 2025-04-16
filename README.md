![Image](https://github.com/user-attachments/assets/9a0f8d6e-c7b0-4fda-9b00-d52502d2be8a)

`gdrust-sprite3d-angle-animator` is a Rust library for animating 3D sprites based on the camera's viewing direction—similar to the classic Doom-style visuals. It allows for smooth sprite angle switching and animation control depending on where the player is looking.

![Image](https://github.com/Just-a-Jason/gdrust-sprite3d-angle-animator/blob/main/preview.gif)

<span style="background-color:#fff8dc; color:#000; padding:12px; border-radius:8px; font-weight:bold; border:1px solid goldenrod; color: goldenrod;">IMPORTANT! ⚠️</span>
<br/>
<br/>
`To be able to use it you have to add all animation variants like for example by using this derive macro to generate all animation names at compile time`

```rs
#[derive(SidedAnimation)]
enum MyAnimations {
    Idle,
    Walk
}
```

<span style="color:yellow"> ⚠️ You have to create all of those animations on your `3DAnimatedSprite` node.<span>

- `idle_front`
- `idle_side`
- `idle_back`
- `walk_front`
- `walk_side`
- `walk_back`

The derive macro `SidedAnimation` compiles  `&'static str` reference to all of your animations with direction prefix.

<span style="color:yellow">⚠️ The `Left/Right` direction compiles to => `{your animation name}_side` and then the `Animator` struct flips it.</span>

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
        // Assign the 3DAnimatedSprite and then play the `default` animation.
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

## Get current animation `enum <T: SidedAnimation>`

```rs
self.animator.get_current_animation();
```

## Get facing direction `enum Direction`

```rs
self.animator.get_current_dir()
```
