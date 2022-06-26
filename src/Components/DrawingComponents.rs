use bevy::prelude::*;
use bracket_bevy::{prelude::*, FontCharType};

#[derive(Component)]
//For rendering a single glyph
pub struct GlyphRenderer {
    pub glyph:FontCharType,
    pub foreground:RGB,
    pub background:RGB,
    pub layer:i32,
}
