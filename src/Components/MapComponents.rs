use bevy::prelude::*;
use noise::Fbm;
use noise::utils::{NoiseMapBuilder, PlaneMapBuilder};

#[derive(Component)]
pub struct Tile{}

#[derive(Component,Debug,Clone)]
pub struct ResourceTile{
    pub resource_type: ResourceType,
}

#[derive(Debug,Clone)] 
pub enum ResourceType{
    Nothing,
    Binary0,
    Binary1
}

pub struct Map{
    pub width: i32,
    pub height: i32,
    pub tiles: Vec<ResourceTile>,
}

impl Map {
    pub fn new_empty(width:i32, height:i32) -> Map {
        Map {
            width: width,
            height: height,
            //Generate vector of ResourceTiles with length of width*height
            tiles: vec![ResourceTile{resource_type:ResourceType::Nothing}; (width*height) as usize],
        }
    }
    pub fn new_perlin(width:i32, height:i32)->Map{
        let fbm=Fbm::new();
        let noisemap=PlaneMapBuilder::new(&fbm)
        .set_size(width as usize, height as usize)
        .build();
        //Create vector of ResourceTiles of length width*height using noisemap.get_value to determine the resourcetype
        let mut tiles:Vec<ResourceTile>=vec![];
        for y in 0..height{
            for x in 0..width{
                let value=noisemap.get_value(x as usize, y as usize);
                let Type = match value{
                    v if (v>0.4) => ResourceType::Binary1,
                    v if(v<0.4 && v>0.2) => ResourceType::Binary0,
                    v if(v<0.2) => ResourceType::Nothing,
                    _ => ResourceType::Binary1,
                };
                tiles.push(ResourceTile{resource_type:Type});
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