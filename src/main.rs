#![warn(
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    clippy::unwrap_used,
    clippy::unwrap_in_result,
    clippy::unneeded_field_pattern,
    clippy::string_to_string,
    clippy::string_slice,
    clippy::string_add,
    clippy::str_to_string,
    clippy::same_name_method,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::rc_mutex,
    clippy::rc_buffer,
    clippy::pattern_type_mismatch,
    clippy::multiple_inherent_impl,
    clippy::missing_enforced_import_renames,
    clippy::lossy_float_literal,
    clippy::let_underscore_must_use,
    clippy::integer_division,
    clippy::inline_asm_x86_att_syntax,
    clippy::indexing_slicing,
    clippy::if_then_some_else_none,
    clippy::get_unwrap,
    clippy::fn_to_numeric_cast,
    clippy::float_cmp_const,
    clippy::filetype_is_file,
    clippy::create_dir,
    clippy::clone_on_ref_ptr,
    clippy::as_conversions,
    clippy::verbose_file_reads
)]
#![allow(clippy::wildcard_imports, unused_imports, clippy::cast_precision_loss)]
use macroquad::camera::{set_camera, set_default_camera, Camera2D};
use macroquad::color::{colors, Color};
use macroquad::color_u8;
use macroquad::input::{
    is_key_down, is_mouse_button_pressed, mouse_position, KeyCode, MouseButton,
};
use macroquad::logging::*;
use macroquad::math::{vec2, Vec2};
use macroquad::rand;
use macroquad::shapes::{draw_circle, draw_line, draw_rectangle};
use macroquad::text::draw_text;
use macroquad::time::{get_fps, get_time};
use macroquad::window::{clear_background, next_frame, screen_height, screen_width};

mod sketch;
use sketch::{draw, setup};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Camera {
    target: Vec2,
    zoom: Vec2,
}

impl Camera {
    #[must_use]
    pub fn mouse_position(&self) -> Vec2 {
        let mouse = mouse_position();
        Vec2::new(
            ((mouse.0 - screen_width() / 2.0) / (screen_width() / 2.0) / self.zoom.x)
                + self.target.x,
            ((-mouse.1 + screen_height() / 2.0)
                / (screen_height() / 2.0)
                / self.zoom.x
                / (screen_width() / screen_height()))
                + self.target.y,
        )
    }
}

fn move_camera(camera: &mut Camera) {
    // scroll
    if is_key_down(KeyCode::Comma) {
        camera.target.y += 0.01 / camera.zoom.x;
    }
    if is_key_down(KeyCode::O) {
        camera.target.y -= 0.01 / camera.zoom.x;
    }
    if is_key_down(KeyCode::A) {
        camera.target.x -= 0.01 / camera.zoom.x;
    }
    if is_key_down(KeyCode::E) {
        camera.target.x += 0.01 / camera.zoom.x;
    }
    // zoom
    if is_key_down(KeyCode::PageUp) || is_key_down(KeyCode::Apostrophe) {
        camera.zoom.x *= 0.98;
        camera.zoom.y *= 0.98;
    }
    if is_key_down(KeyCode::PageDown) || is_key_down(KeyCode::Period) {
        camera.zoom.x /= 0.98;
        camera.zoom.y /= 0.98;
    }
}

// I don't know how to apply this line.
#[allow(clippy::future_not_send, clippy::too_many_lines)]
#[macroquad::main("Name")]
async fn main() {
    let mut sketch = Sketch::new(setup, draw);

    sketch.setup();

    let starting_zoom = 1.0 / screen_width();
    let mut main_camera = Camera {
        target: vec2(0.0, 0.0),
        zoom: vec2(
            starting_zoom,
            starting_zoom * screen_width() / screen_height(),
        ),
    };

    loop {
        move_camera(&mut main_camera);

        // Camera space, render game objects
        set_camera(&Camera2D {
            target: main_camera.target,
            zoom: main_camera.zoom,
            ..Camera2D::default()
        });

        sketch.draw();

        next_frame().await;
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
struct Time {
    elapsed_seconds: f64,
    overall_time: f64,
}
struct Sketch<SetupCallBack, DrawCallBack>
where
    SetupCallBack: FnMut(),
    DrawCallBack: FnMut(f64),
{
    setup_callback: SetupCallBack,
    draw_callback: DrawCallBack,

    time: Time,
}

impl<SetupCallBack, DrawCallBack> Sketch<SetupCallBack, DrawCallBack>
where
    SetupCallBack: FnMut(),
    DrawCallBack: FnMut(f64),
{
    fn new(setup: SetupCallBack, draw: DrawCallBack) -> Self {
        Self {
            setup_callback: setup,
            draw_callback: draw,

            time: Time::default(),
        }
    }

    #[allow(dead_code)]
    fn set_setup(&mut self, setup: SetupCallBack) {
        self.setup_callback = setup;
    }

    #[allow(dead_code)]
    fn set_draw(&mut self, draw: DrawCallBack) {
        self.draw_callback = draw;
    }

    fn setup(&mut self) {
        (self.setup_callback)();
    }

    fn draw(&mut self) {
        let delta = get_time() - self.time.overall_time;
        self.time = Time {
            elapsed_seconds: delta,
            overall_time: get_time(),
        };

        clear_background(color_u8!(255, 255, 255, 255));
        (self.draw_callback)(delta);
    }
}
