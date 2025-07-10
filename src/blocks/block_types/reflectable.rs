use crate::{blocks::block_types::BlockType, Buildable};

/// A Block that can be reflected across the xz plane
pub struct Reflectable<B> 
where 
    B: BlockType + Sized 
{
    /// Internal block data
    pub(crate) block_data: B
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