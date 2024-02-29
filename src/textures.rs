use tetra::Context;
use tetra::graphics::Texture;
use robotics_lib::runner::Robot;
use robotics_lib::world::tile::{Content, Tile, TileType};
use robotics_lib::world::tile::TileType::*;

const DEFAULT_TEXTURE_PATH : &str = "./textures/error_texture.png";
pub trait Texturizable {
    fn get_texture(&self, ctx: &mut Context) -> Texture;
}
impl Texturizable for Tile{
    fn get_texture(&self, ctx: &mut Context) -> Texture {
        match self.tile_type {
            DeepWater =>Texture::new(ctx, "./tiles/DeepWater.png").unwrap_or(Texture::new(ctx, DEFAULT_TEXTURE_PATH).unwrap()),
            ShallowWater => Texture::new(ctx, "./tiles/ShallowWater.png").unwrap_or(Texture::new(ctx, DEFAULT_TEXTURE_PATH).unwrap()),
            Sand => Texture::new(ctx, "./tiles/Sand.png").unwrap_or(Texture::new(ctx, DEFAULT_TEXTURE_PATH).unwrap()),
            Grass => Texture::new(ctx, "./tiles/Grass.png").unwrap_or(Texture::new(ctx, DEFAULT_TEXTURE_PATH).unwrap()),
            Street => Texture::new(ctx, "./tiles/Street.png").unwrap_or(Texture::new(ctx, DEFAULT_TEXTURE_PATH).unwrap()),
            Hill => Texture::new(ctx, "./tiles/Hill.png").unwrap_or(Texture::new(ctx, DEFAULT_TEXTURE_PATH).unwrap()),
            Mountain => Texture::new(ctx, "./tiles/Mountain.png").unwrap_or(Texture::new(ctx, DEFAULT_TEXTURE_PATH).unwrap()),
            Snow => Texture::new(ctx, "./tiles/Snow.png").unwrap_or(Texture::new(ctx, DEFAULT_TEXTURE_PATH).unwrap()),
            Lava => Texture::new(ctx, "./tiles/Lava.png").unwrap_or(Texture::new(ctx, DEFAULT_TEXTURE_PATH).unwrap()),
            Teleport(_) => Texture::new(ctx, "./tiles/Teleport.png").unwrap_or(Texture::new(ctx, DEFAULT_TEXTURE_PATH).unwrap()),
            Wall => Texture::new(ctx, "./tiles/Wall.png").unwrap_or(Texture::new(ctx, DEFAULT_TEXTURE_PATH).unwrap()),
        }
    }
}
pub fn texturize_content(content : &Content, ctx : &mut Context) -> Option<Texture>{
    match content {
        Content::Rock(_) => Some(Texture::new(ctx, "./content/Rock.png").unwrap_or(Texture::new(ctx, DEFAULT_TEXTURE_PATH).unwrap())),
        Content::Tree(_) => Some(Texture::new(ctx, "./content/Tree.png").unwrap_or(Texture::new(ctx, DEFAULT_TEXTURE_PATH).unwrap())),
        Content::Garbage(_) => Some(Texture::new(ctx, "./content/Garbage.png").unwrap_or(Texture::new(ctx, DEFAULT_TEXTURE_PATH).unwrap())),
        Content::Fire => Some(Texture::new(ctx, "./content/Fire.png").unwrap_or(Texture::new(ctx, DEFAULT_TEXTURE_PATH).unwrap())),
        Content::Coin(_) => Some(Texture::new(ctx, "./content/Coin.png").unwrap_or(Texture::new(ctx, DEFAULT_TEXTURE_PATH).unwrap())),
        Content::Bin(_) => Some(Texture::new(ctx, "./content/Bin.png").unwrap_or(Texture::new(ctx, DEFAULT_TEXTURE_PATH).unwrap())),
        Content::Crate(_) => Some(Texture::new(ctx, "./content/Crate.png").unwrap_or(Texture::new(ctx, DEFAULT_TEXTURE_PATH).unwrap())),
        Content::Bank(_) => Some(Texture::new(ctx, "./content/Bank.png").unwrap_or(Texture::new(ctx, DEFAULT_TEXTURE_PATH).unwrap())),
        Content::Water(_) => None,
        Content::Market(_) => Some(Texture::new(ctx, "./content/Market.png").unwrap_or(Texture::new(ctx, DEFAULT_TEXTURE_PATH).unwrap())),
        Content::Fish(_) => Some(Texture::new(ctx, "./content/Fish.png").unwrap_or(Texture::new(ctx, DEFAULT_TEXTURE_PATH).unwrap())),
        Content::Building => Some(Texture::new(ctx, "./content/Building.png").unwrap_or(Texture::new(ctx, DEFAULT_TEXTURE_PATH).unwrap())),
        Content::Bush(_) => Some(Texture::new(ctx, "./content/Bush.png").unwrap_or(Texture::new(ctx, DEFAULT_TEXTURE_PATH).unwrap())),
        Content::JollyBlock(_) => Some(Texture::new(ctx, "./content/JollyBlock.png").unwrap_or(Texture::new(ctx, DEFAULT_TEXTURE_PATH).unwrap())),
        Content::Scarecrow => Some(Texture::new(ctx, "./content/Scarecrow.png").unwrap_or(Texture::new(ctx, DEFAULT_TEXTURE_PATH).unwrap())),
        Content::None => None,
    }
}

