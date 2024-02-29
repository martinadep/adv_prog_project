use rand::Rng;
use tetra::graphics::{Texture};
use tetra::{Context};

use robotics_lib::world::environmental_conditions::{EnvironmentalConditions, WeatherType};
use robotics_lib::world::tile::Content::*;
use robotics_lib::world::tile::TileType::*;
use robotics_lib::world::tile::{Tile};
use robotics_lib::world::World;

use crate::textures;
use crate::textures::texturize_content;

pub(crate) fn init_grass_map(ctx: &mut Context) -> Vec<Vec<Texture>> {
    let grass = Texture::new(ctx, "./textures/grass.png").unwrap();
    let tree = Texture::new(ctx, "./textures/tree.png").unwrap();
    let mut v: Vec<Vec<Texture>> = vec![];
    for row in 0..100 {
        let mut row_to_push = vec![];
        for t in 0..100 {
            if t % 5 == 0 && row % 3 == 0 {
                row_to_push.push(tree.clone());
            } else {
                row_to_push.push(grass.clone());
            }
        }
        v.push(row_to_push);
    }
    v
}

pub(crate) fn init_world_from_digits() -> World {
    let mut vec_to_return: Vec<Vec<Tile>> = vec![];
    let mappa = vec![
        vec![3, 3, 3, 3, 3, 3, 3, 3, 4, 3, 3, 3, 3, 4, 3, 3, 3, 3, 4, 3, 3, 3],
        vec![2, 3, 3, 3, 3, 3, 3, 3, 4, 3, 3, 3, 3, 4, 3, 3, 3, 4, 4, 3, 3, 3],
        vec![2, 2, 2, 2, 3, 3, 3, 3, 4, 3, 3, 3, 3, 4, 4, 4, 4, 4, 3, 3, 3, 3],
        vec![2, 2, 2, 2, 2, 2, 3, 3, 4, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3],
        vec![2, 2, 2, 2, 2, 2, 3, 3, 4, 3, 3, 3, 3, 3, 8, 8, 8, 3, 3, 3, 3, 3],
        vec![2, 2, 2, 2, 2, 2, 3, 3, 4, 3, 3, 3, 3, 8, 8, 8, 8, 8, 3, 3, 3, 3],
        vec![2, 2, 1, 1, 1, 2, 2, 3, 4, 4, 4, 3, 3, 8, 8, 8, 8, 8, 3, 3, 3, 3],
        vec![2, 1, 1, 0, 1, 1, 2, 3, 3, 3, 4, 3, 3, 3, 8, 8, 8, 3, 3, 3, 3, 3],
        vec![2, 1, 0, 0, 1, 1, 2, 3, 3, 3, 4, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3],
        vec![2, 1, 1, 1, 1, 2, 2, 3, 3, 3, 4, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3],
        vec![2, 2, 1, 1, 2, 2, 3, 3, 3, 3, 4, 3, 3, 5, 5, 5, 5, 3, 3, 3, 3, 3],
        vec![2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 4, 3, 3, 5, 5, 5, 5, 5, 5, 3, 3, 3],
        vec![2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 4, 3, 5, 5, 6, 6, 6, 5, 5, 5, 5, 3],
        vec![2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 4, 3, 5, 6, 6, 6, 6, 6, 6, 6, 5, 5],
        vec![3, 3, 3, 3, 3, 3, 4, 3, 3, 3, 3, 3, 5, 6, 6, 6, 6, 6, 6, 6, 6, 5],
        vec![3, 3, 4, 4, 4, 4, 4, 3, 3, 3, 5, 5, 5, 6, 6, 6, 7, 7, 6, 6, 6, 6],
        vec![3, 3, 4, 3, 3, 3, 3, 3, 5, 5, 5, 5, 6, 6, 6, 7, 7, 7, 7, 6, 6, 6],
        vec![3, 3, 4, 3, 3, 3, 3, 3, 5, 5, 5, 5, 6, 6, 6, 6, 7, 7, 7, 6, 5, 5],
        vec![3, 3, 4, 3, 3, 3, 3, 3, 3, 5, 5, 5, 5, 6, 6, 6, 6, 7, 6, 6, 5, 5],
        vec![3, 3, 4, 3, 3, 3, 3, 3, 3, 3, 5, 5, 5, 5, 6, 6, 6, 6, 6, 6, 6, 5],
        vec![3, 3, 4, 3, 3, 3, 3, 3, 3, 3, 5, 5, 5, 5, 6, 6, 6, 6, 6, 6, 6, 5],
        vec![3, 3, 4, 3, 3, 3, 3, 3, 3, 3, 3, 5, 5, 5, 6, 6, 6, 6, 6, 6, 6, 5],
        //0 - DeepWater, 1 ShallowWater, 2 Sand, 3 Grass, 4 Street, 5 Hil
        /* vec![3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3],
         vec![3, 3, 3, 3, 3, 3, 3, 3, 3, 1, 1, 1, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3],
         vec![3, 3, 3, 3, 3, 3, 3, 3, 1, 3, 3, 3, 1, 3, 3, 3, 3, 3, 3, 3, 3, 3],
         vec![3, 3, 3, 3, 3, 3, 3, 3, 1, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3],
         vec![3, 3, 3, 3, 3, 3, 3, 3, 3, 1, 1, 1, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3],
         vec![3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 1, 3, 3, 3, 3, 3, 3, 3, 3, 3],
         vec![3, 3, 3, 3, 3, 3, 3, 3, 1, 3, 3, 3, 1, 3, 3, 3, 3, 3, 3, 3, 3, 3],
         vec![3, 3, 3, 3, 3, 3, 3, 3, 3, 1, 1, 1, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3],
         vec![3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3],
         vec![3, 3, 3, 3, 3, 7, 7, 7, 3, 3, 3, 3, 3, 7, 7, 7, 3, 3, 3, 3, 3, 3],
         vec![3, 3, 3, 3, 7, 3, 3, 3, 7, 3, 3, 3, 7, 3, 3, 3, 7, 3, 3, 3, 3, 3],
         vec![3, 3, 3, 7, 3, 3, 3, 3, 3, 7, 3, 7, 3, 3, 3, 3, 3, 7, 3, 3, 3, 3],
         vec![3, 3, 3, 7, 3, 3, 3, 3, 3, 3, 7, 3, 3, 3, 3, 3, 3, 7, 3, 3, 3, 3],
         vec![3, 3, 3, 3, 7, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 7, 3, 3, 3, 3, 3],
         vec![3, 3, 3, 3, 3, 7, 3, 3, 3, 3, 3, 3, 3, 3, 3, 7, 3, 3, 3, 3, 3, 3],
         vec![3, 3, 3, 3, 3, 3, 7, 3, 3, 3, 3, 3, 3, 3, 7, 3, 3, 3, 3, 3, 3, 3],
         vec![3, 3, 3, 3, 3, 3, 3, 7, 3, 3, 3, 3, 3, 7, 3, 3, 3, 3, 3, 3, 3, 3],
         vec![3, 3, 3, 3, 3, 3, 3, 3, 7, 3, 3, 3, 7, 3, 3, 3, 3, 3, 3, 3, 3, 3],
         vec![3, 3, 3, 3, 3, 3, 3, 3, 3, 7, 3, 7, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3],
         vec![3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 7, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3],
         vec![3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3],
         vec![3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3],*/
    ];
    let mut xr = 0.0;
    let mut xc = 0.0;

    let mut i = 0;
    for row in &mappa {
        let mut row_to_insert = vec![];
        for elem in row {
            let tile_to_insert;
            if i % 3 == 0 {
                tile_to_insert = Tile {
                    tile_type: textures::get_type(*elem),
                    content: Fire,
                    elevation: 0,
                };
            } else {
                tile_to_insert = Tile {
                    tile_type: textures::get_type(*elem),
                    content: None,
                    elevation: 0,
                };
            }
            xc += 1.0;
            row_to_insert.push(tile_to_insert);
        }
        xc = 0.0;
        xr += 1.0;
        vec_to_return.push(row_to_insert);
        i += 1;
    }
    World {
        map: vec_to_return,
        dimension: mappa.len(),
        discoverable: 30,
        environmental_conditions: EnvironmentalConditions::new(&[WeatherType::Sunny], 0, 0).unwrap(),
        score_counter: Default::default(),
    }
}

pub(crate) fn world_to_texturized_map(ctx: &mut Context, world: &World) -> Vec<Vec<(Texture, Option<Texture>)>> {
    let mut map: Vec<Vec<(Texture, Option<Texture>)>> = vec![];
    for row in &world.map {
        let mut row_to_push = vec![];
        for t in row {
            let tile = t.clone();
            let tile_texture = textures::get_texture(Box::new(tile.clone()), ctx);
            let content_texture = texturize_content(&tile.content.clone(), ctx);
            row_to_push.push((tile_texture, content_texture))
        }
        map.push(row_to_push);
    }
    map
}
