use macroquad::prelude::*;

use crate::common::*;
use crate::noise::Noise;
use crate::world::NOISE_SIZE;

const MULTIPLE_360: u8 = 2;

pub struct Planet {
    pub center: Vec2,
    radius: f32,
    max_radius: f32,
    pub extents: Rect,

    pub surface: Vec<Vec2>,
}

impl Planet {
    pub fn new(center: Vec2, radius: f32, noise: &Noise) -> Self {
        let surface =
            Self::create_surface(360 * usize::from(MULTIPLE_360) + 1, center, radius, noise);
        let (extents, max_radius) = Self::calculate_extents(&surface, center);
        Self {
            center,
            radius,
            max_radius,
            extents,
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
        let padding = 200.0;
        let xoffset: i16 = rand::gen_range(padding as i16, NOISE_SIZE as i16 - padding as i16);
        let yoffset: i16 = rand::gen_range(padding as i16, NOISE_SIZE as i16 - padding as i16);
        for point in 0..surface_points {
            let a = point as f32 * std::f32::consts::TAU / surface_points as f32;
            let height = noise.get_point(
                a.sin().mul_add(padding - 20.0, f32::from(xoffset)) as u32,
                a.cos().mul_add(padding - 20.0, f32::from(yoffset)) as u32,
            ) * (radius / 2.0);
            surface.push(center + vec2((radius + height) * a.sin(), (radius + height) * a.cos()));
        }

        surface
    }

    fn calculate_extents(surface: &[Vec2], center: Vec2) -> (Rect, f32) {
        let mut min_x = std::f32::INFINITY;
        let mut min_y = std::f32::INFINITY;
        let mut max_x = std::f32::NEG_INFINITY;
        let mut max_y = std::f32::NEG_INFINITY;
        let mut max_radius = 0.0;
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
            let dist = point.distance(center);
            if dist > max_radius {
                max_radius = dist;
            }
        }
        let width = max_x - min_x;
        let height = max_y - min_y;
        debug_assert!(width >= 0.0);
        debug_assert!(height >= 0.0);
        (Rect::new(min_x, min_y, width, height), max_radius)
    }

    pub fn is_inside_expensive(planet: &Self, point: Vec2) -> bool {
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
            if diff < right_min && diff > 0.0 {
                right_min = diff;
                right = segment_angle;
                right_seg = *segment;
            }
            if diff > left_min && diff < 0.0 {
                left_min = diff;
                left = segment_angle;
                left_seg = *segment;
            }
        }
        let procent = map(angle, left, right, 0.0, 1.0);
        let lerp_x = lerp(left_seg.x, right_seg.x, procent);
        let lerp_y = lerp(left_seg.y, right_seg.y, procent);
        let surface_point = Vec2::new(lerp_x, lerp_y);
        point.distance(planet.center) < surface_point.distance(planet.center)
    }

    pub fn is_inside(planet: &Self, distance: f32, angle: f32) -> bool {
        debug_assert!(angle >= 0.0);
        let index = angle.ceil() as usize;
        let other_index = if angle < 1.0 {
            planet.surface.len() - 1
        } else {
            index - 1
        };
        let p1 = planet.surface[other_index];
        let p2 = planet.surface[index];
        let decimal = angle.fract();
        let x = lerp(p1.x, p2.x, decimal);
        let y = lerp(p1.y, p2.y, decimal);
        let surface_point = Vec2::new(x, y);
        distance < surface_point.distance(planet.center)
    }

    pub fn as_image(planet: &Self) -> Image {
        let Planet {
            center,
            radius,
            max_radius,
            extents,
            surface: _,
        } = planet;
        debug_assert!(extents.w < f32::from(std::u16::MAX));
        debug_assert!(extents.h < f32::from(std::u16::MAX));
        let width = extents.w.ceil() as u16;
        let height = extents.h.ceil() as u16;
        let mut bytes: Vec<u8> = Vec::with_capacity(usize::from(width) * usize::from(height) * 4);

        for y in 0..height {
            for x in 0..width {
                let point = Vec2::new(extents.x + x as f32, extents.y + y as f32);
                let distance = point.distance(*center);

                if distance < *radius {
                    // outer space
                    let color: [u8; 4] = color_u8!(110, 90, 55, 255).into();
                    bytes.extend(color);
                } else if distance > *max_radius {
                    // under the planet's crust
                    let color: [u8; 4] = color_u8!(0, 0, 0, 0).into();
                    bytes.extend(color);
                } else {
                    let angle = (std::f32::consts::PI + point.angle_between(Vec2::new(0.0, -1.0)))
                        .to_degrees()
                        * f32::from(MULTIPLE_360);
                    let up = (distance - *radius).floor() as i32;
                    let color: [u8; 4] = if Self::is_inside(planet, distance, angle) {
                        match up {
                            0..=99 => color_u8!(221, 181, 110, 255).into(),
                            100..=199 => color_u8!(178, 166, 72, 255).into(),
                            200..=299 => color_u8!(85, 150, 83, 255).into(),
                            300..=399 => color_u8!(141, 153, 40, 255).into(),
                            400..=499 => color_u8!(125, 165, 123, 255).into(),
                            500..=599 => color_u8!(193, 212, 169, 255).into(),
                            600..=600 => color_u8!(255, 255, 212, 255).into(),
                            _ => color_u8!(255, 255, 255, 255).into(),
                        }
                    } else if up < 200 {
                        color_u8!(13, 148, 138, 255).into()
                    } else {
                        // outer space;
                        color_u8!(0, 0, 0, 0).into()
                    };
                    bytes.extend(color);
                };
            }
        }

        Image {
            bytes,
            width,
            height,
        }
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
