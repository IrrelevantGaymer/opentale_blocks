use crate::{blocks::{block_types::BlockType, const_into::ConstInto}, Buildable};

/// A Block where the top, bottom, and side faces 
/// have different textures and models, meaning that each side face 
/// (north, west, east, and south faces) have the same texture and model.
pub struct Pillar {
    /// The block's technical name
    pub(crate) name: &'static str,
    /// If no texture is defined, the default texture (a debug texture) will be used.
    pub(crate) texture: PillarTexture,
    /// If no model is defined, the default model (a normal cube face) will be used.
    pub(crate) models: PathsInternal,
    /// a number that represents this block in memory
    pub(crate) id: usize,
    /// a value that represents the index 
    /// of this block's corresponding texture and material
    pub(crate) index: usize,
}

impl Pillar {
    pub const fn with_texture(mut self, texture: &'static str) -> Self {
        self.texture = PillarTexture::Single(Some(texture));
        self
    }

    pub const fn with_textures(
        mut self, 
        textures: impl const ConstInto<PathsInternal>
    ) -> Self {
        self.texture = PillarTexture::Multiple(textures.const_into());
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

impl const Buildable for Pillar {
    fn new_with_name(name: &'static str) -> Self {
        Pillar {
            name,
            texture: PillarTexture::Single(None),
            models: PathsInternal { 
                up: None, 
                sides: None, 
                down: None 
            },
            id: 0,
            index: 0
        }
    }
    fn get_texture_size() -> usize {3usize}
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

impl BlockType for Pillar {
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

pub enum PillarTexture {
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
/// # use opentale_blocks::{blocks::{block::Block, block_types::pillar::{self, Pillar}}, Indexable};
/// #
/// # #[derive(Clone, Copy)]
/// # pub struct TextureIndex;
/// # impl const Indexable for TextureIndex {
/// #     fn value(&self) -> usize { 0 }
/// #     fn default() -> Self { TextureIndex }
/// # }
///
/// const BLOCK: Pillar<TextureIndex> = Block::new_pillar("block")
///     .with_textures(pillar::Paths {
///         up: "top.gif",
///         sides: "north.gif",
///         down: "bottom.gif",
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
    pub sides: &'static str,
    pub down: &'static str,
}

impl const ConstInto<PathsInternal> for Paths {
    fn const_into(self) -> PathsInternal {
        PathsInternal { 
            up: Some(self.up), 
            sides: Some(self.sides), 
            down: Some(self.down),
        }
    }
}

/// A struct that stores paths corresponding 
/// to the top, bottom, and side faces of a Pillar Block.
/// 
/// You should never construct a `PathsInternal` instance yourself, instead
/// use the [`with_pillar_paths!`] macro or the [`Paths`] helper struct.
pub struct PathsInternal {
    pub up: Option<&'static str>,
    pub sides: Option<&'static str>,
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
/// # use opentale_blocks::{with_pillar_paths, blocks::{block::Block, block_types::pillar::{self, Pillar}}, Indexable};
/// #
/// # #[derive(Clone, Copy)]
/// # pub struct TextureIndex;
/// # impl const Indexable for TextureIndex {
/// #     fn value(&self) -> usize { 0 }
/// #     fn default() -> Self { TextureIndex }
/// # }
///
/// const BLOCK: Pillar<TextureIndex> = Block::new_pillar("block")
///     .with_textures(with_pillar_paths! {
///         up: "top.gif",
///         down: "bottom.gif",
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
macro_rules! with_pillar_paths {
    {$($face:ident : $path:expr),* $(,)?} => {
        {
            let mut paths = $crate::blocks::block_types::pillar::PathsInternal {
                up: None,
                sides: None,
                down: None,
            };
            $(
                paths.$face = Some($path);
            )*
            paths
        }
    };
}