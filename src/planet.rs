use macroquad::prelude::*;

use crate::common::*;
use crate::noise::Noise;

pub struct Planet {
    pub center: Vec2,
    radius: f32,
    extents: Rect,

    pub surface: Vec<Vec2>,
}

impl Planet {
    pub fn new(center: Vec2, radius: f32, noise: &Noise) -> Self {
        let surface = Self::create_surface(500, center, radius, noise);
        Self {
            center,
            radius,
            extents: Self::calculate_extents(&surface),
            surface,
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

    fn calculate_extents(surface: &[Vec2]) -> Rect {
        let mut min_x = std::f32::INFINITY;
        let mut min_y = std::f32::INFINITY;
        let mut max_x = std::f32::NEG_INFINITY;
        let mut max_y = std::f32::NEG_INFINITY;
        for point in surface {
            if point.x < min_x {
                min_x = point.x;
            }
            if point.x > max_x {
                max_x = point.x;
            }
            if point.y < min_y {
                min_y = point.y;
            }
            if point.y > max_y {
                max_y = point.y;
            }
        }
        let width = max_x - min_x;
        let height = max_y - min_y;
        Rect::new(min_x, min_y, width, height)
    }

    pub fn is_inside(planet: &Self, point: Vec2) -> bool {
        let angle = point.angle_between(Vec2::new(0.0, 1.0));
        let mut left_min = -10.0;
        let mut right_min = 10.0;
        let mut left = 0.0;
        let mut right = 0.0;
        let mut left_seg = Vec2::new(0.0, 0.0);
        let mut right_seg = Vec2::new(0.0, 0.0);
        for segment in &planet.surface {
            let segment_angle = segment.angle_between(Vec2::new(0.0, 1.0));
            let diff = angle - segment_angle;
            if (diff < right_min && diff > 0.0) {
                right_min = diff;
                right = segment_angle;
                right_seg = *segment;
            }
            if (diff > left_min && diff < 0.0) {
                left_min = diff;
                left = segment_angle;
                left_seg = *segment;
            }
        }
        let procent = map(angle, left, right, 0.0, 1.0);
        let lerp_x = lerp(left_seg.x, right_seg.x, procent);
        let lerp_y = lerp(left_seg.y, right_seg.y, procent);
        draw_circle(lerp_x, lerp_y, 10.0, color_u8!(0, 255, 0, 255));
        let surface_point = Vec2::new(lerp_x, lerp_y);
        point.distance(planet.center) < surface_point.distance(planet.center)
    }

    pub fn as_image(planet: &Self) -> Image {
        let Planet {
            center,
            radius,
            extents,
            surface,
        } = planet;
        let width = extents.w as u16;
        let height = extents.h as u16;
        let mut planet_image = Image::gen_image_color(width, height, color_u8!(255, 0, 0, 60));
        planet_image
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
        let Rect { x, y, w, h } = self.extents;
        draw_rectangle_lines(x, y, w, h, 10.0, color_u8!(255, 255, 255, 255));
    }
}
