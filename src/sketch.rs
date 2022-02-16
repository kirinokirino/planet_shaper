use crate::*;

use macroquad::math::clamp;
use macroquad::shapes::draw_triangle;
use macroquad::texture::{draw_texture, Image, Texture2D};

use macroquad::math::{Mat3, XY};

use simple_simplex::NoiseConfig;

use once_cell::sync::OnceCell;

const NOISE_SIZE: u16 = 1000;
static mut NOISE: Noise = Noise::new();
static mut CIRCLE: OnceCell<Circle> = OnceCell::new();
static mut FRAME: u64 = 0;
static mut PLAYER: OnceCell<Square> = OnceCell::new();
pub static mut CAMERA_FOLLOW: OnceCell<Option<(Vec2, f32)>> = OnceCell::new();
pub fn setup() {
    unsafe { PLAYER.get_or_init(|| Square::new(vec2(1700.0, 100.0))) };
    unsafe { CAMERA_FOLLOW.get_or_init(|| None) };
}

pub fn draw(_delta: f64) {
    let frame = unsafe { FRAME };
    let lmb = is_mouse_button_pressed(MouseButton::Left);
    let W = is_key_down(KeyCode::W) || is_key_down(KeyCode::Comma);
    let S = is_key_down(KeyCode::S) || is_key_down(KeyCode::O);
    let A = is_key_down(KeyCode::A);
    let D = is_key_down(KeyCode::D) || is_key_down(KeyCode::E);

    let mut player = unsafe { PLAYER.get_mut().expect("Should be initialized in setup.") }; //rotated(-std::f32::consts::PI * frame as f32 * 0.001);
    let mut camera_follow = unsafe {
        CAMERA_FOLLOW
            .get_mut()
            .expect("Should be initialized in setup.")
    };

    if D {
        player.rotation += (-0.01);
    } else if A {
        player.rotation -= (-0.01);
    }

    if W {}
    *camera_follow = Some((player.center, player.rotation));
    player.draw();
    unsafe {
        CIRCLE
            .get_or_init(|| Circle::new(vec2(0.0, 0.0), 1500.0))
            .draw();
    }
    if lmb {
        let circle = Circle::new(vec2(0.0, 0.0), 1500.0);
        unsafe {
            *CIRCLE
                .get_mut()
                .expect("CIRCLE should already be initialised") = circle;
        }
    }

    draw_ui();

    unsafe {
        FRAME += 1;
    }
}

fn draw_ui() {
    // Screen space, render fixed ui
    set_default_camera();
    draw_text(
        &format!("mouse: {:?}, fps: {}", mouse_position(), get_fps()),
        10.0,
        20.0,
        30.0,
        colors::BLACK,
    );
}

pub fn lerp(from: f32, to: f32, p: f32) -> f32 {
    from.mul_add(1.0 - p, to * p)
}

pub fn map(value: f32, start1: f32, stop1: f32, start2: f32, stop2: f32) -> f32 {
    (value - start1) / (stop1 - start1) * (stop2 - start2) + start2
}

pub fn norm(value: f32, start: f32, stop: f32) -> f32 {
    map(value, start, stop, 0.0, 1.0)
}

struct Square {
    pub center: Vec2,
    size: f32,
    pub rotation: f32,
}

impl Square {
    pub const fn new(center: Vec2) -> Self {
        Self {
            center,
            size: 25.0,
            rotation: 0.0,
        }
    }
    pub const fn rotated(self, rotation: f32) -> Self {
        Self {
            center: self.center,
            size: self.size,
            rotation,
        }
    }
    pub const fn sized(self, size: f32) -> Self {
        Self {
            center: self.center,
            size,
            rotation: self.rotation,
        }
    }

    fn corners(size: f32, center: Vec2) -> [Vec2; 4] {
        let half_size = size / 2.0;
        let (x, y) = (0.0, 0.0);
        [
            Vec2::new(x - half_size, y - half_size),
            Vec2::new(x + half_size, y - half_size),
            Vec2::new(x + half_size, y + half_size),
            Vec2::new(x - half_size, y + half_size),
        ]
    }

    fn rotate(p: [Vec2; 4], rotation: f32) -> [Vec2; 4] {
        let r = Mat3::from_rotation_z(rotation);
        [
            r.transform_point2(p[0]),
            r.transform_point2(p[1]),
            r.transform_point2(p[2]),
            r.transform_point2(p[3]),
        ]
    }

