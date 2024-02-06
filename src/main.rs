
extern crate sdl2;

use std::f64::consts::PI;
use std::time::{Duration, Instant};

use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};

mod config;
pub use crate::config::video::*;

mod entities;
use crate::entities::triangle::*;


fn main() {
    // rust/SDL playground

    // Init SDL
    let sdl_context = sdl2::init().unwrap();

    // Video
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Triangle", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    // Main canvas
    let mut canvas = window.into_canvas().build().unwrap();

    // Sample texture
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture(
        "assets/image/wooden_floor_seamless.png"
    ).unwrap();
    let query = texture.query();
    let (texture_width , texture_height) = (
        query.width,
        query.height
    );

    // Texture "target" (None would mean: stretch to fill the canvas)
    let destination = Rect::new(
        10,
        10,
        texture_width,
        texture_height,
    );

    #[cfg(debug_assertions)]
    println!("dest {destination:?}");

    let mut event_pump = sdl_context.event_pump().unwrap();
    // Accumulator for R, G, B
    // When used, must be added to 0/1/2 times a (2*PI/3.0) rad gap
    let mut acc = Duration::from_secs(THIRD_OF_CIRCUMFERENCE as u64);

    // Mid canvas as both x and y offset
    let mut x_offset = (WINDOW_WIDTH / 2) as i32;
    let mut y_offset = (WINDOW_HEIGHT / 2) as i32;

    // TODO: move to config
    let mut frame_dur_acc: Duration;
    let frame_duration = Duration::from_secs_f64(MS_PER_FRAME / 1000.0);
    let mut elapsed = Duration::from_secs(0);
    let mut credits = Duration::from_secs(0);

    // Simulation step "cost"
    let sim_cost = frame_duration / TICKS_PER_FRAME;

    // Init clock
    let mut last_clk = Instant::now();

    let mut triangle = Triangle::new(x_offset, y_offset);

    'running: loop {

        // Update clock and frame duration
        let curr_clk = Instant::now();
        frame_dur_acc = curr_clk - last_clk;
        last_clk = curr_clk;

        // Handle input
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Q), .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                }
                Event::MouseMotion { x, y, .. } => {
                    x_offset = x;
                    y_offset = y;

                }
                _ => {}
            }
        }

        // If the last iteration took too long, frameAcc is set to `msVal`
        // to stop the renderer from skipping too many simulation steps.
        if (frame_dur_acc > frame_duration) {
            frame_dur_acc = frame_duration;
        }
        credits += frame_dur_acc;

        // Fixed time simulation step loop. The loop might:
        //     - run more than once, if rendering is slow
        //     - not run at all, if rendering is too fast
        while credits >= sim_cost {
            credits -= sim_cost;

            triangle.update(
                sim_cost,
                x_offset,
                y_offset,
                &mut acc,
            );

            elapsed += sim_cost;
        }

        // Adding mouse coords as offsets so our triangle is drawn at the cursor
        // Transposed to conform to filled_polygon() (and possibly others).
        let (vx, vy) = (
            [triangle.point.0.x as i16, triangle.point.1.x as i16, triangle.point.2.x as i16],
            [triangle.point.0.y as i16, triangle.point.1.y as i16, triangle.point.2.y as i16],
        );

        /*
        #[cfg(debug_assertions)]
        println!("color: ({0:?}, {1:?}, {2:?}), acc(r,g,b)=({3:?}, {4:?}, {5:?})",
            triangle.color.0,
            triangle.color.1,
            triangle.color.2,
            (Triangle::CIRC_THIRDS.0 + acc.as_secs_f64()),
            (Triangle::CIRC_THIRDS.1 + acc.as_secs_f64()),
            (Triangle::CIRC_THIRDS.2 + acc.as_secs_f64()),
        );
        */

        canvas.set_draw_color(Color::RGBA(0, 0, 0, 0));
        canvas.clear();

        // Texture
        canvas.copy(&texture, None, Some(destination)).unwrap();

        // Triangle
        canvas.filled_polygon(
            &vx,
            &vy,
            Color::RGB(
                triangle.color.0,
                triangle.color.1,
                triangle.color.2,
            ),
        ).unwrap_or_else(|err| println!("{:?}", err));

        canvas.present();
    }
}
