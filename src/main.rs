#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
use bevy::ecs::*;
use bevy::input::*;
use bevy::prelude::*;
use bevy::render::*;
use bevy::render::texture::Image;
use bevy::render::render_resource::FilterMode;
use bevy::time::FixedTimestep;
use bevy::window::PresentMode;
use bracket_bevy::prelude::*;
use std::{default, vec};
use bevy::render::texture::*;
use rand::prelude::*;
mod Components;
use Components::DrawingComponents::*;
use Components::GameComponents::*;
use Components::MapComponents::*;
pub const SCREEN_WIDTH: i32 = 80;
pub const SCREEN_HEIGHT: i32 = 50;
const TIMESTEP: f64 = 1.0/60.0;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum State {
    Playing,
    Update,
}

fn main() {
    let bterm =  BTermBuilder::empty()
    .with_font("terminal8x8.png", 16, 16, (8.0, 8.0))
    .with_sparse_console(0, SCREEN_WIDTH, SCREEN_HEIGHT)
    .with_background(false)
    .with_sparse_console(0, SCREEN_WIDTH, SCREEN_HEIGHT)
    .with_background(false);
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Roguelike Game".to_string(),
            width: SCREEN_WIDTH as f32 * 10.0,
            height: SCREEN_HEIGHT as f32 * 10.0,
            resizable: false,
            present_mode: PresentMode::Fifo,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins)
        .add_plugin(bterm)
        .add_system(set_filtering)
        .add_state(State::Playing)
        .add_system_set(SystemSet::on_enter(State::Playing).with_system(add_player))
        .add_system_set(SystemSet::on_enter(State::Playing).with_system(generate_map))
        .add_system_set(
            SystemSet::new()
                // This prints out "hello world" once every second
                .with_run_criteria(FixedTimestep::step(TIMESTEP))
                .with_system(tick)
                .with_system(input)
                .with_system(destroy_tile)
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
            foreground: RGB::named(WHITE),
            background: RGB::named(BLACK),
            layer:1
        })
        .insert(Transform {
            translation: Vec3::new(10., 10., 0.),
            ..Default::default()
        })
        .insert(Controllable {});
}

fn generate_map(mut commands: Commands) {
    let m = Map::new_perlin(SCREEN_WIDTH/2,SCREEN_HEIGHT/2);
    //Get each tile and create a new entity with commands.spawn
    for (mut i, mut tile) in m.tiles.iter().enumerate() {
        let x = i as i32 % m.width;
        let y = i as i32 / m.width;
        let glyph = match tile.resource_type {
            ResourceType::Nothing => ' ',
            ResourceType::Binary0 => '0',
            ResourceType::Binary1 => '1',
        };
        let tile_entity = commands
            .spawn()
            .insert(tile.clone())
            .insert(Transform {
                translation: Vec3::new(x as f32, y as f32, 0.),
                ..Default::default()
            })
            .insert(GlyphRenderer{
                glyph: to_cp437(glyph),
                foreground: RGB::named(WHITE),
                background: RGB::named(BLACK),
                layer:0
            });
        //Add the tile entity to the map entity
    }


}

fn destroy_tile(
    mut commands: Commands,
    buttons: Res<Input<MouseButton>>,
    ctx:Res<BracketContext>,
    mut query: Query<(Entity, &Transform, With<ResourceTile>)>,
) {
    let _position=ctx.get_mouse_position_for_current_layer();
    let m_pos = Vec3::new(_position.x as f32,_position.y as f32,0.);
    
    if buttons.pressed(MouseButton::Left) {
        for (entity, transform, resourcetile) in query.iter_mut() {
            if transform.translation==m_pos
            {
                commands.entity(entity).despawn();
            }
        }
    }
}
fn set_filtering(mut ev_asset: EventReader<AssetEvent<Image>>,mut assets: ResMut<Assets<Image>>,){
    for event in ev_asset.iter() {
        match event {
            AssetEvent::Created { handle } => {
                if let Some(mut texture) = assets.get_mut(handle) { 
                    texture.sampler_descriptor=ImageSampler::Descriptor(ImageSampler::nearest_descriptor());
                }
            }
            _ => {}
        }
    }
}

fn render(ctx: Res<BracketContext>, query: Query<(&GlyphRenderer, &Transform)>) {
    //Do two passes

    ctx.set_active_console(1);
    ctx.cls();
    for (renderer, transform) in query.iter() {
        if(renderer.glyph!=to_cp437(' ')){
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
    ctx.print(0, 0, format!("FPS: {}",ctx.fps));
    for (renderer, transform) in query.iter() {
        if(renderer.glyph!=to_cp437(' ')){
            ctx.set(
                transform.translation.x as i32,
                transform.translation.y as i32,
                renderer.foreground,
                renderer.background,
                renderer.glyph,
            );
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
