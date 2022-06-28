use bevy::prelude::*;
use bracket_bevy::{prelude::*, FontCharType};

use super::MapComponents::TileType;

//For rendering a single glyph
pub trait GlyphRenderer {
    fn render_glyph(&self, x: i32, y: i32, ctx:&BracketContext);
}


#[derive(Component)]
//For rendering multiple glyphs at once 
pub struct MultiGlyphRenderer {
    pub glyphs: Vec<FontCharType>,
    pub width:i32, 
    pub height: i32,
    pub foreground: RGB,
    pub background: RGB,
    pub layer: i32,
}