impl Texturizable for Content{
    fn get_texture(&self, ctx: &mut Context) -> Texture {
        match self {
            Content::Rock(_) => Texture::new(ctx, "./content/Rock.png").unwrap_or(Texture::new(ctx, DEFAULT_TEXTURE_PATH).unwrap()),
            Content::Tree(_) => Texture::new(ctx, "./content/Tree.png").unwrap_or(Texture::new(ctx, DEFAULT_TEXTURE_PATH).unwrap()),
            Content::Garbage(_) => Texture::new(ctx, "./content/Garbage.png").unwrap_or(Texture::new(ctx, DEFAULT_TEXTURE_PATH).unwrap()),
            Content::Fire => Texture::new(ctx, "./content/Fire.png").unwrap_or(Texture::new(ctx, DEFAULT_TEXTURE_PATH).unwrap()),
            Content::Coin(_) => Texture::new(ctx, "./content/Coin.png").unwrap_or(Texture::new(ctx, DEFAULT_TEXTURE_PATH).unwrap()),
            Content::Bin(_) => Texture::new(ctx, "./content/Bin.png").unwrap_or(Texture::new(ctx, DEFAULT_TEXTURE_PATH).unwrap()),
            Content::Crate(_) => Texture::new(ctx, "./content/Crate.png").unwrap_or(Texture::new(ctx, DEFAULT_TEXTURE_PATH).unwrap()),
            Content::Bank(_) => Texture::new(ctx, "./content/Bank.png").unwrap_or(Texture::new(ctx, DEFAULT_TEXTURE_PATH).unwrap()),
            Content::Water(_) => Texture::new(ctx, "./content/").unwrap_or(Texture::new(ctx, DEFAULT_TEXTURE_PATH).unwrap()),
            Content::Market(_) => Texture::new(ctx, "./content/Market.png").unwrap_or(Texture::new(ctx, DEFAULT_TEXTURE_PATH).unwrap()),
            Content::Fish(_) => Texture::new(ctx, "./content/Fish.png").unwrap_or(Texture::new(ctx, DEFAULT_TEXTURE_PATH).unwrap()),
            Content::Building => Texture::new(ctx, "./content/Building.png").unwrap_or(Texture::new(ctx, DEFAULT_TEXTURE_PATH).unwrap()),
            Content::Bush(_) => Texture::new(ctx, "./content/Bush.png").unwrap_or(Texture::new(ctx, DEFAULT_TEXTURE_PATH).unwrap()),
            Content::JollyBlock(_) => Texture::new(ctx, "./content/JollyBlock.png").unwrap_or(Texture::new(ctx, DEFAULT_TEXTURE_PATH).unwrap()),
            Content::Scarecrow => Texture::new(ctx, "./content/Scarecrow.png").unwrap_or(Texture::new(ctx, DEFAULT_TEXTURE_PATH).unwrap()),
            Content::None => Texture::new(ctx, "./content/").unwrap_or(Texture::new(ctx, DEFAULT_TEXTURE_PATH).unwrap()),
        }
    }
}
impl Texturizable for Robot{
    fn get_texture(&self, ctx: &mut Context) -> Texture {
        Texture::new(ctx, "./textures/player.png").unwrap_or(Texture::new(ctx, DEFAULT_TEXTURE_PATH).unwrap())
    }
}

///It returns the texture of the texturizable object
pub(crate) fn get_texture(text_obj: Box<dyn Texturizable>, ctx: &mut Context) -> Texture {
    text_obj.get_texture(ctx)
}

///This associates a number to a TyleType
pub(crate) fn get_type(i: i32) -> TileType {
    match i {
        0 => DeepWater,
        1 => ShallowWater,
        2 => Sand,
        3 => Grass,
        4 => Street,
        5 => Hill,
        6 => Mountain,
        7 => Snow,
        _ => Lava,
    }
}
