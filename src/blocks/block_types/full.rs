use crate::{blocks::{block_types::BlockType, const_into::ConstInto}, Buildable};

/// A Block where each face has a unique texture and model
pub struct Full {
    /// The block's technical name
    pub(crate) name: &'static str,
    /// If no texture is defined, the default texture (a debug texture) will be used.
    pub(crate) texture: FullTexture,
    /// If no model is defined, the default model (a normal cube face) will be used.
    pub(crate) models: PathsInternal,
    /// a number that represents this block in memory
    pub(crate) id: usize,
    /// a value that represents the index 
    /// of this block's corresponding texture and material
    pub(crate) index: usize,
}

impl Full {
    pub const fn with_texture(mut self, texture: &'static str) -> Self {
        self.texture = FullTexture::Single(Some(texture));
        self
    }
    
    pub const fn with_textures(
        mut self, 
        textures: impl const ConstInto<PathsInternal>
    ) -> Self {
        self.texture = FullTexture::Multiple(textures.const_into());
        self
    } 

    pub const fn with_models(
        mut self, 
        models: impl const ConstInto<PathsInternal>
    ) -> Self {
        self.models = models.const_into();
        self
    } 
}

impl const Buildable for Full {
    fn new_with_name(name: &'static str) -> Self {
        Full {
            name,
            texture: FullTexture::Single(None),
            models: PathsInternal { 
                up: None, 
                north: None, 
                west: None, 
                east: None, 
                south: None, 
                down: None 
            },
            id: 0,
            index: 0
        }
    }
    fn get_texture_size() -> usize {6usize}
    fn with_index(mut self, idx: usize) -> Self {
        self.index = idx;
        self
    }
    fn set_index(&mut self, idx: usize) {
        self.index = idx;
    }
    fn with_id(mut self, id: usize) -> Self {
        self.id = id;
        self
    }
    fn set_id(&mut self, id: usize) {
        self.id = id;
    }
}

impl BlockType for Full {
    /// This block's technical name
    fn name(&self) -> &'static str {
        self.name
    }

    /// a value that represents the index 
    /// of this block's corresponding texture and material
    fn index(&self) -> usize {
        self.index
    }

    /// a number that represents this block in memory
    fn id(&self) -> usize {
        self.id
    }
}

pub enum FullTexture {
    Single(Option<&'static str>),
    Multiple(PathsInternal)
}

/// A strict helper struct for constructing a fully-defined [`PathsInternal`] instance.
///
/// Unlike [`with_full_paths!`], `Paths` requires **all six fields** to be
/// specified. This ensures that your code always produces a complete `PathsInternal`
/// instance with no `None` values.
///
/// This type implements [`ConstInto<PathsInternal>`], allowing you to convert it into
/// a `Paths` value, including in `const fn` contexts.
///
/// ### Example
///
/// ```rust
/// # #![feature(const_trait_impl)]
/// # 
/// # use opentale_blocks::{blocks::{block::Block, block_types::full::{self, Full}}, Indexable};
/// #
/// # #[derive(Clone, Copy)]
/// # pub struct TextureIndex;
/// # impl const Indexable for TextureIndex {
/// #     fn value(&self) -> usize { 0 }
/// #     fn default() -> Self { TextureIndex }
/// # }
///
/// const BLOCK: Full<TextureIndex> = Block::new_full("block")
///     .with_textures(full::Paths {
///         up: "top.png",
///         north: "north.png",
///         south: "south.png",
///         east: "east.png",
///         west: "west.png",
///         down: "bottom.png",
///     }
/// );
/// ```
///
/// ### Use Case
///
/// Use this when you want compile-time guarantees that all faces are defined.
///
/// ---
///
/// To be able to set only some paths, see [`with_full_paths!`] instead.
pub struct Paths {
    pub up: &'static str,
    pub north: &'static str,
    pub west: &'static str,
    pub east: &'static str,
    pub south: &'static str,
    pub down: &'static str,
}

impl const ConstInto<PathsInternal> for Paths {
    fn const_into(self) -> PathsInternal {
        PathsInternal { 
            up: Some(self.up), 
            north: Some(self.north), 
            west: Some(self.west), 
            east: Some(self.east), 
            south: Some(self.south), 
            down: Some(self.down) 
        }
    }
}

/// A struct that stores paths corresponding to each face of a Full Block.
/// 
/// You should never construct a `PathsInternal` instance yourself, instead
/// use the [`with_full_paths!`] macro or the [`Paths`] helper struct.
pub struct PathsInternal {
    pub up: Option<&'static str>,
    pub north: Option<&'static str>,
    pub west: Option<&'static str>,
    pub east: Option<&'static str>,
    pub south: Option<&'static str>,
    pub down: Option<&'static str>,
}

/// Constructs a `Paths` instance with only the fields you specify.
///
/// Unspecified fields are set to `None`. This is useful for defining partial
/// sets of paths without having to write out every field manually.
///
/// ### Example
///
/// ```rust
/// # #![feature(const_trait_impl)]
/// # 
/// # use opentale_blocks::{with_full_paths, blocks::{block::Block, block_types::full::{self, Full}}, Indexable};
/// #
/// # #[derive(Clone, Copy)]
/// # pub struct TextureIndex;
/// # impl const Indexable for TextureIndex {
/// #     fn value(&self) -> usize { 0 }
/// #     fn default() -> Self { TextureIndex }
/// # }
///
/// const BLOCK: Full<TextureIndex> = Block::new_full("block")
///     .with_textures(with_full_paths! {
///         up: "top.png",
///         north: "north.png",
///         south: "south.png",
///         down: "bottom.png",
///     }
/// );
/// ```
///
/// ### Use Case
///
/// Use this macro when you only care about a subset of paths.
///
/// ---
///
/// For stricter requirements (e.g. all fields must be defined),
/// see [`Paths`] instead.
#[macro_export]
macro_rules! with_full_paths {
    {$($face:ident : $path:expr),* $(,)?} => {
        {
            let mut paths = $crate::blocks::block_types::full::PathsInternal {
                up: None,
                north: None,
                west: None,
                east: None,
                south: None,
                down: None,
            };
            $(
                paths.$face = Some($path);
            )*
            paths
        }
    };
}