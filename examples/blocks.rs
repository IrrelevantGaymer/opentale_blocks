#![feature(const_trait_impl)]

use opentale_blocks::{
    blocks::{
        block::Block, 
        block_types::{
            basic::Basic, full::{self, Full}, pillar::{self, Pillar}, reflectable::Reflectable, rotateable::{RotDir, Rotateable}, BlockType
        }
    }, table, with_full_paths, with_pillar_paths
};

table!(BlockType<BlockIds>, enum BlockIds, static BLOCKS = {
    let Dirt: Basic = Block::new_basic("dirt_block")
        .with_texture("dirt.gif");
    let Log: Pillar = Block::new_pillar("log")
        .with_texture("log.gif");
    let IronOre: Basic = Block::new_basic("iron_ore")
        .with_texture("iron_ore.gif")
        .with_model("ore.bbno$");
    let Furnace: Full = Block::new_full("furnace")
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
    let IronBlock: Basic = Block::new_basic("iron_block")
        .with_texture("iron_block.gif")
        .with_model("iron_block.bbno$");
});

pub fn main() {
    for block in BLOCKS {
        println!("{} has index {}", block.name(), block.index().value());
    }
}