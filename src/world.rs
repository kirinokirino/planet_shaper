use crate::camera::{top_down_camera_controls, Camera};
use macroquad::prelude::*;
use once_cell::sync::OnceCell;

use crate::common::*;
use crate::noise::Noise;
use crate::planet::Planet;

pub const NOISE_SIZE: u16 = 2000;

pub struct World {
    planet: OnceCell<Planet>,
    planet_texture: Option<Texture2D>,
    noise: Noise,

    main_camera: Camera,
}

impl World {
    #[must_use]
    pub fn new() -> Self {
        Self {
            planet: OnceCell::new(),
            planet_texture: None,
            noise: Noise::new(),
            main_camera: Camera::new(),
        }
    }

    pub fn setup(&mut self) {
        let noise = &self.noise;
        let planet = self
            .planet
            .get_or_init(|| Planet::new(vec2(0.0, 0.0), 1500.0, noise));
        let planet_image = Planet::as_image(planet);
        self.planet_texture = Some(Texture2D::from_image(&planet_image));
    }

    pub fn input(&mut self) {
        let lmb = is_mouse_button_pressed(MouseButton::Left);
        let _w = is_key_down(KeyCode::W) || is_key_down(KeyCode::Comma);
        let _s = is_key_down(KeyCode::S) || is_key_down(KeyCode::O);
        let _a = is_key_down(KeyCode::A);
        let _d = is_key_down(KeyCode::D) || is_key_down(KeyCode::E);

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
                let new_planet = Planet::new(vec2(0.0, 0.0), 1500.0, &self.noise);
                let planet_image = Planet::as_image(&new_planet);
                self.planet_texture = Some(Texture2D::from_image(&planet_image));
                std::mem::replace(planet, new_planet);
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
        let is_inside_planet = Planet::is_inside_expensive(planet, mouse);
        if is_inside_planet {
            draw_circle(mouse.x, mouse.y, 10.0, color_u8!(0, 255, 0, 255));
        } else {
            draw_circle(mouse.x, mouse.y, 10.0, color_u8!(255, 0, 0, 255));
        }

        let _viewport = self.main_camera.viewport_rect();
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
        if let Some(planet_texture) = self.planet_texture {
            draw_texture(
                planet_texture,
                4000.0,
                -2000.0,
                color_u8!(255, 255, 255, 255),
            );
        }
    }
}
