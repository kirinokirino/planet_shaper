use macroquad::math::Vec2;

pub struct Player {
    pub pos: Vec2,
}

impl Player {
    pub fn new(pos: Vec2) -> Self {
        Self { pos }
    }
}
