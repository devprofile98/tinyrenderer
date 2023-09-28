use self::math::Vec2;
use std::mem::swap;
use tgars::{Color, TargaImage};

pub fn draw_line(image: &mut TargaImage, mut p1: Vec2, mut p2: Vec2, color: &Color) -> Vec<Vec2> {
    let mut steep = false;
    let mut all_y = Vec::new();
    if (p2.x as i32 - p1.x as i32).abs() < (p2.y as i32 - p1.y as i32).abs() {
        swap(&mut p1.x, &mut p1.y);
        swap(&mut p2.x, &mut p2.y);
        steep = true;
    }
    if p1.x > p2.x {
        swap(&mut p1.x, &mut p2.x);
        swap(&mut p1.y, &mut p2.y);
    }
    let dx = (p2.x - p1.x) as f32;
    for x in p1.x..=p2.x {
        let t = (x - p1.x) as f32 / dx;
        let y = (p1.y as f32 * (1.0f32 - t) + p2.y as f32 * t) as u16;
        all_y.push(Vec2 { x: x, y: y });
        if steep {
            image.set_pixel(x, y, color);
        } else {
            image.set_pixel(y, x, color);
        }
    }
    all_y
}

pub mod math;
pub mod model;
