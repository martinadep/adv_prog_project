use tetra::{Context, input};
use tetra::graphics::{DrawParams, Texture};
use tetra::input::Key;
use tetra::math::Vec2;

use robotics_lib::utils::LibError;
use robotics_lib::world::tile::Tile;
use robotics_lib::world::World;

use crate::visualizer::{MAP_INIT_POS, MAP_SCALE, PIXEL};
use crate::worldgen::*;

type TileMatrix = (Vec<Vec<Option<Tile>>>, (usize, usize));

pub struct MapView {
    pub world: World,
    pub map: Vec<Vec<(Texture, Option<Texture>)>>,
    pub discovered_tiles: Vec<(usize, usize)>,
    pub pointer_position: (f32, f32),
}

impl MapView {
    pub fn new_from_digits(ctx: &mut Context) -> Self {
        let mut tmp = Self {
            world: init_world_from_digits(),
            map: vec![],
            discovered_tiles: vec![(0, 0), (0, 1), (1, 0), (1, 1)],
            pointer_position: (0.0, 0.0),
        };
        tmp.map = world_to_texturized_map(ctx, &tmp.world);
        tmp
    }

    ///moves the robot pointer on the map
    pub fn move_robot_pointer(&mut self, ctx: &mut Context, go_result: Result<TileMatrix, LibError>) -> tetra::Result {
        if input::get_keys_pressed(ctx).next().is_some() && go_result.is_ok() {
            let mut new_pointer_pos = (self.pointer_position.0, self.pointer_position.1);
            match input::get_keys_pressed(ctx).next().unwrap() {
                Key::A | Key::Left => {
                    new_pointer_pos.0 -= MAP_SCALE * PIXEL;
                }
                Key::D | Key::Right => {
                    new_pointer_pos.0 += MAP_SCALE * PIXEL;
                }
                Key::W | Key::Up => {
                    new_pointer_pos.1 -= MAP_SCALE * PIXEL;
                }
                Key::S | Key::Down => {
                    new_pointer_pos.1 += MAP_SCALE * PIXEL;
                }
                _ => {}
            }
            match go_result {
                Ok(tilematrix) => {
                    self.pointer_position = new_pointer_pos;
                    self.discovered_tiles.push((tilematrix.1.0, tilematrix.1.1));
                    self.discovered_tiles.push((tilematrix.1.0 + 1usize, tilematrix.1.1));
                    self.discovered_tiles.push((tilematrix.1.0, tilematrix.1.1 + 1usize));
                    if tilematrix.1.0 > 0 {
                        self.discovered_tiles.push((tilematrix.1.0 - 1usize, tilematrix.1.1));
                        self.discovered_tiles.push((tilematrix.1.0 - 1usize, tilematrix.1.1 + 1usize));
                    }
                    if tilematrix.1.1 > 0 {
                        self.discovered_tiles.push((tilematrix.1.0, tilematrix.1.1 - 1usize));
                        self.discovered_tiles.push((tilematrix.1.0 + 1usize, tilematrix.1.1 - 1usize));
                    }
                    if tilematrix.1.1 > 0 && tilematrix.1.0 > 0 {
                        self.discovered_tiles.push((tilematrix.1.0 - 1usize, tilematrix.1.1 - 1usize));
                    }
                    self.discovered_tiles.push((tilematrix.1.0 + 1usize, tilematrix.1.1 + 1usize));
                }
                Err(e) => {
                    if e != LibError::EmptyForecast {
                        println!("--- go: {:?} ---", e)
                    }
                }
            }
        }
        Ok(())
    }

    ///draws the robot on the map
    pub fn draw_robot_pointer(&mut self, ctx: &mut Context) {
        let pointer_texture = Texture::new(ctx, "./pointer.png").unwrap();
        pointer_texture.draw(
            ctx,
            DrawParams::new()
                .position(Vec2::new(MAP_INIT_POS + self.pointer_position.0, MAP_INIT_POS + self.pointer_position.1))
                .scale(Vec2::new(MAP_SCALE, MAP_SCALE)),
        )
    }
    ///draws only the discovered tiles of the world
    pub fn draw_discovered_world(&mut self, ctx: &mut Context) {
        let mut i = 0.0;
        let mut j = 0.0;
        let mut x = 0;
        for row in &self.map {
            let mut y = 0;
            for t in row {
                if self.discovered_tiles.contains(&(x, y)) {
                    t.0.draw(
                        ctx,
                        DrawParams::new()
                            .position(Vec2::new(MAP_INIT_POS + j, MAP_INIT_POS + i))
                            .scale(Vec2::new(MAP_SCALE, MAP_SCALE)),
                    );
                    /*
                    if t.1.is_some() {
                        t.1.clone().unwrap().draw(
                            ctx,
                            DrawParams::new()
                                .position(Vec2::new(MAP_INIT_POS + j, MAP_INIT_POS + i))
                                .scale(Vec2::new(MAP_SCALE, MAP_SCALE)),
                        );
                    }*/
                }
                y += 1;
                j += PIXEL * MAP_SCALE;
            }
            j = 0.0;
            i += PIXEL * MAP_SCALE;
            x += 1;
        }
    }
    ///draws the whole map
    pub fn draw_world(&mut self, ctx: &mut Context) {
        let mut i = 0.0;
        let mut j = 0.0;
        for row in &self.map {
            for (tile_texture, content_texture) in row {
                tile_texture.draw(
                    ctx,
                    DrawParams::new()
                        .position(Vec2::new(MAP_INIT_POS + j, MAP_INIT_POS + i))
                        .scale(Vec2::new(MAP_SCALE, MAP_SCALE)),
                );
                if content_texture.is_some() {
                    content_texture.clone().unwrap().draw(
                        ctx,
                        DrawParams::new()
                            .position(Vec2::new(MAP_INIT_POS + j, MAP_INIT_POS + i))
                            .scale(Vec2::new(MAP_SCALE, MAP_SCALE)),
                    );
                }
                j += PIXEL * MAP_SCALE;
            }
            j = 0.0;
            i += PIXEL * MAP_SCALE;
        }
    }
}

