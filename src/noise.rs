use crate::world::NOISE_SIZE;
use macroquad::prelude::*;
use once_cell::sync::OnceCell;
use simple_simplex::NoiseConfig;

use ::rand::prelude::random;

pub struct Noise {
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
            4,               // Octaves
            0.008,           // X-Frequency
            0.008,           // Y-Frequency
            0.05,            // Amplitude
            3.0,             // Lacunarity
            0.25,            // Gain
            (0.0, 1.0),      // range
            random::<u64>(), // seed
        );

        let mut image = Image::gen_image_color(NOISE_SIZE, NOISE_SIZE, color_u8!(255, 0, 255, 255));

        for y in 0..NOISE_SIZE {
            for x in 0..NOISE_SIZE {
                let color = simplex.generate_range(x.into(), y.into());
                let color = Color {
                    r: color,
                    g: color,
                    b: color,
                    a: 1.0,
                };
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
