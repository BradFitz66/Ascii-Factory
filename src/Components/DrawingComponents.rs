use bevy::prelude::*;
use bracket_bevy::{prelude::*, FontCharType};

#[derive(Component)]
//For rendering a single glyph
pub struct GlyphRenderer {
    pub glyph: FontCharType,
    pub foreground: RGB,
    pub background: RGB,
    pub layer: i32,
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
