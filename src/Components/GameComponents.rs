use bevy::prelude::*;
use bracket_bevy::prelude::*;

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