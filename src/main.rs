#![allow(warnings)]

use bevy::ecs::*;
use bevy::input::*;
use bevy::prelude::*;
use bevy::render::*;
use bevy::render::texture::*;
use bevy::tasks::ComputeTaskPool;
use bevy::time::*;
use bevy::window::*;
use iyes_loopless::prelude::*;

use bracket_bevy::prelude::*;
use rand::prelude::*;
use std::time::Duration;
use std::{default, vec};

mod Components;
use Components::DrawingComponents::*;
use Components::GameComponents::*;
use Components::MapComponents::*;
mod Structs;
use Structs::Bounds::Bounds;



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
            title: "Ascii Factory".to_string(),
            width: SCREEN_WIDTH as f32 * 8.0,
            height: SCREEN_HEIGHT as f32 * 8.0,
            resizable: true,
            present_mode: PresentMode::Fifo,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(ImageSettings{
            default_sampler: ImageSampler::nearest_descriptor(),
        })
        .add_plugins(DefaultPlugins)
        .add_system(set_filtering)
        .add_plugin(BTermBuilder::empty()
            .with_font("terminal8x8.png", 16, 16, (8.0, 8.0))
            .with_sparse_console(0, SCREEN_WIDTH, SCREEN_HEIGHT)
            .with_background(false)
            .with_scaling_mode(TerminalScalingMode::ResizeTerminals)
            .with_sparse_console(0, SCREEN_WIDTH, SCREEN_HEIGHT)
            .with_background(false)
            .with_scaling_mode(TerminalScalingMode::Stretch)
        )
        .init_resource::<Map>()
        .add_state(GameState::StartUp)
        .add_system_set(SystemSet::on_enter(GameState::StartUp).with_system(add_player))
        .add_system(window_title_system)
        .add_system_set(
            SystemSet::new()
            .with_run_criteria(FixedTimestep::step(0.25))
            .with_system(input)        
        )
        .add_system_set(
            SystemSet::new()
                // This prints out "hello world" once every second
                .with_run_criteria(FixedTimestep::step(TIMESTEP))
                .with_system(render_map.before(render_entities::<Player>))
                .with_system(handle_conveyer_placement)
                .with_system(render_entities::<Player>.after(render_map))
                .with_system(render_entities::<Miner>.after(render_map))
                .with_system(render_entities::<Processor>.after(render_map))
                .with_system(render_entities::<Tile>.after(render_map))        
        )
        .add_system_set(
            SystemSet::new()
            .with_run_criteria(FixedTimestep::step(0.5))
            .with_system(update_mover_conveyer)
        )
        .add_system_set(
            SystemSet::new()
            .with_run_criteria(FixedTimestep::step(5.0))
            .with_system(update_miner)
        )
        .add_system(place_miner)
        .add_system(place_processor)        
        .add_system(update_processor_process)
        .run();
}

fn place_miner(
    mut commands: Commands,
    ctx: Res<BracketContext>,
    buttons: Res<Input<MouseButton>>,
    mut query: Query<(&Miner, &mut Bounds)>,
){
    if buttons.just_pressed(MouseButton::Left) {
        let m_pos=ctx.get_mouse_position_for_current_layer();
        let m_pos_vec = Vec3::new(m_pos.x as f32, m_pos.y as f32, 0.0);
        let bounds=Bounds{
            x: m_pos_vec.x,
            y: m_pos_vec.y,
            width: 6.0,
            height: 6.0,
        };
        let centre=bounds.get_center();


        for (miner, mut other_bounds) in &mut query.iter() {
            if bounds.overlaps(&other_bounds) {
                return;
            }
        }
        let centre=bounds.get_center();

        commands.spawn()
        .insert(Miner{radius: 4, ..Default::default()})
        .insert(Transform{
            translation: Vec3::new(m_pos_vec.x-(bounds.width/2.0), m_pos_vec.y-(bounds.height/2.0), 0.0),
            ..Default::default()
        })    
        .insert(GenericTimer{timer:Timer::new(Duration::from_secs(1),true)})
        //Insert bounds
        .insert(bounds);
    }
}

