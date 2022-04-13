use crate::common::*;
use macroquad::math::Vec2;

pub struct Player {
    pub pos: Vec2,
    pub rotation: f32,
}

impl Player {
    pub fn new(pos: Vec2) -> Self {
        Self { pos, rotation: 0.0 }
    }

    pub fn update(&mut self) {
        self.rotation = -self.pos.angle_between(Vec2::new(0.0, 1.0)) - std::f32::consts::PI;
    }

    pub fn apply_speed(&mut self, speed: Vec2) {
        self.pos += rotate(speed, self.rotation);
    }
}
