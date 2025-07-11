#![feature(const_trait_impl)]

use opentale_blocks::{
    blocks::{
        block::Block, 
        block_types::{
            basic::Basic, custom::Custom, full::{self, Full}, pillar::{self, Pillar}, reflectable::Reflectable, rotateable::{RotDir, Rotateable}, BlockType
        }
    }, table, with_full_paths, with_pillar_paths
};

table!(BlockType, enum BlockId, static BLOCKS = {
    let Dirt: Basic = Block::new_basic("dirt_block")
        .with_texture("dirt.gif");
    let Log: Pillar = Block::new_pillar("log")
        .with_textures(with_pillar_paths! {
            up: "log_up.gif",
            down: "log_down.gif"
        });
    let IronOre: Basic = Block::new_basic("iron_ore")
        .with_texture("iron_ore.gif")
        .with_model("ore.bbno$");
    let Furnace: Rotateable<Full, {RotDir::Y}> = Block::new_rotateable(
        Block::new_full("furnace")
            .with_textures(with_full_paths! {
                up: "furnace_top.gif",
                north: "furnace_north.gif",
                down: "furnace_bottom.gif",
            })
            .with_models(full::Paths {
                up: "furnace_top.bbno$",
                north: "furnace_north.bbno$",
                west: "furnace_west.bbno$",
                east: "furnace_east.bbno$",
                south: "furnace_south.bbno$",
                down: "furnace_bottom.bbno$",
            })
        );
    let IronBlock: Basic = Block::new_basic("iron_block")
        .with_texture("iron_block.gif")
        .with_model("iron_block.bbno$");
    let StoneSlab: Reflectable<Pillar> = Block::new_reflectable(
        Block::new_pillar("stone_slab")
            .with_textures(pillar::Paths {
                up: "stone_slab_end.gif",
                sides: "stone_slab_side.gif",
                down: "stone_slab_end.gif",
            })
            .with_models(pillar::Paths {
                up: "slab_top.bbno$",
                sides: "slab_side.bbno$",
                down: "slab_bottom.bbno$",
            })
    );
    let StonePile: Custom = Block::new_custom("stone_pile")
        .with_texture("stone_pile.gif")
        .with_model("pile.bbno$");
    let SnowPile: Custom = Block::new_custom("stone_pile")
        .with_texture("snow_pile.gif")
        .with_model("pile.bbno$");
});

pub fn main() {
    for block in BLOCKS {
        println!("{} has id {} and index {}", block.name(), block.id(), block.index());
        // Here we could generate a table of textures, materials, models, etc.
        //
        // We could also decompose BLOCKS, splitting blocks with multiple block states
        // into different distinct Blocks, and create a vector that we can index into
    }
}