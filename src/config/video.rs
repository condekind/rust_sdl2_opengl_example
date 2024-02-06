use std::f64::consts::PI;

pub const WINDOW_WIDTH: u32 = 640;
pub const WINDOW_HEIGHT: u32 = 480;
pub const DESIRED_FPS: u32 = 60;
pub const MS_PER_FRAME: f64 = 1000.0 / DESIRED_FPS as f64;
pub const TICKS_PER_FRAME: u32 = 30;

pub const THIRD_OF_CIRCUMFERENCE: f64 = 2.0 * PI / 3.0;
