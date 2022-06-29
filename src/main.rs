#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
use bevy::ecs::*;
use bevy::input::*;
use bevy::prelude::*;
use bevy::render::*;
use bevy::render::texture::*;
use bevy::tasks::ComputeTaskPool;
use bevy::time::*;
use bevy::window::*;

use bracket_bevy::prelude::*;
use rand::prelude::*;
use std::{default, vec};

mod Components;
use Components::DrawingComponents::*;
use Components::GameComponents::*;
use Components::MapComponents::*;

pub const SCREEN_WIDTH: i32 = 80;
pub const SCREEN_HEIGHT: i32 = 60;
const TIMESTEP: f64 = 1.0 / 60.0;
const TIMESTEP_INPUT: f64 = 12.0 / 60.0;
const TIMESTEP_MACHINE: f64 = 120.0 / 60.0;
const LAYERS: i32 = 2;
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    StartUp,
    Tick,
}



fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Ascii Factory".to_string(),
            width: SCREEN_WIDTH as f32 * 8.0,
            height: SCREEN_HEIGHT as f32 * 8.0,
            resizable: false,
            present_mode: PresentMode::Fifo,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins)
        .add_system(set_filtering)
        .add_plugin(BTermBuilder::empty()
            .with_font("terminal8x8.png", 16, 16, (8.0, 8.0))
            .with_sparse_console(0, SCREEN_WIDTH, SCREEN_HEIGHT)
            .with_background(false)
            .with_scaling_mode(TerminalScalingMode::ResizeTerminals)
            .with_sparse_console(0, SCREEN_WIDTH, SCREEN_HEIGHT)
            .with_background(false)
            .with_scaling_mode(TerminalScalingMode::ResizeTerminals)
        )
        .init_resource::<Map>()
        .add_state(GameState::StartUp)
        .add_system_set(SystemSet::on_enter(GameState::StartUp).with_system(add_player))
        .add_system(window_title_system)
        .add_system_set(
            SystemSet::new()
                // This prints out "hello world" once every second
                .with_run_criteria(FixedTimestep::step(TIMESTEP))
                .with_system(handle_conveyer_placement)
                .with_system(place_miner)
        )
        .add_system_set(
            SystemSet::new()
                // This prints out "hello world" once every second
                .with_run_criteria(FixedTimestep::step(TIMESTEP_INPUT))
                .with_system(update_mover_conveyer)
                .with_system(input)
        )
        .add_system_set(
            SystemSet::new()
                // This prints out "hello world" once every second
                .with_run_criteria(FixedTimestep::step(TIMESTEP_MACHINE))
                .with_system(update_miner)
        )
        .add_system(render)
        .run();
}

fn place_miner(
    mut commands: Commands,
    ctx: Res<BracketContext>,
    buttons: Res<Input<MouseButton>>,
    mut query: Query<(&Miner, &mut Transform)>,
){
    if buttons.pressed(MouseButton::Left) {
        let m_pos=ctx.get_mouse_position_for_current_layer();
        //Make sure there's no miners currently at m_pos 


        for (miner, mut transform) in &mut query.iter() {
            if transform.translation == Vec3::new(m_pos.x as f32, m_pos.y as f32, 0.0) {
                return;
            }
        }
        commands.spawn().insert(Miner{radius: 3, ..Default::default()}).insert(Transform{
            translation: Vec3::new(m_pos.x as f32, m_pos.y as f32,0.),
            ..Default::default()
        });    
    }
}

