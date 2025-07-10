use crate::{blocks::block_types::BlockType, Buildable};

/// A Block where all faces use the same texture and model
pub struct Basic {
    /// The block's technical name
    pub(crate) name: &'static str,
    /// If no texture is defined, the default texture (a debug texture) will be used.
    pub(crate) texture: Option<&'static str>,
    /// If no model is defined, the default model (a normal cube face) will be used.
    pub(crate) model: Option<&'static str>,
    /// a number that represents this block in memory
    pub(crate) id: usize,
    /// a value that represents the index 
    /// of this block's corresponding texture and material
    pub(crate) index: usize,
}

impl Basic {
    pub const fn with_texture(mut self, texture: &'static str) -> Self {
        self.texture = Some(texture);
        self
    } 

    pub const fn with_model(mut self, model: &'static str) -> Self {
        self.model = Some(model);
        self
    } 
}

impl const Buildable for Basic {
    fn new_with_name(name: &'static str) -> Self {
        Basic {
            name,
            texture: None,
            model: None,
            id: 0,
            index: 0
        }
    }
    fn get_texture_size() -> usize {1usize}
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

impl BlockType for Basic {
    /// The block's technical name
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