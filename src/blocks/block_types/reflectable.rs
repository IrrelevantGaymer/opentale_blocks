use crate::{blocks::block_types::BlockType, AsId, Buildable, HasBuildVariants};

/// A Block that can be reflected across the xz plane
pub struct Reflectable<B> 
where 
    B: BlockType + Sized 
{
    /// Internal block data
    pub(crate) block_data: B
}

pub enum ReflectionFacing {
    Up, Down
}

impl AsId for ReflectionFacing {
    type Name = &'static str;
    const NAME: Self::Name = "spin";

    fn from_id(id: usize) -> Self {
        match id {
            0 => Self::Up,
            1 => Self::Down,
            _ => panic!("{id} is not a valid Id for ReflectionFacing")
        }
    }

    fn to_id(&self) -> usize {
        match self {
            ReflectionFacing::Up => 0,
            ReflectionFacing::Down => 1,
        }
    }

    fn get_id_span() -> usize {
        2
    }

    fn to_string(&self) -> String {
        Self::NAME.to_string() + ": " +  match self {
            ReflectionFacing::Up => "up",
            ReflectionFacing::Down => "down",
        }
    }
}

impl<B> const Buildable for Reflectable<B> 
where 
    B: const Buildable + BlockType + Sized
{
    fn new_with_name(name: &'static str) -> Self {
        Reflectable {
            block_data: B::new_with_name(name)
        }
    }
    fn get_texture_size() -> usize {
        B::get_texture_size()
    }
    fn get_id_size() -> usize { 2 }
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

impl<B> HasBuildVariants for Reflectable<B> 
where 
    B: const Buildable + BlockType + Sized
{
    type Variants = (ReflectionFacing, B::Variants);
}

impl<B> BlockType for Reflectable<B>
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