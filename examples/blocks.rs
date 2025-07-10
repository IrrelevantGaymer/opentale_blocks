#![feature(const_trait_impl)]

use opentale_blocks::{
    block_types::{full, Basic, Block, BlockType, Full, Pillar}, 
    table, Indexable
};

table!(BlockType<BlockIds>, enum BlockIds, static BLOCKS = {
    let DIRT: Basic<BlockIds> = Block::new_basic("dirt_block")
        .with_texture("dirt.gif");
    let LOG: Pillar<BlockIds> = Block::new_pillar("log")
        .with_texture("log.gif");
    let IRON_ORE: Basic<BlockIds> = Block::new_basic("iron_ore")
        .with_texture("iron_ore.gif")
        .with_model("ore.bbno$");
    let FURNACE: Full<BlockIds> = Block::new_full("furnace")
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
    let IRON_BLOCK: Basic<BlockIds> = Block::new_basic("iron_block")
        .with_texture("iron_block.gif")
        .with_model("iron_block.bbno$");
});

pub fn main() {
    for block in BLOCKS {
        println!("{} has index {}", block.name(), block.index().value());
    }
}