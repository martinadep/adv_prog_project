use std::collections::HashMap;
use tetra::graphics::{self, Color, DrawParams, Texture};
use tetra::input::{self, Key};
use tetra::math::Vec2;
use tetra::{Context, ContextBuilder, State};
use tetra::graphics::text::{Font, Text};

use robotics_lib::interface::{robot_view, teleport};
use robotics_lib::interface::{destroy, go};
use robotics_lib::interface::Direction::{Down, Left, Right, Up};
use robotics_lib::runner::Runnable;
use robotics_lib::utils::LibError;
use robotics_lib::world::tile::Content::*;
use robotics_lib::world::tile::TileType::*;
use robotics_lib::world::tile::{Tile};
use crate::world_view::MapView;

use crate::myrobot::MyRobot;
use crate::textures::{get_texture, Texturizable, texturize_content};

pub const PIXEL: f32 = 64.0;

const ROBOT_VIEW_SCALE : f32 = 0.5;
const ROBOT_INIT_POS: f32 = PIXEL / 2.0;

pub const MAP_SCALE: f32 = 0.2;
pub const MAP_INIT_POS: f32 = PIXEL * 11.0;
const WINDOW_WIDTH: i32 = 800;
const WINDOW_HEIGHT: i32 = 600;
const TEXT_OFFSET: Vec2<f32> = Vec2::new(16.0, 16.0);

pub struct GameState {
    map_view: MapView,
    robot_view_world: Vec<Vec<Option<Tile>>>,

    player : MyRobot,
    energy_text: Text,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let robot = MyRobot::new();
        let energy = robot.get_energy().get_energy_level();
        let mut gs = GameState {
            map_view: MapView::new_from_digits(ctx),
            player: robot,
            robot_view_world: vec![],
            energy_text: Text::new(
                format!("Robot Energy: {}", energy),
                Font::vector(ctx, "./fonts/roboto.ttf", 17.0)?
            ),
        };
        gs.robot_view_world = robot_view(&gs.player, &gs.map_view.world);
        Ok(gs)
    }
}
pub fn draw_robot_view(gamestate : &mut GameState, ctx: &mut Context){
    let mut i = 0.0;
    let mut j = 0.0;
    for row in robot_view(&gamestate.player, &gamestate.map_view.world) {
        for t in row {
            match t {
                Option::None => {}
                Some(tile) => {
                    tile.get_texture(ctx)
                        .draw(
                        ctx,
                        DrawParams::new()
                            //.position(Vec2::new(self.map_position.0 + j, self.map_position.1 + i))
                            .position(Vec2::new(ROBOT_INIT_POS + j, ROBOT_INIT_POS + i))
                            .scale(Vec2::new(ROBOT_VIEW_SCALE, ROBOT_VIEW_SCALE)),
                    );
                    if texturize_content(&tile.content, ctx).is_some(){
                        texturize_content(&tile.content, ctx).unwrap().draw(
                        ctx,
                        DrawParams::new()
                            //.position(Vec2::new(self.map_position.0 + j, self.map_position.1 + i))
                            .position(Vec2::new(ROBOT_INIT_POS + j, ROBOT_INIT_POS + i))
                            .scale(Vec2::new(ROBOT_VIEW_SCALE, ROBOT_VIEW_SCALE)),
                    );
                    }
                }
            }
            j += ROBOT_VIEW_SCALE * PIXEL;
        }
        j = 0.0;
        i += ROBOT_VIEW_SCALE * PIXEL;
    }
    Texture::new(ctx, "./player.png")
        .unwrap_or(Texture::new(ctx, "./error_texture.png").unwrap())
        .draw( ctx,
        DrawParams::new()
            .position(Vec2::new(ROBOT_INIT_POS + ROBOT_VIEW_SCALE * PIXEL, ROBOT_INIT_POS + ROBOT_VIEW_SCALE * PIXEL))
            .scale(Vec2::new(ROBOT_VIEW_SCALE , ROBOT_VIEW_SCALE)),
    );
}

impl State for GameState {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        if input::get_keys_pressed(ctx).next().is_some() {
            let mut go_res= Err(LibError::EmptyForecast);

            match input::get_keys_pressed(ctx).next().unwrap() {
                Key::A | Key::Left => {
                    go_res = go(&mut self.player, &mut self.map_view.world, Left);
                },
                Key::D | Key::Right => {
                    go_res = go(&mut self.player, &mut self.map_view.world, Right);
                },
                Key::W | Key::Up => {
                    go_res = go(&mut self.player, &mut self.map_view.world, Up);
                },
                Key::S | Key::Down => {
                    go_res = go(&mut self.player, &mut self.map_view.world, Down);
                },
                Key::X => {
                    let destroy_res = destroy(&mut self.player, &mut self.map_view.world, Down);
                    println!("--- destroy: {:?} ---", destroy_res)
                },
                Key::T => {
                    let teleport_res = teleport(&mut self.player, &mut self.map_view.world, (0, 0));
                    match teleport_res {
                        Ok(tilematrix) => {
                            self.robot_view_world = tilematrix.0;
                            println!("teleported");
                        }
                        Err(e) => println!("-- error during teleport: {:?}", e)
                    }
                },
                _ => {}
            }
            self.map_view.move_robot_pointer(ctx, go_res.clone())?;

            //update robot view
            match go_res {
                Ok(tilematrix) => {
                    self.robot_view_world = tilematrix.0
                }
                Err(e) => { if e != LibError::EmptyForecast{
                    println!("--- go error: {:?} ---", e)
                }
                }
            }

            //update energy
            self.energy_text.set_content(format!("Robot Energy: {}", self.player.robot.energy.get_energy_level()))
        }
        /*let mut pressed = input::get_keys_pressed(ctx).peekable();
        if pressed.peek().is_some() {
            println!("Keys pressed this update: {:?}", pressed.collect::<Vec<_>>());
        }*/

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.094, 0.11, 0.16));
        draw_robot_view(self, ctx);
        self.map_view.draw_world(ctx);
        self.map_view.draw_robot_pointer(ctx);

        self.energy_text.draw(ctx, TEXT_OFFSET + Vec2::new(0.0, 128.0));

        Ok(())
    }
}
pub fn init_window() -> tetra::Result {
    ContextBuilder::new("tyrannosauRUST-rex", WINDOW_WIDTH, WINDOW_HEIGHT)
        .quit_on_escape(true)
        .build()?
        .run(GameState::new)
}

