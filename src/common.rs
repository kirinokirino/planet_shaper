use macroquad::prelude::*;

#[must_use]
pub fn lerp(from: f32, to: f32, p: f32) -> f32 {
    from.mul_add(1.0 - p, to * p)
}

#[must_use]
pub fn map(value: f32, start1: f32, stop1: f32, start2: f32, stop2: f32) -> f32 {
    ((value - start1) / (stop1 - start1)).mul_add(stop2 - start2, start2)
}

#[must_use]
pub fn norm(value: f32, start: f32, stop: f32) -> f32 {
    map(value, start, stop, 0.0, 1.0)
}

pub fn draw_vector(start: Vec2, vector: Vec2) {
    let end = Vec2::new(start.x + vector.x, start.y + vector.y);
    const SQUARE_SIZE: f32 = 30.0;
    draw_line(
        start.x,
        start.y,
        end.x,
        end.y,
        1.0,
        color_u8!(255, 100, 100, 255),
    );
    draw_rectangle_lines(
        end.x - (SQUARE_SIZE / 2.0),
        end.y - (SQUARE_SIZE / 2.0),
        SQUARE_SIZE,
        SQUARE_SIZE,
        2.0,
        color_u8!(100, 255, 100, 100),
    );
}

#[must_use]
pub fn rotate(vector: Vec2, radians: f32) -> Vec2 {
    let rotation_matrix = Mat3::from_rotation_z(radians);
    rotation_matrix.transform_vector2(vector)
}
