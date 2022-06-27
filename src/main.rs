#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
use bevy::ecs::*;
use bevy::input::*;
use bevy::prelude::*;
use bevy::render::render_resource::FilterMode;
use bevy::render::texture::Image;
use bevy::render::texture::*;
use bevy::render::*;
use bevy::time::FixedTimestep;
use bevy::window::PresentMode;
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
const LAYERS: i32 = 2;
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    StartUp,
    Tick,
}
enum WorldMode{
    UpdatingRender
}
struct WorldState{
    mode:WorldMode
}



fn main() {
    let bterm = BTermBuilder::empty()
        .with_font("terminal8x8.png", 16, 16, (8.0, 8.0))
        .with_sparse_console(0, SCREEN_WIDTH, SCREEN_HEIGHT)
        .with_background(false)
        .with_scaling_mode(TerminalScalingMode::ResizeTerminals)
        .with_sparse_console(0, SCREEN_WIDTH, SCREEN_HEIGHT)
        .with_background(false)
        .with_scaling_mode(TerminalScalingMode::ResizeTerminals);
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
        .add_plugin(bterm)
        .init_resource::<Map>()
        .add_system(set_filtering)
        .add_state(GameState::StartUp)
        .add_system_set(SystemSet::on_enter(GameState::StartUp).with_system(add_player))
        .add_system(multi_glyph_entity_test)
        .add_system(window_title_system)
        .add_system_set(
            SystemSet::new()
                // This prints out "hello world" once every second
                .with_run_criteria(FixedTimestep::step(TIMESTEP))
                .with_system(tick)
                .with_system(input)
                .with_system(destroy_tile),
        )
        .add_system(render)
        .run();
}

fn add_player(mut commands: Commands) {
    commands
        .spawn()
        .insert(Player {})
        .insert(GlyphRenderer {
            glyph: to_cp437('@'),
            foreground: RGB::named(YELLOW),
            background: RGB::named(BLACK),
            layer: 1,
        })
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
        glyphs: vec![to_cp437('@'), to_cp437('@'), to_cp437('@'), to_cp437('@'), to_cp437('@'), to_cp437('@'), 
                     to_cp437('@'), to_cp437(' '), to_cp437(' '), to_cp437(' '), to_cp437(' '), to_cp437('@'),
                     to_cp437('@'), to_cp437(' '), to_cp437(' '), to_cp437(' '), to_cp437(' '), to_cp437('@'),
                     to_cp437('@'), to_cp437(' '), to_cp437(' '), to_cp437(' '), to_cp437(' '), to_cp437('@'),
                     to_cp437('@'), to_cp437(' '), to_cp437(' '), to_cp437(' '), to_cp437(' '), to_cp437('@'),
                     to_cp437('@'), to_cp437('@'), to_cp437('@'), to_cp437('@'), to_cp437('@'), to_cp437('@')],
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
                let xy = (x+_position.x) + (y+_position.y) * map.width;
                if xy<map.tiles.len().try_into().unwrap() {
                    map.tiles[xy as usize].tile_glyph = test_multi.glyphs[(x + y * test_multi.width) as usize];
                }
            }
        }
    }
}

fn window_title_system(mut windows: ResMut<Windows>, ctx: Res<BracketContext>) {
    let window = windows.get_primary_mut().unwrap();
    //Set window title to fps and frame time
    window.set_title(format!(
        "FPS: {} | Frame Time: {:.2}ms",
        ctx.fps,
        ctx.frame_time_ms
    ));
}
fn destroy_tile(
    mut commands: Commands,
    buttons: Res<Input<MouseButton>>,
    ctx: Res<BracketContext>,
    mut query: Query<(Entity, &Transform, With<Tile>)>,
) {
    let _position = ctx.get_mouse_position_for_current_layer();
    let m_pos = Vec3::new(_position.x as f32, _position.y as f32, 0.);

    if buttons.pressed(MouseButton::Left) {
        for (entity, transform, resourcetile) in query.iter_mut() {
            if transform.translation == m_pos {
                commands.entity(entity).despawn();
            }
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
    mut query: Query<(&GlyphRenderer, &Transform)>,
    mut query2: Query<(&MultiGlyphRenderer, &Transform)>,
    mut map: ResMut<Map>
) {
    ctx.set_active_console(1);
    ctx.cls();
    for (renderer, transform) in query.iter() {
        if renderer.glyph!=to_cp437(' ') {
            ctx.set(
                transform.translation.x as i32,
                transform.translation.y as i32,
                renderer.foreground,
                renderer.background,
                renderer.glyph,
            );
        }
    }
    ctx.set_active_console(0);
    ctx.cls();
    //Iterate through map.tiles as if it were a 2D array and render each tile
    for x in 0..map.width {
        for y in 0..map.height {
            let xy = (x+y*map.width);
            let glyph = map.tiles[xy as usize].tile_glyph;
            if map.tiles[xy as usize].tile_glyph!=to_cp437(' ') {
                ctx.set(
                    x,
                    y,
                    RGB::named(WHITE),
                    RGB::named(BLACK),
                    glyph,
                );
            }
        }
    }
}

fn input(
    keyboard: Res<Input<KeyCode>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Controllable, &mut Transform)>,
) {
    for (controllable, mut transform) in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::W) {
            transform.translation.y -= 1.;
        }
        if keyboard_input.pressed(KeyCode::A) {
            transform.translation.x -= 1.;
        }
        if keyboard_input.pressed(KeyCode::S) {
            transform.translation.y += 1.;
        }
        if keyboard_input.pressed(KeyCode::D) {
            transform.translation.x += 1.;
        }
    }
}

fn tick(ctx: Res<BracketContext>, keyboard: Res<Input<KeyCode>>) {}
