mod animator;
mod tests;

pub use animator::Animator;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    #[default]
    Front,
    Back,
    Left,
    Right,
}

pub trait SidedAnimation {
    fn to_sided(&self, dir: Direction) -> &'static str;
}
