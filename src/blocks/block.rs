use crate::{blocks::block_types::{basic::Basic, custom::Custom, full::Full, pillar::Pillar, reflectable::Reflectable, rotateable::{RotDir, Rotateable}, BlockType}, Buildable};

pub struct Block;

impl Block {
    /// Builds a new Basic Block with default values and a specified name.
    /// 
    /// A Basic Block is a block where all its faces uses the same texture and model.
    /// 
    /// Should only ever be used inside the `table` macro.  Blocks have
    /// pivotal data that can only be safely set inside the `table` macro.
    pub const fn new_basic(name: &'static str) -> Basic {
        Basic::new_with_name(name)
    }

    /// Builds a new Custom Block with default values and a specified name.
    /// 
    /// A Custom Block is a block with a custom model that is not culled by
    /// nor culls other faces.
    /// 
    /// Should only ever be used inside the `table` macro.  Blocks have
    /// pivotal data that can only be safely set inside the `table` macro.
    pub const fn new_custom(name: &'static str) -> Custom {
        Custom::new_with_name(name)
    }

    /// Builds a new Full Block with default values and a specified name.
    /// 
    /// A Full Block is a block where each face has a different texture and model.
    /// 
    /// Should only ever be used inside the `table` macro.  Blocks have
    /// pivotal data that can only be safely set inside the `table` macro.
    pub const fn new_full(name: &'static str) -> Full {
        Full::new_with_name(name)
    }

    /// Builds a new Pillar Block with default values and a specified name.
    /// 
    /// A Pillar Block is a block where the top, bottom, and side faces 
    /// have different textures and models, meaning that each side 
    /// (north, west, east, and south faces) have the same texture and model.
    /// 
    /// Should only ever be used inside the `table` macro.  Blocks have
    /// pivotal data that can only be safely set inside the `table` macro.
    pub const fn new_pillar(name: &'static str) -> Pillar {
        Pillar::new_with_name(name)
    }

    /// Builds a new Rotateable Block from another Block.
    /// 
    /// A Rotateable Block is a block that can be rotated in various ways 
    /// constrained by a [`RotationDir`].  The type Rotateable only
    /// stores rotation information, and can generically represent 
    /// Rotateable versions of any block such as a Rotateable Pillar or Full.
    /// 
    /// ### Examples
    /// 
    /// ```rust
    /// # #![feature(const_trait_impl)]
    /// # 
    /// # use opentale_blocks::{blocks::{block::Block, block_types::{full::Full, rotateable::{Rotateable, RotDir}}}, with_full_paths};
    /// 
    /// const ROTATEABLE_FULL: Rotateable<Full, {RotDir::Y}> = Block::new_rotateable(
    ///     Block::new_full("block")
    ///     .with_textures(with_full_paths! {
    ///         north: "north.gif",
    ///         west: "west.gif",
    ///         east: "east.gif",
    ///         south: "south.gif"
    ///     })
    ///     .with_models(with_full_paths! {
    ///         north: "north.bbno$"
    ///     })
    /// );
    /// 
    /// ```
    pub const fn new_rotateable<B, const ROT: RotDir>(
        block: B
    ) -> Rotateable<B, ROT> where 
        B: BlockType + Sized
    {
        Rotateable {
            block_data: block
        }
    }

    /// Builds a new Reflectable Block from another Block.
    /// 
    /// A Reflectable Block is a block that can be reflected
    /// across the xz plane, so anything with a Top or Bottom orientation.  
    /// The type Reflectable only stores rotation information, 
    /// and can generically represent Reflectable versions of any block 
    /// such as a Reflectable Pillar or Full.
    /// 
    /// ### Examples
    /// 
    /// ```rust
    /// # #![feature(const_trait_impl)]
    /// # 
    /// # use opentale_blocks::{with_full_paths, blocks::{block::Block, block_types::{full::Full, reflectable::Reflectable}}};
    /// 
    /// const REFLECTABLE_FULL: Reflectable<Full> = Block::new_reflectable(
    ///     Block::new_full("block")
    ///     .with_textures(with_full_paths! {
    ///         north: "north.gif",
    ///         west: "west.gif",
    ///         east: "east.gif",
    ///         south: "south.gif"
    ///     })
    ///     .with_models(with_full_paths! {
    ///         north: "north.bbno$"
    ///     })
    /// );
    /// 
    /// ```
    pub const fn new_reflectable<B>(
        block: B
    ) -> Reflectable<B> where 
        B: BlockType + Sized
    {
        Reflectable {
            block_data: block
        }
    }
}

/// The direction of a face.
/// 
/// Used for relative indexing from a texture index
pub enum FaceDir {
    Up, North, West, East, South, Down
}

/// A Flag representing the culling properties of a face 
/// and its corresponding texture and model
pub enum CullingFlag {
    /// This face can not be culled nor does it cull other faces. 
    /// 
    /// Useful for transparent blocks, or blocks that aren't the size of a full block.
    /// 
    /// For example: glass, fences, water, and torches
    None = 0,
    /// Can cull other faces, but can't be culled itself.
    /// 
    /// Useful for blocks whose models' bounding box 
    /// is larger than a standard block.
    Cullable = 1,
    /// Can be culled itself, but can't cull other faces.
    /// 
    /// Useful for blocks whose models' bounding box
    /// is smaller than a standard block
    /// 
    /// For example: stairs, slopes, and slabs
    Culling = 2,
    /// Can cull and be culled by other faces.
    ///
    /// Useful for most solid blocks
    Both = 3
}