use std::marker::PhantomData;

use crate::{Buildable, Indexable};

pub struct Block<T: Indexable> {
    _phantom: PhantomData<T>
}

impl<T: const Indexable> Block<T> {
    pub const fn new_basic(name: &'static str) -> Basic<T> {
        Basic { name, texture: None, model: None, index: T::default() }
    }

    pub const fn new_full(name: &'static str) -> Full<T> {
        Full { 
            name,
            texture: full::Texture::Texture(None),
            models: full::Paths::default(),
            index: T::default()
        }
    }

    pub const fn new_pillar(name: &'static str) -> Pillar<T> {
        Pillar {
            name,
            texture: pillar::Texture::Texture(None),
            models: pillar::Paths::default(),
            index: T::default()
        }
    }
}

pub trait BlockType<T: const Indexable>: Send + Sync {
    fn name(&self) -> &'static str;
    fn index(&self) -> T;
}

pub enum Face {
    Up, North, West, East, South, Down
}

pub struct Basic<T: const Indexable> {
    name: &'static str,
    texture: Option<&'static str>,
    model: Option<&'static str>,
    index: T
}

impl<T: const Indexable> Basic<T> {
    pub const fn with_texture(mut self, texture: &'static str) -> Self {
        self.texture = Some(texture);
        self
    } 

    pub const fn with_model(mut self, model: &'static str) -> Self {
        self.model = Some(model);
        self
    } 
}

impl<T: const Indexable> const Buildable<T> for Basic<T> {
    fn get_size() -> usize {1usize}
    fn with_index(mut self, idx: T) -> Self {
        self.index = idx;
        self
    }
}

impl<T: const Indexable> BlockType<T> for Basic<T> {
    fn name(&self) -> &'static str {
        self.name
    }

    fn index(&self) -> T {
        self.index
    }
}

pub struct Full<T: const Indexable> {
    name: &'static str,
    texture: full::Texture,
    models: full::Paths,
    index: T
}

impl<T: const Indexable> Full<T> {
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

impl<T: const Indexable> const Buildable<T> for Full<T> {
    fn get_size() -> usize {6usize}
    fn with_index(mut self, idx: T) -> Self {
        self.index = idx;
        self
    }
}

impl<T: const Indexable> BlockType<T> for Full<T> {
    fn name(&self) -> &'static str {
        self.name
    }

    fn index(&self) -> T {
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

pub struct Pillar<T: const Indexable> {
    name: &'static str,
    texture: pillar::Texture,
    models: pillar::Paths,
    index: T
}

impl<T: const Indexable> Pillar<T> {
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

impl<T: const Indexable> const Buildable<T> for Pillar<T> {
    fn get_size() -> usize {3usize}
    fn with_index(mut self, idx: T) -> Self {
        self.index = idx;
        self
    }
}

impl<T: const Indexable> BlockType<T> for Pillar<T> {
    fn name(&self) -> &'static str {
        self.name
    }

    fn index(&self) -> T {
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