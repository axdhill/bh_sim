use macroquad::prelude::*;

#[derive(Debug)]
pub struct Object {
    pub pos: Vec2,
    pub vel: Vec2,
    pub mass: f32,
    pub color: Color,
}
