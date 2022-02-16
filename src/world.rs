use crate::camera::{top_down_camera_controls, Camera};
use macroquad::prelude::*;
use once_cell::sync::OnceCell;

use crate::common::*;
use crate::noise::Noise;
use crate::planet::Planet;

pub const NOISE_SIZE: u16 = 1000;

pub struct World {
    planet: OnceCell<Planet>,
    noise: Noise,

    main_camera: Camera,
}

impl World {
    #[must_use]
    pub fn new() -> Self {
        Self {
            planet: OnceCell::new(),
            noise: Noise::new(),
            main_camera: Camera::new(),
        }
    }

    pub fn setup(&mut self) {
        let noise = &self.noise;
        self.planet
            .get_or_init(|| Planet::new(vec2(0.0, 0.0), 1500.0, noise));
    }

    pub fn input(&mut self) {
        let lmb = is_mouse_button_pressed(MouseButton::Left);
        let W = is_key_down(KeyCode::W) || is_key_down(KeyCode::Comma);
        let S = is_key_down(KeyCode::S) || is_key_down(KeyCode::O);
        let A = is_key_down(KeyCode::A);
        let D = is_key_down(KeyCode::D) || is_key_down(KeyCode::E);

        if lmb {
            let camera = self.main_camera;
            debug!(
                "{}",
                format!(
                    "target: {}, zoom: {:?}, view_port: {:?}",
                    camera.target,
                    camera.zoom,
                    camera.viewport_size(),
                )
            );
            let mouse = camera.mouse_world_position();
            debug!("mouse: {:?}, mouse_world: {}", mouse_position(), mouse);

            if let Some(planet) = self.planet.get_mut() {
                std::mem::replace(planet, Planet::new(vec2(0.0, 0.0), 1500.0, &self.noise));
            }
        }

        if is_key_down(KeyCode::LeftControl) {
            top_down_camera_controls(&mut self.main_camera);
        }
    }

    pub fn update(&mut self) {
        self.main_camera.update();
    }

    pub fn draw(&self) {
        clear_background(color_u8!(0, 0, 0, 255));
        let zoom = vec2(self.main_camera.zoom.x, -self.main_camera.zoom.y);
        set_camera(&Camera2D {
            target: self.main_camera.target,
            rotation: -self.main_camera.rotation.to_degrees(),
            zoom,
            ..Camera2D::default()
        });

        let planet = unsafe { self.planet.get_unchecked() };
        planet.draw();

        let mouse = self.main_camera.mouse_world_position();
        let (x, y) = (mouse.x, mouse.y);
        let mouse = Vec2::new(x, y);
        let angle = mouse.angle_between(Vec2::new(0.0, 1.0));
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
        //draw_circle(left_seg.x, left_seg.y, 30.0, color_u8!(255, 0, 255, 255));
        //draw_circle(right_seg.x, right_seg.y, 30.0, color_u8!(0, 255, 255, 255));
        draw_circle(lerp_x, lerp_y, 10.0, color_u8!(0, 255, 0, 255));
        let surface_point = Vec2::new(lerp_x, lerp_y);
        let is_inside_planet =
            mouse.distance(planet.center) < surface_point.distance(planet.center);
        if is_inside_planet {
            draw_circle(x, y, 10.0, color_u8!(0, 255, 0, 255));
        } else {
            draw_circle(x, y, 10.0, color_u8!(255, 0, 0, 255));
        }

        let mut viewport = self.main_camera.viewport_rect();
        let (width, height) = (screen_width(), screen_height());
        let (center_x, center_y) = (self.main_camera.target.x, self.main_camera.target.y);
        let top_left_x = center_x - width;
        let top_left_y = center_y - height;
        draw_rectangle_lines(
            top_left_x,
            top_left_y,
            width * 2.0,
            height * 2.0,
            50.0,
            color_u8!(50, 120, 100, 100),
        );
    }
}
