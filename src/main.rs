extern crate sdl2;

use std::f64::consts::PI;
use std::time::{Duration, Instant};
use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;

const WINDOW_WIDTH: u32 = 640;
const WINDOW_HEIGHT: u32 = 480;


fn main() {
    //

    let mut last_clk = Instant::now();
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Triangle", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGBA(0, 0, 0, 0));
    canvas.clear();

    let mut event_pump = sdl_context.event_pump().unwrap();
    // Accumulators for R, G, B start with a (2*PI/3.0) rad gap from each other
    let mut acc_r = Duration::from_millis((1.0 * (2.0 * PI / 3.0) * 1000.0) as u64);
    let mut acc_g = Duration::from_millis((2.0 * (2.0 * PI / 3.0) * 1000.0) as u64);
    let mut acc_b = Duration::from_millis((3.0 * (2.0 * PI / 3.0) * 1000.0) as u64);

    // Mid canvas as both x and y offset
    let mut x_offset = (WINDOW_WIDTH / 2) as i32;
    let mut y_offset = (WINDOW_HEIGHT / 2) as i32;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Q), .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                }
                Event::MouseMotion { timestamp, window_id, which, mousestate, x, y, xrel, yrel } => {
                    x_offset = x;
                    y_offset = y;

                    //#[cfg(debug_assertions)]
                    //println!("timestamp={timestamp:?}, window_id={window_id:?}, which={which:?}, mousestate={mousestate:?}, x={x:?}, y={y:?}, xrel={xrel:?}, yrel={yrel:?}")
                }
                _ => {}
            }
        }

        // Accumulators as f64 for R, G, B
        let accfr = acc_r.as_secs_f64();
        let accfg = acc_g.as_secs_f64();
        let accfb = acc_b.as_secs_f64();

        let triangle_points = [
            Point::new(x_offset + ((accfr.cos() * 100.0) as i32), y_offset + ((accfr.sin() * 100.0) as i32)),
            Point::new(x_offset + ((accfg.cos() * 100.0) as i32), y_offset + ((accfg.sin() * 100.0) as i32)),
            Point::new(x_offset + ((accfb.cos() * 100.0) as i32), y_offset + ((accfb.sin() * 100.0) as i32)),
        ];

        //#[cfg(debug_assertions)]
        //println!("triangle(v0, v1, v2): {triangle_points:?}");

        // Update clock and frame duration
        let curr_clk = Instant::now();
        let frame_dur_acc = curr_clk - last_clk;
        last_clk = curr_clk;

        // Add frame duration to accumulators
        acc_r += frame_dur_acc;
        acc_g += frame_dur_acc;
        acc_b += frame_dur_acc;

        let (r, g, b) = (
            ((accfr.sin() * accfr.cos()).powi(1) * 255.0) as u8,
            ((accfg.sin() * accfg.sin()).powi(1) * 255.0) as u8,
            ((accfb.sin() * accfb.sin()).powi(1) * 255.0) as u8,
        );

        //#[cfg(debug_assertions)]
        //println!("color: ({r:?}, {g:?}, {b:?}), acc(r,g,b)=({acc_r:?}, {acc_g:?}, {acc_b:?})");

        let (vx, vy) = (
            [triangle_points[0].x as i16, triangle_points[1].x as i16, triangle_points[2].x as i16],
            [triangle_points[0].y as i16, triangle_points[1].y as i16, triangle_points[2].y as i16],
        );

        canvas.set_draw_color(Color::RGBA(0, 0, 0, 0));
        canvas.clear();
        canvas.filled_polygon(&vx, &vy, Color::RGB(r, g, b)).unwrap_or_else(|err| println!("{:?}", err));
        canvas.present();
    }
}
