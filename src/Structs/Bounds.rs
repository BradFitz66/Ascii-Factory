use bevy::prelude::Component;

#[derive(Component, Debug)]
pub struct Bounds{
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Bounds {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self { 
            x,
            y,
            width,
            height,
        }
    }
    pub fn get_center(&self) -> (f32, f32) {
        (self.x + self.width / 2.0, self.y + self.height / 2.0)
    }

    //Check if point is inside bounds
    pub fn contains(&self, x: f32, y: f32) -> bool {
        x >= self.x && x <= self.x + self.width && y >= self.y && y <= self.y + self.height
    }

    //Check if two bounds overlap
    pub fn overlaps(&self, other: &Bounds) -> bool {
        self.x < other.x + other.width && self.x + self.width > other.x && self.y < other.y + other.height && self.y + self.height > other.y
    }
}