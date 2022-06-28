use bevy::prelude::*;
use bracket_bevy::{FontCharType, prelude::{to_cp437, RGB, *}, BracketContext};
use bracket_noise::prelude::{FastNoise, FractalType, NoiseType};

use std::time::{SystemTime, UNIX_EPOCH};

use super::DrawingComponents::GlyphRenderer;
#[derive(Component,Debug,Clone,Copy)]
pub struct Tile{
    pub tile_type: TileType,
    pub tile_glyph: FontCharType,
}

impl GlyphRenderer for Tile {
    fn render_glyph(&self, x: i32, y: i32, ctx: &bracket_bevy::BracketContext) {
        //Render glyph based on tile_type
        let glyph=match self.tile_type {
            TileType::Nothing=>to_cp437(' ') ,
            TileType::Binary0=>to_cp437('0'),
            TileType::Binary1=>to_cp437('1'),
            TileType::Ascii=>self.tile_glyph, //Defined externally,
            TileType::Conveyer=>self.tile_glyph,//Defined externally
        };
        //Set fg and bg based on tile_type
        let (fg,bg)=match self.tile_type {
            TileType::Ascii | TileType::Binary0 | TileType::Binary1 | TileType::Nothing =>(RGB::named(WHITE),RGB::named(BLACK)),
            TileType::Conveyer=>(RGB::named(YELLOW1),RGB::named(BLACK)),
        };

        ctx.set(x, y, fg, bg, glyph)
    }
}

#[derive(Debug,Clone,Copy,PartialEq,Eq)] 
pub enum TileType{
    Nothing,
    Binary0,//Resource type
    Binary1,//Resource type
    Ascii,//An output 
    Conveyer,
}
impl TileType{
    pub fn get_tile_glyph(&self)->FontCharType{
        match self{
            TileType::Nothing=>to_cp437(' ') ,
            TileType::Binary0=>to_cp437('0'),
            TileType::Binary1=>to_cp437('1'),
            TileType::Ascii=>to_cp437(' '), //Defined externally,
            TileType::Conveyer=>to_cp437(' '),//Defined externally
        }
    }
}

#[derive(Debug,Clone)]
pub struct Map{
    pub width: i32,
    pub height: i32,
    pub tiles: Vec<Tile>,
}

impl Map{
    //Get tile from position
    pub fn get_tile_at_pos(&self, x: i32, y: i32)->Tile{
        //Bounds check
        if x<0 || x>=self.width || y<0 || y>=self.height {
            return Tile{
                tile_type: TileType::Nothing,
                tile_glyph: to_cp437(' '),
            };
        }
        self.tiles[(y*self.width+x) as usize]
    }
    //Set tile at position
    pub fn set_tile_at_pos(&mut self, x: i32, y: i32, tile: Tile){
        //Bounds check
        if x<0 || x>=self.width || y<0 || y>=self.height {
            return;
        }
        self.tiles[(y*self.width+x) as usize]=tile;
    }
}

impl Default for Map {
    fn default()->Map{
        let start = SystemTime::now();
        let width=80;
        let height=60;
        let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
        let mut noise = FastNoise::seeded(since_the_epoch.as_secs());
        noise.set_noise_type(NoiseType::PerlinFractal);
        noise.set_fractal_type(FractalType::FBM);
        noise.set_fractal_octaves(4);
        noise.set_fractal_gain(1.0);
        noise.set_fractal_lacunarity(2.0);
        noise.set_frequency(2.0);
                
        //Create vector of ResourceTiles of length width*height using noisemap.get_value to determine the resourcetype
        let mut tiles:Vec<Tile>=vec![];
        for y in 0..height{
            for x in 0..width{
                let value=noise.get_noise((x as f32)/60.0, (y as f32)/60.0);
                
                let Type = match value{
                    v if (v>0.2) => TileType::Binary1,
                    v if(v<0.2 && v>0.1) => TileType::Binary0,
                    v if(v < 0.1) => TileType::Nothing,
                    _ => TileType::Binary1,
                };
                let Glyph = Type.get_tile_glyph();
                tiles.push(Tile{tile_type:Type,tile_glyph:Glyph});
            }
        }
        
        Map {
            width: width,
            height: height,
            //Generate vector of ResourceTiles with length of width*height using noise crate
            tiles: tiles.clone(),

        }
    }
}