//Update processor collisions (adding mined bits to the processor)
fn update_processor_collision(
    mut commands: Commands,
    ctx: Res<BracketContext>,
    buttons: Res<Input<MouseButton>>,
    mut query_proc: Query<(&Processor, &mut Bounds)>,
    mut query_movable: Query<(Entity,&Movable,&Transform,&Tile)>,
    time:Res<Time>
){
    //Loop through all processors and check if any movables are within the bounds of the processor
    for (processor, mut bounds) in &mut query_proc.iter() {
        for (entity, movable,transform,tile) in &mut query_movable.iter() {
            if bounds.contains(transform.translation.x,transform.translation.y) {
                commands.entity(entity).despawn();
            }
        }
    }
}

fn update_processor_process(
    mut commands: Commands,
    ctx: Res<BracketContext>,
    buttons: Res<Input<MouseButton>>,
    mut query_proc: Query<(&mut Processor, &mut Bounds, &mut GenericTimer)>,
    mut query_movable: Query<(Entity,&Movable, &Transform,&Tile)>,
    time:Res<Time>
){
    //Loop through all processors, 
    for (mut processor, mut bounds, mut clock) in query_proc.iter_mut() {
        //Check if any movable is within the bounds of the processor
        for (entity, movable,transform,tile) in query_movable.iter_mut() {
            if bounds.contains(transform.translation.x,transform.translation.y)&&processor.count>0 {
                //Add the movable to the processor
                processor.count-=1; 
                //Set or reset Nth bit of processor.bits with bitwise operation based on if movable is Binary0 or Binary1
                match tile.tile_type {
                    TileType::Binary0 => {processor.bits &= !(1 << processor.count-1)},
                    TileType::Binary1 => {processor.bits |=  (1 << processor.count-1)}
                    TileType::Nothing | TileType::Ascii | TileType::Conveyer => {},
                }
                println!("bits: {:8b}", processor.bits);
                //Remove the movabler from the wold
                commands.entity(entity).despawn();
            }
            else if(processor.count==0){
                clock.timer.tick(time.delta());
                if(clock.timer.finished()){
                    clock.timer.reset();
                    processor.count=8;
                    let output_char:char = char::from_u32(processor.bits).unwrap_or(' ');
                    println!("{}",output_char);
                    processor.bits=0;
                    if(output_char!=' '){
                        commands.spawn()
                        .insert(Movable{})
                        .insert(Transform{
                            translation: Vec3::new(bounds.x+bounds.width, bounds.y, 0.0),
                            ..Default::default()
                        })
                        .insert(Tile{
                            tile_type: TileType::Ascii,
                            tile_glyph: to_cp437(output_char)
                        });
                    }
                }
            }
        }
    }
}

fn place_processor(
    mut commands: Commands,
    ctx: Res<BracketContext>,
    buttons: Res<Input<MouseButton>>,
    mut query: Query<(&Miner, &mut Bounds)>,
    time:Res<Time>
){
    if buttons.just_pressed(MouseButton::Middle) {
        let m_pos=ctx.get_mouse_position_for_current_layer();
        let m_pos_vec = Vec3::new(m_pos.x as f32, m_pos.y as f32, 0.0);
        let bounds=Bounds{
            x: m_pos_vec.x-4.0,
            y: m_pos_vec.y,
            width: 8.0,
            height: 6.0,
        };
        let centre=bounds.get_center();
        //Make sure there's no miners currently at m_pos 

        //Check if placing would overlap any other processor
        for (miner, mut other_bounds) in &mut query.iter() {
            if(bounds.overlaps(&other_bounds)){
                return;
            }
        }
        
        commands.spawn().insert(Processor{..Default::default()}).insert(Transform{
            translation: Vec3::new(m_pos_vec.x-(bounds.width/2.0), m_pos_vec.y-(bounds.height/2.0), 0.0),
            ..Default::default()
        })
        .insert(bounds)
        .insert(GenericTimer{timer:Timer::new(Duration::from_secs(1),false)});
    }
}

