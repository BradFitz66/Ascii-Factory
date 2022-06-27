use bevy::prelude::*;
use bracket_bevy::{FontCharType, prelude::to_cp437};
use bracket_noise::prelude::{FastNoise, FractalType, NoiseType};

use std::time::{SystemTime, UNIX_EPOCH};
#[derive(Component,Debug,Clone)]
pub struct Tile{
    pub tile_type: TileType,
    pub tile_glyph: FontCharType
}

#[derive(Debug,Clone)] 
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