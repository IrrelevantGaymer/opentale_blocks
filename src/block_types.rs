use crate::Buildable;

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

pub trait BlockType: Send + Sync {
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
}

pub enum Face {
    Up, North, West, East, South, Down
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

impl const Buildable for Basic {
    fn get_size() -> usize {1usize}
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
        self.texture = full::Texture::Texture(Some(texture));
        self
    }
    
    pub const fn with_textures(mut self, textures: full::Paths) -> Self {
        self.texture = full::Texture::Textures(textures);
        self
    } 

    pub const fn with_models(mut self, models: full::Paths) -> Self {
        self.models = models;
        self
    } 
}

impl const Buildable for Full {
    fn get_size() -> usize {6usize}
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

impl const Buildable for Pillar {
    fn get_size() -> usize {3usize}
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