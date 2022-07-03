use bevy::{prelude::*};
use bracket_bevy::{prelude::*, FontCharType};
use std::time::Instant;
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
pub struct Processor {pub bits: u32, pub glyphs: Vec<FontCharType>, pub count:i32}
impl GlyphRenderer for Processor {
    fn render_glyph(&self, x_pos: i32, y_pos: i32, ctx: &BracketContext) {
        for x in 0..8 {
            for y in 0..6 {
                let glyph = self.glyphs[(x + y * 8) as usize];
                if glyph!=to_cp437(' ') {
                    ctx.set(x+x_pos,  y+y_pos, RGB::named(YELLOW), RGB::named(BLACK), glyph);
                }
            }
        }
    }
}

impl Default for Processor{
    fn default()->Processor{
        Processor{
            bits: 0,
            glyphs: vec![
                        to_cp437('#'), to_cp437('#'), to_cp437('#'), to_cp437('#'), to_cp437('#'), to_cp437('#'), to_cp437('#'), to_cp437('#'), 
                        to_cp437('#'), to_cp437(' '), to_cp437(' '), to_cp437(' '), to_cp437(' '), to_cp437(' '), to_cp437(' '), to_cp437('#'),
                        to_cp437('#'), to_cp437(' '), to_cp437(' '), to_cp437(' '), to_cp437(' '), to_cp437(' '), to_cp437(' '), to_cp437('#'),
                        to_cp437('#'), to_cp437(' '), to_cp437(' '), to_cp437(' '), to_cp437(' '), to_cp437(' '), to_cp437(' '), to_cp437('#'),
                        to_cp437('#'), to_cp437(' '), to_cp437(' '), to_cp437(' '), to_cp437(' '), to_cp437(' '), to_cp437(' '), to_cp437('#'),
                        to_cp437('#'), to_cp437('#'), to_cp437('#'), to_cp437('#'), to_cp437('#'), to_cp437('#'), to_cp437('#'), to_cp437('#')
                        ],
            count:8
        }
    }
}


#[derive(Component)]
pub struct GenericTimer {
    pub timer: Timer,
}
#[derive(Component, Debug)]
pub struct Miner {pub radius: i32, pub glyphs: Vec<FontCharType>,}
impl GlyphRenderer for Miner {
    fn render_glyph(&self, x_pos: i32, y_pos: i32, ctx: &BracketContext) {
        for x in 0..6 {
            for y in 0..6 {
                let glyph = self.glyphs[(x + y * 6) as usize];
                if glyph!=to_cp437(' ') {
                    ctx.set(x+x_pos, y+y_pos , RGB::named(YELLOW), RGB::named(BLACK), glyph);
                }
            }
        }
    }
}

impl Default for Miner{
    fn default()->Miner{
        Miner{
            radius:6,
            glyphs: vec![to_cp437('#'), to_cp437('#'), to_cp437('#'), to_cp437('#'), to_cp437('#'), to_cp437('#'), 
                        to_cp437('#'), to_cp437(' '), to_cp437(' '), to_cp437(' '), to_cp437(' '), to_cp437('#'),
                        to_cp437('#'), to_cp437(' '), to_cp437(' '), to_cp437(' '), to_cp437(' '), to_cp437('#'),
                        to_cp437('#'), to_cp437(' '), to_cp437(' '), to_cp437(' '), to_cp437(' '), to_cp437('#'),
                        to_cp437('#'), to_cp437(' '), to_cp437(' '), to_cp437(' '), to_cp437(' '), to_cp437('#'),
                        to_cp437('#'), to_cp437('#'), to_cp437('#'), to_cp437('#'), to_cp437('#'), to_cp437('#')],}
    }
}