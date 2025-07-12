use std::marker::ConstParamTy;

use crate::{blocks::block_types::BlockType, AsId, Buildable, HasBuildVariants};

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

pub enum YRotFacing {
    North, West, East, South
}

impl AsId for YRotFacing {
    type Name = &'static str;
    const NAME: Self::Name = "facing";
    
    fn from_id(id: usize) -> Self {
        match id {
            0 => Self::North,
            1 => Self::West,
            2 => Self::East,
            3 => Self::South,
            _ => panic!("{id} is an invalid Id for YRotFacing")
        }
    }

    fn to_id(&self) -> usize {
        match self {
            YRotFacing::North => 0,
            YRotFacing::West  => 1,
            YRotFacing::East  => 2,
            YRotFacing::South => 3,
        }
    }

    fn get_id_span() -> usize {
        4
    }

    fn to_string(&self) -> String {
        Self::NAME.to_string() + ": " + match self {
            YRotFacing::North => "north",
            YRotFacing::West  => "west",
            YRotFacing::East  => "east",
            YRotFacing::South => "south",
        }
    }
}

pub enum AxisRotFacing {
    X, Y, Z
}

impl AsId for AxisRotFacing {
    type Name = &'static str;
    const NAME: Self::Name = "axis";

    fn from_id(id: usize) -> Self {
        match id {
            0 => Self::X,
            1 => Self::Y,
            2 => Self::Z,
            _ => panic!("{id} is an invalid Id for AxisRotFacing")
        }
    }

    fn to_id(&self) -> usize {
        match self {
            AxisRotFacing::X => 0,
            AxisRotFacing::Y => 1,
            AxisRotFacing::Z => 2,
        }
    }

    fn get_id_span() -> usize {
        3
    }

    fn to_string(&self) -> String {
        Self::NAME.to_string() + ": " + match self {
            AxisRotFacing::X => "x",
            AxisRotFacing::Y => "y",
            AxisRotFacing::Z => "z",
        }
    }
}

pub enum AllRotFacing {
    Up, North, West, East, South, Down
}

impl AsId for AllRotFacing {
    type Name = &'static str;
    const NAME: Self::Name = "direction";

    fn from_id(id: usize) -> Self {
        match id {
            0 => Self::Up,
            1 => Self::North,
            2 => Self::West, 
            3 => Self::East,
            4 => Self::South,
            5 => Self::Down,
            _ => panic!("{id} is an invalid Id for AllRotFacing")
        }
    }

    fn to_id(&self) -> usize {
        match self {
            AllRotFacing::Up    => 0,
            AllRotFacing::North => 1,
            AllRotFacing::West  => 2,
            AllRotFacing::East  => 3,
            AllRotFacing::South => 4,
            AllRotFacing::Down  => 5,
        }
    }

    fn get_id_span() -> usize {
        6
    }

    fn to_string(&self) -> String {
        Self::NAME.to_string() + ": " + match self {
            AllRotFacing::Up    => "up",
            AllRotFacing::North => "north",
            AllRotFacing::West  => "west",
            AllRotFacing::East  => "east",
            AllRotFacing::South => "south",
            AllRotFacing::Down  => "down",
        }
    }
}

impl<B, const ROT: RotDir> const Buildable for Rotateable<B, ROT> 
where 
    B: const Buildable + BlockType + Sized,
    Self: HasBuildVariants
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

impl<B> HasBuildVariants for Rotateable<B, {RotDir::Y}>
where 
    B: const Buildable + BlockType + Sized
{
    type Variants = (YRotFacing, B::Variants);
}

impl<B> HasBuildVariants for Rotateable<B, {RotDir::Axis}>
where 
    B: const Buildable + BlockType + Sized
{
    type Variants = (AxisRotFacing, B::Variants);
}

impl<B> HasBuildVariants for Rotateable<B, {RotDir::All}>
where 
    B: const Buildable + BlockType + Sized
{
    type Variants = (AllRotFacing, B::Variants);
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