fn update_miner(
    mut commands: Commands,
    ctx: Res<BracketContext>,
    buttons: Res<Input<MouseButton>>,
    mut query: Query<(&Miner, &mut Transform)>,
    map: Res<Map>,
    t: Res<Time>
){
    //Get all map tiles underneath each miner within it's radius
    for (miner, mut transform) in query.iter_mut() {
        let mut tiles: Vec<Tile> = Vec::new();
        let miner_radius_half=(miner.radius as f32)/2.0;
        for x in ((transform.translation.x+3.0) - miner_radius_half) as i32..=((transform.translation.x+3.0) + miner_radius_half) as i32 {
            for y in ((transform.translation.y+3.0) - miner_radius_half) as i32..=((transform.translation.y+3.0) + miner_radius_half) as i32 {
                if map.get_tile_at_pos(x, y).tile_type==TileType::Binary0 || map.get_tile_at_pos(x, y).tile_type==TileType::Binary1 {
                    tiles.push(map.get_tile_at_pos(x, y));
                }
            }
        }
        //Spawn a random entity based on a random tile from tiles vector
        if tiles.len()>0{
            let mut rng = rand::thread_rng();
            let random_tile = tiles[rng.gen_range(0..tiles.len())];
            commands.spawn()
            .insert(Movable{})
            .insert(Tile{
                tile_type: TileType::Ascii,
                tile_glyph: random_tile.tile_type.get_tile_glyph(),
            })
            .insert(Transform{
                translation: Vec3::new(transform.translation.x + 6.,transform.translation.y+3.0,0.),
                ..Default::default()
            });
        }
    }
}



fn update_mover_conveyer(
    mut commands: Commands,
    mut entities:Query<(&Movable, &mut Transform)>,
    mut map: ResMut<Map>,
    t:Res<Time>
)
{
    let possible_directions=[
        to_cp437('►'),
        to_cp437('▼'),
        to_cp437('◄'),
        to_cp437('▲'),
    ];
    entities.for_each_mut(|(movable,mut transform,)|{
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
            TileType::Binary0 | TileType::Binary1 | TileType::Ascii |TileType::Conveyer => {},
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
        .insert(Controllable {})
        .insert(GenericTimer{timer:Timer::new(Duration::from_millis(500),false)});
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
    // for event in ev_asset.iter() {
    //     match event {
    //         AssetEvent::Created { handle } => {
    //             if let Some(mut texture) = assets.get_mut(handle) {
    //                 texture.sampler_descriptor = ImageSampler::Descriptor(ImageSampler::nearest_descriptor());
    //             }
    //         },
    //         AssetEvent::Modified { handle } => {
    //             if let Some(mut texture) = assets.get_mut(handle) {
    //                 texture.sampler_descriptor = ImageSampler::Descriptor(ImageSampler::nearest_descriptor());
    //             }
    //         },
    //         _ => {}
    //     }
    // }
}

fn render_map(ctx:Res<BracketContext>, map:Res<Map>){
    ctx.cls();
    map.render(ctx.as_ref().clone());
}

fn render_entities<C: Component + GlyphRenderer>(query_glyphs:Query<(&C,&Transform)>,ctx:Res<BracketContext>){
    for (renderer, transform) in query_glyphs.iter() {  
        renderer.render_glyph(transform.translation.x as i32, transform.translation.y as i32, ctx.as_ref().clone());
    }
}

fn input(
    keyboard: Res<Input<KeyCode>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut ctx:ResMut<BracketContext>,
    mut query: Query<(&Controllable, &mut Transform)>,
    mut map: ResMut<Map>,
    mut commands: Commands
) {
    
    for (controllable, mut transform) in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::W) {
            transform.translation.y -= 1.0 ;
        }
        if keyboard_input.pressed(KeyCode::A) {
            transform.translation.x -= 1.0 ;
        }
        if keyboard_input.pressed(KeyCode::S) {
            transform.translation.y += 1.0 ;
        }
        if keyboard_input.pressed(KeyCode::D) {
            transform.translation.x += 1.0 ;
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
    if(keyboard.pressed(KeyCode::Key0)){
        //Spawn movable tile at mouse pos
        let m_pos=ctx.get_mouse_position_for_current_layer();
        let m_pos_vec=Vec3::new(m_pos.x as f32,m_pos.y as f32,0.);
        commands.spawn()
            .insert(Movable{})
            .insert(Transform{
                translation:m_pos_vec,
                ..Default::default()
            })
            .insert(
                Tile{
                    tile_type:TileType::Binary0,
                    tile_glyph:to_cp437('0'),
                }
            );
    }
    if(keyboard.pressed(KeyCode::Key1)){
        //Spawn movable tile at mouse pos
        let m_pos=ctx.get_mouse_position_for_current_layer();
        let m_pos_vec=Vec3::new(m_pos.x as f32,m_pos.y as f32,0.);
        commands.spawn()
            .insert(Movable{})
            .insert(Transform{
                translation:m_pos_vec,
                ..Default::default()
            })
            .insert(
                Tile{
                    tile_type:TileType::Binary1,
                    tile_glyph:to_cp437('1'),
                }
            );
    }
}