    pub fn draw(&self) {
        let corners = Self::rotate(Self::corners(self.size, self.center), self.rotation);
        let thickness = 5.0;
        let color = color_u8!(155, 155, 155, 155);
        let rot_matrix = Mat3::from_rotation_z(self.rotation);

        let rot_point = rot_matrix.transform_vector2(vec2(0.0, self.size / 2.0));
        draw_line(
            self.center.x,
            self.center.y,
            self.center.x + rot_point.x,
            self.center.y + rot_point.y,
            thickness,
            color,
        );
        draw_line(
            self.center.x + corners[0].x,
            self.center.y + corners[0].y,
            self.center.x + corners[1].x,
            self.center.y + corners[1].y,
            thickness,
            color,
        );
        draw_line(
            self.center.x + corners[1].x,
            self.center.y + corners[1].y,
            self.center.x + corners[2].x,
            self.center.y + corners[2].y,
            thickness,
            color,
        );
        draw_line(
            self.center.x + corners[2].x,
            self.center.y + corners[2].y,
            self.center.x + corners[3].x,
            self.center.y + corners[3].y,
            thickness,
            color,
        );
        draw_line(
            self.center.x + corners[3].x,
            self.center.y + corners[3].y,
            self.center.x + corners[0].x,
            self.center.y + corners[0].y,
            thickness,
            color,
        );
    }
}

struct Circle {
    center: Vec2,
    radius: f32,

    surface: Vec<Vec2>,
}

impl Circle {
    pub fn new(center: Vec2, radius: f32) -> Self {
        Self {
            center,
            radius,
            surface: Self::create_surface(500, center, radius),
        }
    }

    fn create_surface(surface_points: usize, center: Vec2, radius: f32) -> Vec<Vec2> {
        let mut surface: Vec<Vec2> = Vec::with_capacity(surface_points);
        let xoffset: i16 = rand::gen_range(100, 900);
        let yoffset: i16 = rand::gen_range(100, 900);
        for point in 0..surface_points {
            let a = point as f32 * std::f32::consts::TAU / surface_points as f32;
            let noise = unsafe {
                NOISE.get_point(
                    (a.sin() * 80.0 + f32::from(xoffset)) as u32,
                    (a.cos() * 80.0 + f32::from(yoffset)) as u32,
                ) * (radius / 2.0)
            };
            surface.push(center + vec2((radius + noise) * a.sin(), (radius + noise) * a.cos()));
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

        let mut last_point = self.surface.last().expect("No points in surface");
        for point in &self.surface {
            draw_line(
                point.x,
                point.y,
                last_point.x,
                last_point.y,
                2.0,
                color_u8!(0, 0, 0, 255),
            );
            last_point = point;
        }
    }
}

struct Noise {
    image: OnceCell<Image>,
    texture: OnceCell<Texture2D>,
}

impl Noise {
    pub const fn new() -> Self {
        Self {
            image: OnceCell::new(),
            texture: OnceCell::new(),
        }
    }

    pub fn get_point(&self, x: u32, y: u32) -> f32 {
        self.image.get_or_init(Self::gen_image).get_pixel(x, y).r
    }

    pub fn gen_image() -> Image {
        let simplex: NoiseConfig = NoiseConfig::new(
            4,                   // Octaves
            0.01,                // X-Frequency
            0.01,                // Y-Frequency
            0.05,                // Amplitude
            3.0,                 // Lacunarity
            0.25,                // Gain
            (0.0, 255.0),        // range
            rand::rand().into(), // seed
        );

        let mut image = Image::gen_image_color(NOISE_SIZE, NOISE_SIZE, color_u8!(255, 0, 255, 255));

        for y in 0..NOISE_SIZE {
            for x in 0..NOISE_SIZE {
                let color: u8 = simplex.generate_range(x.into(), y.into()) as u8;
                let color = color_u8!(color, color, color, 255);
                image.set_pixel(u32::from(x), u32::from(y), color);
            }
        }
        image
    }
    pub fn draw_at(&self, x: f32, y: f32) {
        draw_texture(
            *self
                .texture
                .get_or_init(|| Texture2D::from_image(self.image.get_or_init(Self::gen_image))),
            x,
            y,
            color_u8!(255, 255, 255, 255),
        );
    }
}