fn update_miner(
    mut commands: Commands,
    ctx: Res<BracketContext>,
    buttons: Res<Input<MouseButton>>,
    mut query: Query<(&Miner, &mut Transform)>,
    map: Res<Map>,
){
    //Get all map tiles underneath each miner within it's radius
    for (miner, mut transform) in &mut query.iter() {
        let mut tiles: Vec<Tile> = Vec::new();
        for x in (transform.translation.x - miner.radius as f32) as i32..=(transform.translation.x + miner.radius as f32) as i32 {
            for y in (transform.translation.y - miner.radius as f32) as i32..=(transform.translation.y + miner.radius as f32) as i32 {
                if map.get_tile_at_pos(x, y).tile_type==TileType::Binary0 || map.get_tile_at_pos(x, y).tile_type==TileType::Binary1 {
                    tiles.push(map.get_tile_at_pos(x, y));
                }
            }
        }
        //Spawn a random entity based on a random tile from tiles vector
        if tiles.len()>0 {
            println!("Creating new entity");
            let mut rng = rand::thread_rng();
            let random_tile = tiles[rng.gen_range(0..tiles.len())];
            commands.spawn()
            .insert(Movable{})
            .insert(Tile{
                tile_type: random_tile.tile_type,
                tile_glyph: random_tile.tile_glyph,
            })
            .insert(Transform{
                translation: Vec3::new(transform.translation.x + 6.,transform.translation.y,0.),
                ..Default::default()
            });
        }
    }
}

fn update_mover_conveyer(
    mut commands: Commands,
    mut entities:Query<(&Movable, &mut Transform)>,
    mut map: ResMut<Map>)
{
    let possible_directions=[
        to_cp437('►'),
        to_cp437('▼'),
        to_cp437('◄'),
        to_cp437('▲'),
    ];
    entities.for_each_mut(|(movable,mut transform)|{
        let on_tile=map.get_tile_at_pos(transform.translation.x as i32,transform.translation.y as i32);
        if on_tile.tile_type == TileType::Conveyer {
            //Get index of the conveyers direction inside the possible_directions array
            let glyph_index = possible_directions.iter().position(|&x| x == on_tile.tile_glyph);
            //Do a match statement for glyph index of the conveyer
            match glyph_index {
                Some(0) => {
                    transform.translation.x += 1.0;
                },
                Some(1) => {
                    transform.translation.y += 1.0;
                },
                Some(2) => {
                    transform.translation.x -= 1.0;
                },
                Some(3) => {
                    transform.translation.y -= 1.0;
                },
                _ => {
                    //Do nothing
                }
            }

        }
    });
}

fn handle_conveyer_placement(
    mut commands: Commands,
    buttons: Res<Input<MouseButton>>,
    ctx: Res<BracketContext>,
    keyboard_input: Res<Input<KeyCode>>,
    mut map: ResMut<Map>)
{
    //Create vector for all conveyer belt glyphs
    let m_pos=ctx.get_mouse_position_for_current_layer();
    let tile=map.get_tile_at_pos(m_pos.x,m_pos.y);

    if buttons.pressed(MouseButton::Right) {
        //Get tile at mouse position from map.tiles
        match tile.tile_type{
            TileType::Nothing => {
                //Set conveyer tile at mouse position in map.tiles
                map.set_tile_at_pos(m_pos.x, m_pos.y, Tile{
                    tile_type:TileType::Conveyer,
                    tile_glyph: to_cp437('►'),
                });
            },
            TileType::Binary0 | TileType::Binary1 => {},
            TileType::Ascii => {},
            TileType::Conveyer => {},
        }
    }
}

fn add_player(mut commands: Commands) {
    commands
        .spawn()
        .insert(Player {})
        .insert(Transform {
            translation: Vec3::new(10., 10., 1.),
            ..Default::default()
        })
        .insert(Movable{})
        .insert(Controllable {});
}

fn window_title_system(mut windows: ResMut<Windows>, ctx: Res<BracketContext>) {
    let window = windows.get_primary_mut();

    //Window is destroyed before this system stops. Only change window title if a window exists.
    match window{
        Some(window) => {
            //Set window title to fps and frame time
            window.set_title(format!(
                "FPS: {} | Frame Time: {:.2}ms",
                ctx.fps,
                ctx.frame_time_ms
            ));
        }
        None => {
        }
    }
}

