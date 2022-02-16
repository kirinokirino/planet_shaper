use macroquad::prelude::*;

use crate::noise::Noise;

pub struct Planet {
    pub center: Vec2,
    radius: f32,

    pub surface: Vec<Vec2>,
}

impl Planet {
    pub fn new(center: Vec2, radius: f32, noise: &Noise) -> Self {
        Self {
            center,
            radius,
            surface: Self::create_surface(500, center, radius, noise),
        }
    }

    fn create_surface(
        surface_points: usize,
        center: Vec2,
        radius: f32,
        noise: &Noise,
    ) -> Vec<Vec2> {
        let mut surface: Vec<Vec2> = Vec::with_capacity(surface_points);
        let xoffset: i16 = rand::gen_range(100, 900);
        let yoffset: i16 = rand::gen_range(100, 900);
        for point in 0..surface_points {
            let a = point as f32 * std::f32::consts::TAU / surface_points as f32;
            let height = noise.get_point(
                (a.sin() * 80.0 + f32::from(xoffset)) as u32,
                (a.cos() * 80.0 + f32::from(yoffset)) as u32,
            ) * (radius / 2.0);
            surface.push(center + vec2((radius + height) * a.sin(), (radius + height) * a.cos()));
            //surface.push(center + vec2((radius) * a.sin(), (radius) * a.cos()));
        }

        surface
    }

    pub fn draw(&self) {
        let scale = 5.0;
        draw_triangle(
            self.center + vec2(0.0, 1.0 * scale),
            self.center + vec2(1.0 * scale, 0.0),
            self.center + vec2(-1.0 * scale, 0.0),
            color_u8!(50, 100, 200, 255),
        );

        let dotted = true;
        if dotted {
            for point in &self.surface {
                draw_circle(point.x, point.y, 5.0, color_u8!(255, 255, 255, 255));
            }
        } else {
            let mut last_point = self.surface.last().expect("No points in surface");
            for point in &self.surface {
                draw_line(
                    point.x,
                    point.y,
                    last_point.x,
                    last_point.y,
                    20.0,
                    color_u8!(0, 0, 0, 255),
                );
                last_point = point;
            }
        }
    }
}
