use speedy2d::color::Color;
use speedy2d::dimen::Vector2;
use speedy2d::font::{Font, TextLayout, TextOptions};
use speedy2d::window::{KeyScancode, MouseButton, VirtualKeyCode, WindowHandler, WindowHelper};
use speedy2d::{Graphics2D, Window};
use std::time::{Duration, Instant};
const MAX_STEPS: u32 = 7;
const ANIMATION_SPEED_MS: u64 = 500;

fn main() {
    let window = Window::new_centered(
        "Chaikin's Algorithm | Left Click -> Add Point | Enter -> Animate | Esc -> Quit",
        (800, 600),
    )
    .unwrap();
    window.run_loop(ChaikinApp::new());
}

fn chaikin_step(points: &[(f32, f32)]) -> Vec<(f32, f32)> {
    if points.len() < 2 {
        return points.to_vec();
    }
    let mut new_points = Vec::new();

    new_points.push(points[0]);

    for segment in points.windows(2) {
        let p0 = segment[0];
        let p1 = segment[1];
        let q = (0.75 * p0.0 + 0.25 * p1.0, 0.75 * p0.1 + 0.25 * p1.1);
        let r = (0.25 * p0.0 + 0.75 * p1.0, 0.25 * p0.1 + 0.75 * p1.1);
        new_points.push(q);
        new_points.push(r);
    }

    new_points.push(*points.last().unwrap());

    new_points
}

fn draw_line_strip(graphics: &mut Graphics2D, points: &[(f32, f32)], thickness: f32, color: Color) {
    for segment in points.windows(2) {
        let p0 = segment[0];
        let p1 = segment[1];
        graphics.draw_line(p0, p1, thickness, color);
    }
}

struct ChaikinApp {
    font: Font,
    btn: bool,
    check: bool,
    mouse_pos: Vector2<f32>,
    control_points: Vec<(f32, f32)>,
    displayed_points: Vec<(f32, f32)>,
    is_animating: bool,
    animation_step: u32,
    last_update: Instant,
    window_size: Vector2<f32>,
}

impl ChaikinApp {
    fn new() -> Self {
        let font_data =
            std::fs::read("dejavu-fonts-ttf-2.37/ttf/DejaVuSerif-BoldItalic.ttf").unwrap();
        let font = Font::new(&font_data).unwrap();
        Self {
            font,
            btn: true,
            check: true,
            mouse_pos: Vector2::ZERO,
            control_points: Vec::new(),
            displayed_points: Vec::new(),
            is_animating: false,
            animation_step: 0,
            last_update: Instant::now(),
            window_size: Vector2::new(800.0, 600.0),
        }
    }
}

impl WindowHandler for ChaikinApp {
    fn on_mouse_move(&mut self, _helper: &mut WindowHelper, position: Vector2<f32>) {
        self.mouse_pos = position;
    }

    fn on_mouse_button_down(&mut self, _helper: &mut WindowHelper, button: MouseButton) {
        println!(
            "--------___- {:?}",
            (&self.control_points, self.is_animating)
        );
        if button == MouseButton::Left && !self.is_animating {
            self.control_points
                .push((self.mouse_pos.x, self.mouse_pos.y));
            self.check = true;
        }
    }

    fn on_key_down(
        &mut self,
        helper: &mut WindowHelper,
        virtual_keycode: Option<VirtualKeyCode>,
        _scancode: KeyScancode,
    ) {
        if let Some(key) = virtual_keycode {
            match key {
                VirtualKeyCode::Escape => helper.terminate_loop(),
                VirtualKeyCode::Backspace => {
                    let mouse_pos = self.mouse_pos;
                    *self = ChaikinApp::new();
                    self.mouse_pos = mouse_pos;
                }
                VirtualKeyCode::Return => {
                    if self.control_points.len() > 1 && self.btn {
                        self.is_animating = true;
                        self.animation_step = 0;
                        self.last_update = Instant::now();
                        self.btn = false;

                        let mut temp_points = self.control_points.clone();
                        for _ in 0..self.animation_step {
                            temp_points = chaikin_step(&temp_points);
                        }
                        self.displayed_points = temp_points;
                    } else if self.control_points.len() == 0 {
                        self.check = false;
                    }
                }
                _ => {}
            }
        }
    }

    fn on_resize(&mut self, _helper: &mut WindowHelper<()>, size_pixels: speedy2d::dimen::UVec2) {
        self.window_size = Vector2::new(size_pixels.x as f32, size_pixels.y as f32);
    }

    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        graphics.clear_screen(Color::BLACK);
        let size = self.window_size;
        let layout = self.font.layout_text(
            "click on the mouse to draw the ponit!!!\nBACKSPACE to reset",
            24.0,
            TextOptions::new(),
        );

        if self.is_animating {
            if self.last_update.elapsed() >= Duration::from_millis(ANIMATION_SPEED_MS) {
                self.animation_step += 1;
                if self.animation_step > MAX_STEPS {
                    self.animation_step = 0;
                }

                let mut temp_points = self.control_points.clone();
                for _ in 0..self.animation_step {
                    temp_points = chaikin_step(&temp_points);
                }
                self.displayed_points = temp_points;
                self.last_update = Instant::now();
            }

            if self.displayed_points.len() > 1 {
                draw_line_strip(graphics, &self.displayed_points, 2.0, Color::GREEN);
            }
        }
        let pos = Vector2::new(size.x / 2.0 - layout.width() / 2.0, size.y / 2.0);
        match self.control_points.len() {
            0 => {
                if !self.check {
                    graphics.draw_text(pos, Color::WHITE, &layout);
                }
            }
            _ => {
                for &point in &self.control_points {
                    graphics.draw_circle(point, 3.0, Color::WHITE);
                    graphics.draw_circle(point, 2.0, Color::BLACK);
                }
            }
        }

        helper.request_redraw();
    }
}