fn set_filtering(mut ev_asset: EventReader<AssetEvent<Image>>, mut assets: ResMut<Assets<Image>>) {
    for event in ev_asset.iter() {
        match event {
            AssetEvent::Created { handle } => {
                if let Some(mut texture) = assets.get_mut(handle) {
                    texture.sampler_descriptor = ImageSampler::Descriptor(ImageSampler::nearest_descriptor());
                }
            },
            AssetEvent::Modified { handle } => {
                if let Some(mut texture) = assets.get_mut(handle) {
                    texture.sampler_descriptor = ImageSampler::Descriptor(ImageSampler::nearest_descriptor());
                }
            },
            _ => {}
        }
    }
}
//Sstem to render all objects with GlyphRenderers and MultiGlyphRenderers
fn render(
    mut ctx: Res<BracketContext>,
    mut query_player: Query<(&Player, &Transform)>,
    mut query_movable: Query<(&Movable, &Transform,&Tile)>,
    mut query_render: Query<(&Miner, &Transform)>,
    mut map: ResMut<Map>
) {
    ctx.set_active_console(1);
    ctx.cls();
    for (player, transform) in query_player.iter() {
        player.render_glyph(transform.translation.x as i32, transform.translation.y as i32, ctx.as_ref().clone());
    }
    for (movable, transform,tile) in query_movable.iter() {
        tile.render_glyph(transform.translation.x as i32, transform.translation.y as i32, ctx.as_ref().clone());
    }
    for(miner,trannsform) in query_render.iter_mut(){
        miner.render_glyph(trannsform.translation.x as i32, trannsform.translation.y as i32, ctx.as_ref().clone());
    }
    

    ctx.set_active_console(0);
    ctx.cls();
    //Iterate through map.tiles as if it were a 2D array and render each tile
    for x in 0..map.width {
        for y in 0..map.height {
            let xy = x+y*map.width;
            let glyph = map.tiles[xy as usize].tile_glyph;
            if map.tiles[xy as usize].tile_glyph!=to_cp437(' ') {
                map.tiles[xy as usize].render_glyph(x, y, ctx.as_ref().clone())
            }
        }
    }
}

fn input(
    keyboard: Res<Input<KeyCode>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut ctx:ResMut<BracketContext>,
    mut query: Query<(&Controllable, &mut Transform)>,
    mut map: ResMut<Map>
) {
    for (controllable, mut transform) in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::W) {
            transform.translation.y -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::A) {
            transform.translation.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::S) {
            transform.translation.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::D) {
            transform.translation.x += 1.0;
        }
    }
    let mut conveyer_glyphs = vec![
        to_cp437('►'),
        to_cp437('▼'),
        to_cp437('◄'),
        to_cp437('▲'),
    ];
    //If R key on keyboard is pressed, rotate the conveyer belt underneath the mouse (switch to next conveyer glyph) (if there is one)
    if keyboard.pressed(KeyCode::R) {
        let _position = ctx.get_mouse_position_for_current_layer();
        let m_pos = Vec3::new(_position.x as f32, _position.y as f32, 0.);
        //Check if tile under m_pos is conveyer. If so, switch to the next glyph in the vector based on which glyph is currently being used
        let mut tile=map.get_tile_at_pos(_position.x, _position.y);
        //Get which glyph is currently being used in the vector
        let glyph_index = conveyer_glyphs.iter().position(|&x| x == tile.tile_glyph);
        //If the glyph is not found in the vector, do nothing. If it is found, switch to the next glyph in the vector.
        if let Some(glyph_index) = glyph_index {
            if glyph_index+1<conveyer_glyphs.len() {
                tile.tile_glyph = conveyer_glyphs[glyph_index+1];
                map.set_tile_at_pos(_position.x, _position.y, tile.clone());
            }
            else{
                tile.tile_glyph=conveyer_glyphs[0];
                map.set_tile_at_pos(_position.x, _position.y, tile.clone());
            }
        }

    }
}

