#![feature(const_trait_impl)]

use opentale_blocks::blocks;

pub struct Block;

impl Block {
    pub const fn new_basic(name: &'static str) -> Basic {
        Basic { name, texture: None, model: None, index: 0usize }
    }

    pub const fn new_full(name: &'static str) -> Full {
        Full { 
            name,
            texture: full::Texture::Texture(None),
            models: full::Paths::default(),
            index: 0
        }
    }

    pub const fn new_pillar(name: &'static str) -> Pillar {
        Pillar {
            name,
            texture: pillar::Texture::Texture(None),
            models: pillar::Paths::default(),
            index: 0
        }
    }
}

#[const_trait]
pub trait BlockTypeBuildable {
    fn with_index(self, idx: usize) -> Self;
}

pub trait BlockType: Send + Sync {
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
}

pub struct Basic {
    name: &'static str,
    texture: Option<&'static str>,
    model: Option<&'static str>,
    index: usize
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

impl const BlockTypeBuildable for Basic {
    fn with_index(mut self, idx: usize) -> Self {
        self.index = idx;
        self
    }
}

impl BlockType for Basic {
    fn name(&self) -> &'static str {
        self.name
    }

    fn index(&self) -> usize {
        self.index
    }
}

pub struct Full {
    name: &'static str,
    texture: full::Texture,
    models: full::Paths,
    index: usize
}

impl Full {
    pub const fn with_texture(mut self, texture: &'static str) -> Self {
        assert!(
            self.models.up.is_none() &&
            self.models.north.is_none() &&
            self.models.west.is_none() &&
            self.models.east.is_none() &&
            self.models.south.is_none() &&
            self.models.down.is_none(),
            "Cannot apply texture when models are defined"
        );
        self.texture = full::Texture::Texture(Some(texture));
        self
    }
    
    pub const fn with_textures(mut self, textures: full::Paths) -> Self {
        self.texture = full::Texture::Textures(textures);
        self
    } 

    pub const fn with_models(mut self, models: full::Paths) -> Self {
        assert!(
            !matches!(self.texture, full::Texture::Texture(Some(_))),
            "Cannot apply models when only a single texture is defined, use with_textures instead of with_texture"
        );
        self.models = models;
        self
    } 
}

impl const BlockTypeBuildable for Full {
    fn with_index(mut self, idx: usize) -> Self {
        self.index = idx;
        self
    }
}

impl BlockType for Full {
    fn name(&self) -> &'static str {
        self.name
    }

    fn index(&self) -> usize {
        self.index
    }
}

pub mod full {
    pub enum Texture {
        Texture(Option<&'static str>),
        Textures(Paths)
    }

    pub struct Paths {
        pub up: Option<&'static str>,
        pub north: Option<&'static str>,
        pub west: Option<&'static str>,
        pub east: Option<&'static str>,
        pub south: Option<&'static str>,
        pub down: Option<&'static str>,
    }

    impl Paths {
        pub const fn default() -> Self {
            Paths {
                up: None,
                north: None,
                west: None,
                east: None,
                south: None,
                down: None
            }
        }
    }
}

pub struct Pillar {
    name: &'static str,
    texture: pillar::Texture,
    models: pillar::Paths,
    index: usize
}

impl Pillar {
    pub const fn with_texture(mut self, texture: &'static str) -> Self {
        assert!(
            self.models.up.is_none() &&
            self.models.sides.is_none() &&
            self.models.down.is_none(),
            "Cannot apply texture when models are defined"
        );
        self.texture = pillar::Texture::Texture(Some(texture));
        self
    }

    pub const fn with_textures(mut self, textures: pillar::Paths) -> Self {
        self.texture = pillar::Texture::Textures(textures);
        self
    } 

    pub const fn with_models(mut self, models: pillar::Paths) -> Self {
        assert!(
            !matches!(self.texture, pillar::Texture::Texture(Some(_))),
            "Cannot apply models when only a single texture is defined, use with_textures instead of with_texture"
        );
        self.models = models;
        self
    } 
}

impl const BlockTypeBuildable for Pillar {
    fn with_index(mut self, idx: usize) -> Self {
        self.index = idx;
        self
    }
}

impl BlockType for Pillar {
    fn name(&self) -> &'static str {
        self.name
    }

    fn index(&self) -> usize {
        self.index
    }
}

pub mod pillar {
    pub enum Texture {
        Texture(Option<&'static str>),
        Textures(Paths),
    }

    pub struct Paths {
        pub up: Option<&'static str>,
        pub sides: Option<&'static str>,
        pub down: Option<&'static str>,
    }

    impl Paths {
        pub const fn default() -> Self {
            Paths {
                up: None,
                sides: None,
                down: None
            }
        }
    }
}

blocks! {
    DIRT: Basic = Block::new_basic("dirt_block")
        .with_texture("dirt.gif");
    LOG: Pillar = Block::new_pillar("log")
        .with_texture("log.gif");
    IRON_ORE: Basic = Block::new_basic("iron_ore")
        .with_texture("iron_ore.gif")
        .with_model("ore.bbno$");
    FURNACE: Full = Block::new_full("furnace")
        .with_textures(full::Paths {
            up: Some("furnace_top.gif"),
            north: Some("furnace_north.gif"),
            west: Some("furnace_west.gif"),
            east: Some("furnace_east.gif"),
            south: Some("furnace_south.gif"),
            down: Some("furnace_bottom.gif")
        })
        .with_models(full::Paths {
            up: Some("furnace_top.bbno$"),
            north: Some("furnace_north.bbno$"),
            west: Some("furnace_west.bbno$"),
            east: Some("furnace_east.bbno$"),
            south: Some("furnace_south.bbno$"),
            down: Some("furnace_bottom.bbno$f")
        });
    IRON_BLOCK: Basic = Block::new_basic("iron_block")
        .with_texture("iron_block.gif")
        .with_model("iron_block.bbno$");
}

pub fn main() {
    for block in BLOCKS {
        println!("{} has index {}", block.name(), block.index());
    }
}