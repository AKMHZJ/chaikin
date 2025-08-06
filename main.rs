use speedy2d::color::Color;
use speedy2d::dimen::Vector2;
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

/// Performs one iteration of Chaikin's algorithm for an open polyline.
/// This version is modified to ensure the resulting curve passes through the
/// original start and end points by adding them to the refined point list.
fn chaikin_step(points: &[(f32, f32)]) -> Vec<(f32, f32)> {
    // If there's less than one segment, no "corner cutting" can be done.
    if points.len() < 2 {
        return points.to_vec();
    }
    let mut new_points = Vec::new();

    // 1. Add the very first control point to anchor the start of the curve.
    new_points.push(points[0]);

    // 2. Generate two new points for each segment in the original polyline.
    for segment in points.windows(2) {
        let p0 = segment[0];
        let p1 = segment[1];
        // Point 'q' is 1/4 of the way from p0 to p1.
        let q = (0.75 * p0.0 + 0.25 * p1.0, 0.75 * p0.1 + 0.25 * p1.1);
        // Point 'r' is 3/4 of the way from p0 to p1.
        let r = (0.25 * p0.0 + 0.75 * p1.0, 0.25 * p0.1 + 0.75 * p1.1);
        new_points.push(q);
        new_points.push(r);
    }

    // 3. Add the very last control point to anchor the end of the curve.
    // The length check at the start guarantees that .last() will not fail.
    new_points.push(*points.last().unwrap());
    
    new_points
}

/// Helper to draw a series of connected lines.
fn draw_line_strip(graphics: &mut Graphics2D, points: &[(f32, f32)], thickness: f32, color: Color) {
    for segment in points.windows(2) {
        let p0 = segment[0];
        let p1 = segment[1];
        graphics.draw_line(p0, p1, thickness, color);
    }
}

struct ChaikinApp {
    mouse_pos: Vector2<f32>,
    control_points: Vec<(f32, f32)>,
    displayed_points: Vec<(f32, f32)>,
    is_animating: bool,
    animation_step: u32,
    last_update: Instant,
}

impl ChaikinApp {
    fn new() -> Self {
        Self {
            mouse_pos: Vector2::ZERO,
            control_points: Vec::new(),
            displayed_points: Vec::new(),
            is_animating: false,
            animation_step: 0,
            last_update: Instant::now(),
        }
    }
}

impl WindowHandler for ChaikinApp {
    fn on_mouse_move(&mut self, _helper: &mut WindowHelper, position: Vector2<f32>) {
        self.mouse_pos = position;
    }

    fn on_mouse_button_down(&mut self, _helper: &mut WindowHelper, button: MouseButton) {
        if button == MouseButton::Left && !self.is_animating {
            self.control_points.push((self.mouse_pos.x, self.mouse_pos.y));
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
                VirtualKeyCode::Return => {
                    // Start animation only if there are enough points to form a curve.
                    if self.control_points.len() > 2 {
                        self.is_animating = true;
                        self.animation_step = 0;
                        self.last_update = Instant::now();
                        
                        // Calculate the first frame of the animation immediately.
                        let mut temp_points = self.control_points.clone();
                        for _ in 0..self.animation_step {
                            temp_points = chaikin_step(&temp_points);
                        }
                        self.displayed_points = temp_points;
                    }
                }
                _ => {}
            }
        }
    }

    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        graphics.clear_screen(Color::BLACK);

        if self.is_animating {
            // --- Animation Logic ---
            if self.last_update.elapsed() >= Duration::from_millis(ANIMATION_SPEED_MS) {
                self.animation_step += 1;
                if self.animation_step > MAX_STEPS {
                    self.animation_step = 0; // Loop animation
                }

                // Recalculate the curve from the original control points for the current step.
                let mut temp_points = self.control_points.clone();
                for _ in 0..self.animation_step {
                    temp_points = chaikin_step(&temp_points);
                }
                self.displayed_points = temp_points;
                self.last_update = Instant::now();
            }

            // Draw the smoothed curve for the current animation step.
            if self.displayed_points.len() > 1 {
                draw_line_strip(graphics, &self.displayed_points, 2.0, Color::GREEN);
            }
        } 
            // --- Drawing Mode (Not Animating) ---
            // Draw visual feedback based on the number of control points.
            match self.control_points.len() {
                0 => { /* Draw nothing */ }
                1 => {
                    // Draw a single point.
                    graphics.draw_circle(self.control_points[0], 5.0, Color::WHITE);
                }
                _ => {
                    // Draw the control points as circles.
                    for &point in &self.control_points {
                        graphics.draw_circle(point, 5.0, Color::WHITE);
                    }
                    // Draw the control polyline connecting the points.
                    // draw_line_strip(graphics, &self.control_points, 1.0, Color::RED);
                }
            // }
        }

        helper.request_redraw();
    }
}