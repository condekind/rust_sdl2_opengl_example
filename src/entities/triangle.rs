use sdl2::rect::Point;
use std::time::Duration;

use crate::config::video::THIRD_OF_CIRCUMFERENCE;

#[derive(Debug)]
pub struct RGB(pub u8, pub u8, pub u8);

#[derive(Debug)]
pub struct TrianglePoints(pub Point, pub Point, pub Point);

#[derive(Debug)]
pub struct Triangle {
    x_offset: i32,
    y_offset: i32,
    pub(crate) point: TrianglePoints,
    pub color: RGB,
}

impl Triangle {
    pub const CIRC_THIRDS: (f64, f64, f64) = (
        0.0 * THIRD_OF_CIRCUMFERENCE,
        1.0 * THIRD_OF_CIRCUMFERENCE,
        2.0 * THIRD_OF_CIRCUMFERENCE,
    );

    pub fn new(x_offset: i32, y_offset: i32) -> Self {
        // The initial color is derived from 3 points on the edge of a
        // circumference equally distant from each other
        let (r, g, b) = (
            (Self::CIRC_THIRDS.0.cos() * 255.0) as u8,
            (Self::CIRC_THIRDS.1.cos() * 255.0) as u8,
            (Self::CIRC_THIRDS.2.cos() * 255.0) as u8,
        );

        let triangle_points = TrianglePoints(
            Point::new(
                x_offset + ((Self::CIRC_THIRDS.0.cos() * 100.0) as i32),
                y_offset + ((Self::CIRC_THIRDS.0.sin() * 100.0) as i32),
            ),
            Point::new(
                x_offset + ((Self::CIRC_THIRDS.1.cos() * 100.0) as i32),
                y_offset + ((Self::CIRC_THIRDS.1.sin() * 100.0) as i32),
            ),
            Point::new(
                x_offset + ((Self::CIRC_THIRDS.2.cos() * 100.0) as i32),
                y_offset + ((Self::CIRC_THIRDS.2.sin() * 100.0) as i32),
            ),
        );

        Self {
            x_offset,
            y_offset,
            point: triangle_points,
            color: RGB(r, g, b),
        }
    }

    pub fn update(&mut self, sim_cost: Duration, x_offset: i32, y_offset: i32, acc: &mut Duration) {
        self.x_offset = x_offset;
        self.y_offset = y_offset;

        // Add frame duration to accumulators
        *acc += sim_cost;

        // Accumulators as f64 for R, G, B
        let accf = acc.as_secs_f64();

        // Colors values (RGB): each varying 0..1 (=255), separated by 2pi/3
        // due to the initial acc difference (when initialized)
        (self.color.0, self.color.1, self.color.2) = (
            ((Self::CIRC_THIRDS.0 + accf).cos() * 255.0) as u8,
            ((Self::CIRC_THIRDS.1 + accf).cos() * 255.0) as u8,
            ((Self::CIRC_THIRDS.2 + accf).cos() * 255.0) as u8,
        );

        // Adding mouse coords as offsets so our triangle is drawn at the cursor
        self.point = TrianglePoints(
            Point::new(
                self.x_offset + (((Self::CIRC_THIRDS.0 + accf).cos() * 100.0) as i32),
                self.y_offset + (((Self::CIRC_THIRDS.0 + accf).sin() * 100.0) as i32),
            ),
            Point::new(
                self.x_offset + (((Self::CIRC_THIRDS.1 + accf).cos() * 100.0) as i32),
                self.y_offset + (((Self::CIRC_THIRDS.1 + accf).sin() * 100.0) as i32),
            ),
            Point::new(
                self.x_offset + (((Self::CIRC_THIRDS.2 + accf).cos() * 100.0) as i32),
                self.y_offset + (((Self::CIRC_THIRDS.2 + accf).sin() * 100.0) as i32),
            ),
        );

        #[cfg(debug_assertions)]
        println!(
            "triangle(v0, v1, v2): {0:?}, {1:?} {2:?}",
            self.point.0, self.point.1, self.point.2,
        );
    }
}
