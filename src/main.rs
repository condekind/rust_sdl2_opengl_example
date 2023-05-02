extern crate sdl2;

use std::time::{Duration, Instant};
use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;

const WINDOW_WIDTH: u32 = 640;
const WINDOW_HEIGHT: u32 = 480;

fn transpose<T>(original: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!original.is_empty());
    let mut transposed = (0..original[0].len()).map(|_| vec![]).collect::<Vec<_>>();

    for original_row in original {
        for (item, transposed_row) in original_row.into_iter().zip(&mut transposed) {
            transposed_row.push(item);
        }
    }

    transposed
}

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

    canvas.set_draw_color(Color::RGBA(140, 90, 150, 0));
    //canvas.clear();

    //let mut rng: ThreadRng = rand::thread_rng();
    //const FRAME_DUR_RANGE: Range<Duration> = Duration::from_micros(14000)..Duration::from_millis(19);
    //let between = Uniform::from(0..255);
    //let frame_dur_distr = Uniform::from(FRAME_DUR_RANGE);

    // let (r, g, b) = (
    //     between.sample(&mut rng),
    //     between.sample(&mut rng),
    //     between.sample(&mut rng),
    // );

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut acc = Duration::from_millis(1000);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {},
            }
        }
        //let delta_cost = (frame_dur_distr.sample(&mut rng).as_secs_f64() * 1000.0);
        //println!("delta_cost = {delta_cost}");

        // ((acc.as_secs_f64().sin() + acc.as_secs_f64().cos()) / std::f64::consts::SQRT_2 * 255.0) as i32
        let triangle_points = [
            Point::new((WINDOW_WIDTH / 2) as i32 + ((acc.as_secs_f64().sin()+0.1) * 100.0) as i32, (WINDOW_HEIGHT / 2) as i32 + ((acc.as_secs_f64().sin()+0.1) * 100.0) as i32),
            Point::new((WINDOW_WIDTH / 2) as i32 + ((acc.as_secs_f64().cos()+0.1) * 100.0) as i32, (WINDOW_HEIGHT / 2) as i32 + ((acc.as_secs_f64().cos()+0.1) * 100.0) as i32),
            Point::new((WINDOW_WIDTH / 2) as i32 + ((acc.as_secs_f64().sin()+0.1) * 200.0) as i32, (WINDOW_HEIGHT / 2) as i32 + ((acc.as_secs_f64().cos()+0.1) * 200.0) as i32),
        ];

        let curr_clk = Instant::now();
        let frame_dur_acc = curr_clk - last_clk;
        last_clk = curr_clk;
        acc += frame_dur_acc;
        // let (r, g, b) = (
        //     ((0.97*acc.as_secs_f64().cos()+0.97*acc.as_secs_f64().sin()) * 255.0) as u8,
        //     ((0.83*acc.as_secs_f64().cos()+0.83*acc.as_secs_f64().sin()) * 255.0) as u8,
        //     ((0.71*acc.as_secs_f64().cos()+0.71*acc.as_secs_f64().sin()) * 255.0) as u8,
        // );
        let (r, g, b) = (
            ((acc.as_secs_f64().sin()) * 255.0) as u8,
            ((0.83*acc.as_secs_f64().cos()+0.83*acc.as_secs_f64().sin()) * 255.0) as u8,
            ((0.71*acc.as_secs_f64().cos()+0.71*acc.as_secs_f64().sin()) * 255.0) as u8,
        );
        //sin(x)*(x as u32 / 360) as f64
        // ((acc.as_secs_f64().sin() + acc.as_secs_f64().cos()).abs()) / std::f64::consts::SQRT_2 * 255.0
        println!("color: ({r:?}, {g:?}, {b:?}), acc={acc:?}");
        //println!("clk: {curr_clk:?}, duration: {frame_dur_acc:?}");

        let vx = [
            triangle_points[0].x as i16, triangle_points[1].x as i16, triangle_points[2].x as i16,
        ];
        let vy = [
            triangle_points[0].y as i16, triangle_points[1].y as i16, triangle_points[2].y as i16,
        ];
        //println!("delta_cost = {triangle_points:?}");
        canvas.filled_polygon(&vx, &vy, Color::RGB(r, g, b)).unwrap_or_else(|err| println!("{:?}", err));
        canvas.present();
    }
}
