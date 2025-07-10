pub mod basic;
pub mod custom;
pub mod full;
pub mod pillar;
pub mod rotateable;
pub mod reflectable;

pub trait BlockType: Send + Sync {
    fn name(&self) -> &'static str;
    fn id(&self) -> usize;
    fn index(&self) -> usize;
}