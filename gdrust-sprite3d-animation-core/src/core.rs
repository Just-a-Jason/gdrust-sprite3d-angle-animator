use crate::Direction;
use godot::{builtin::Vector3, obj::Gd};

const SIDE_ANGLE: f32 = 155.0;
const BACK_ANGLE: f32 = 65.0;

pub fn calculate_dir(
    camera3d: &Gd<godot::classes::Camera3D>,
    sprite: &Gd<godot::classes::AnimatedSprite3D>,
) -> Direction {
    let forward = camera3d.get_global_transform().basis.col_c();
    let cam_forward = Vector3::new(forward.x, 0.0, forward.z).normalized();
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
