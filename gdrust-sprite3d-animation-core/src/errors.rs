#[derive(PartialEq, Debug)]
pub enum AnimatorError {
    SpriteNotSetError(&'static str),
    CameraNotSetError(&'static str),
    ChangingAnimationFailed(&'static str),
}
