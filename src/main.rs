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
const LAYERS: i32 = 2;
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    StartUp,
    Tick,
}



fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Roguelike Game".to_string(),
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
        //.add_system(multi_glyph_entity_test)
        .add_system(window_title_system)
        .add_system_set(
            SystemSet::new()
                // This prints out "hello world" once every second
                .with_run_criteria(FixedTimestep::step(TIMESTEP))
                .with_system(handle_conveyer_placement)
                .with_system(test_add_movable)        
        )
        .add_system_set(
            SystemSet::new()
                // This prints out "hello world" once every second
                .with_run_criteria(FixedTimestep::step(TIMESTEP_INPUT))
                .with_system(update_mover_conveyer)
                .with_system(input)
        )
        .add_system(render)
        .run();
}

fn test_add_movable(
    mut commands: Commands,
    buttons: Res<Input<MouseButton>>,
    ctx: Res<BracketContext>,
    entities:Query<(Entity, &Transform)>,
    mut map: ResMut<Map>)
{
    //Spawn movable at mouse position
    let m_pos=ctx.get_mouse_position_for_current_layer();
    //Check if any entities already exist at mouse position using the query
    let mut entity_at_pos = false;
    for (entity,transform) in entities.iter() {
        if transform.translation.x == m_pos.x as f32 && transform.translation.y == m_pos.y as f32 {
            entity_at_pos = true;
        }
    }

    if !entity_at_pos && buttons.pressed(MouseButton::Middle) {
        commands.spawn()
        .insert(Movable{})
        .insert(Transform{
            translation:Vec3::new(m_pos.x as f32,m_pos.y as f32,0.0),
            ..Default::default()
        })
        .insert(Tile{
            tile_type:TileType::Binary0,
            tile_glyph:to_cp437('0')
        });
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
        .insert(Controllable {});
}

fn multi_glyph_entity_test(    
    mut commands: Commands,
    buttons: Res<Input<MouseButton>>,
    ctx: Res<BracketContext>,
    mut query: Query<(Entity, &Transform, With<Tile>)>,
    mut map: ResMut<Map>
){
    let _position = ctx.get_mouse_position_for_current_layer();
    let m_pos = Vec3::new(_position.x as f32, _position.y as f32, 0.);

    let test_multi= MultiGlyphRenderer {
        glyphs: vec![to_cp437('#'), to_cp437('#'), to_cp437('#'), to_cp437('#'), to_cp437('#'), to_cp437('#'), 
                     to_cp437('#'), to_cp437(' '), to_cp437(' '), to_cp437(' '), to_cp437(' '), to_cp437('#'),
                     to_cp437('#'), to_cp437(' '), to_cp437(' '), to_cp437(' '), to_cp437(' '), to_cp437('#'),
                     to_cp437('#'), to_cp437(' '), to_cp437(' '), to_cp437(' '), to_cp437(' '), to_cp437('#'),
                     to_cp437('#'), to_cp437(' '), to_cp437(' '), to_cp437(' '), to_cp437(' '), to_cp437('#'),
                     to_cp437('#'), to_cp437('#'), to_cp437('#'), to_cp437('#'), to_cp437('#'), to_cp437('#')],
        foreground: RGB::named(YELLOW),
        background: RGB::named(BLACK),
        width: 6,
        height: 6,
        layer: 1,
    };

    if buttons.pressed(MouseButton::Left) {
        //Insert the glyphs from the multi glyph renderer into the map's tile vector at the correct x,y position
        for x in 0..test_multi.width {
            for y in 0..test_multi.height {
                //Turn x, y into 1D coordinates for map.tiles
                let xy = (x+(_position.x-test_multi.width/2)) + (y+(_position.y-test_multi.height/2)) * map.width;
                let glyph=test_multi.glyphs[(x + y * test_multi.width) as usize];
                if xy<map.tiles.len() as i32 && glyph!=to_cp437(' ')  {                    
                    map.tiles[xy as usize].tile_type = TileType::Ascii;
                    map.tiles[xy as usize].tile_glyph = glyph;
                }
            }
        }
    }
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

    ctx.set_active_console(0);
    ctx.cls();
    //Iterate through map.tiles as if it were a 2D array and render each tile
    for x in 0..map.width {
        for y in 0..map.height {
            let xy = (x+y*map.width);
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

