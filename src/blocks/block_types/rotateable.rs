use std::marker::ConstParamTy;

use crate::{blocks::block_types::BlockType, Buildable};

/// A Block that can be rotated in various ways constrained by [`RotationDir`]
pub struct Rotateable<B, const ROT: RotDir> 
where 
    B: BlockType + Sized 
{
    /// Internal block data
    pub(crate) block_data: B
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, ConstParamTy)]
pub enum RotDir {
    /// The Block can rotate around the Y axis giving 4 possible states:
    /// North, West, East, and South facing
    Y, 
    /// The Block can rotate to point towards all 3 axis: X, Y, Z
    Axis, 
    /// The Block can rotate to point to all directions: 
    /// Up, North, West, East, South, and Down
    All
}

impl<B, const ROT: RotDir> const Buildable for Rotateable<B, ROT> 
where 
    B: const Buildable + BlockType + Sized
{
    fn new_with_name(name: &'static str) -> Self {
        Rotateable {
            block_data: B::new_with_name(name)
        }
    }
    fn get_texture_size() -> usize {
        B::get_texture_size()
    }
    fn get_id_size() -> usize {
        match ROT {
            RotDir::Y => 4,
            RotDir::Axis => 3,
            RotDir::All => 6,
        }
    }
    fn with_index(mut self, idx: usize) -> Self {
        self.block_data.set_index(idx);
        self
    }
    fn set_index(&mut self, idx: usize) {
        self.block_data.set_index(idx);
    }
    fn with_id(mut self, id: usize) -> Self {
        self.block_data.set_id(id);
        self
    }
    fn set_id(&mut self, id: usize) {
        self.block_data.set_id(id);
    }
}

impl<B, const ROT: RotDir> BlockType for Rotateable<B, ROT>
where
    B: BlockType + Sized
{
    fn name(&self) -> &'static str {
        self.block_data.name()
    }

    fn id(&self) -> usize {
        self.block_data.id()
    }

    fn index(&self) -> usize {
        self.block_data.index()
    }
}