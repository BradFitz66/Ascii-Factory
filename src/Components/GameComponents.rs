use bevy::prelude::*;
use bracket_bevy::{prelude::*, FontCharType};

use super::DrawingComponents::GlyphRenderer;


//Something that can be moved (i.e. on a conveyer belt)
#[derive(Component)]
pub struct Movable {}

#[derive(Component, Debug)]
pub struct Controllable {}

#[derive(Component, Debug)]
pub struct Player {}

//Implement glyph renderer for player
impl GlyphRenderer for Player {
    fn render_glyph(&self, x: i32, y: i32, ctx: &BracketContext) {
        ctx.set(x, y, RGB::named(YELLOW), RGB::named(BLACK), to_cp437('@'));
    }
}

#[derive(Component, Debug)]
pub struct Miner {pub radius: i32, pub glyphs: Vec<FontCharType>,}
impl GlyphRenderer for Miner {
    fn render_glyph(&self, x_pos: i32, y_pos: i32, ctx: &BracketContext) {
        for x in 0..6 {
            for y in 0..6 {
                let glyph = self.glyphs[(x + y * 6) as usize];
                ctx.set(x_pos + x, y_pos + y, RGB::named(YELLOW), RGB::named(BLACK), glyph);
            }
        }
    }
}
impl Default for Miner{
    fn default()->Miner{
        Miner{
            radius:1,
            glyphs: vec![to_cp437('#'), to_cp437('#'), to_cp437('#'), to_cp437('#'), to_cp437('#'), to_cp437('#'), 
                        to_cp437('#'), to_cp437(' '), to_cp437(' '), to_cp437(' '), to_cp437(' '), to_cp437('#'),
                        to_cp437('#'), to_cp437(' '), to_cp437(' '), to_cp437(' '), to_cp437(' '), to_cp437('#'),
                        to_cp437('#'), to_cp437(' '), to_cp437(' '), to_cp437(' '), to_cp437(' '), to_cp437('#'),
                        to_cp437('#'), to_cp437(' '), to_cp437(' '), to_cp437(' '), to_cp437(' '), to_cp437('#'),
                        to_cp437('#'), to_cp437('#'), to_cp437('#'), to_cp437('#'), to_cp437('#'), to_cp437('#')],}
    }
}