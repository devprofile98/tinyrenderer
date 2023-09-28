// use crate::utils123::Vec3;
use crate::utils::{
    math::{Vec2, Vec3},
    model::Model,
};
use rand::Rng;
use std::{
    cmp::{max, min},
    mem::swap,
};
use tgars::{Color, TargaImage};

fn barycenteric(points: &[Vec2; 3], p: Vec2) -> Vec3 {
    let left_side = Vec3::new(
        points[2].x as f32 - points[0].x as f32,
        points[1].x as f32 - points[0].x as f32,
        points[0].x as f32 - p.x as f32,
    );
    let right_side = Vec3::new(
        points[2].y as f32 - points[0].y as f32,
        points[1].y as f32 - points[0].y as f32,
        points[0].y as f32 - p.y as f32,
    );
    let u = left_side.cross(&right_side);
    if u.z.abs() < 1.0 {
        Vec3::newi(-1, 1, 1)
    } else {
        Vec3::new(1.0 - (u.x + u.y) / u.z, u.y / u.z, u.x / u.z)
    }
}

fn triangle_bc(image: &mut TargaImage, p: Vec3, p1: Vec3, p2: Vec3) {
    let light_dir = Vec3::newi(0, 0, -1); // define light_dir
    let mut bboxmin = Vec2 { x: 1023, y: 1023 };
    let mut bboxmax = Vec2 { x: 0, y: 0 };
    let mut rng = rand::thread_rng();
    for p in [&p, &p1, &p2] {
        bboxmin.x = min(bboxmin.x, p.x as u16);
        bboxmin.y = min(bboxmin.y, p.y as u16);

        bboxmax.x = max(bboxmax.x, p.x as u16);
        bboxmax.y = max(bboxmax.y, p.y as u16);
    }

    for x in bboxmin.x..bboxmax.x {
        for y in bboxmin.y..bboxmax.y {
            let bc = barycenteric(
                &[Vec2::from(&p), Vec2::from(&p1), Vec2::from(&p2)],
                Vec2 { x, y },
            );
            if bc.x < 0.0 || bc.y < 0.0 || bc.z < 0.0 {
                continue;
            }
            let mut face_normal = (p2.clone() - p.clone()).cross(&(p1.clone() - p.clone()));
            face_normal.normalize();
            let intensity = face_normal.dot(&light_dir);
            // if intensity >= 0.9 {}
            if intensity > 0.0 {
                image.set_pixel(
                    y,
                    x,
                    &Color::rgb(
                        (intensity * 255.0) as u8,
                        (intensity * 255.0) as u8,
                        (intensity * 255.0) as u8,
                    ),
                );
            }
        }
    }
}

#[allow(dead_code)]
fn calc_xy(first: Vec2, sec: Vec2, c: f32) -> Vec2 {
    Vec2 {
        x: ((first.x as f32 + ((sec.x as i32 - first.x as i32) as f32 * c)) as u16),
        y: ((first.y as f32 + ((sec.y as i32 - first.y as i32) as f32 * c)) as u16),
    }
}

#[allow(dead_code)]
fn triangle(image: &mut TargaImage, mut p: Vec2, mut p1: Vec2, mut p2: Vec2) {
    let mut rng = rand::thread_rng();
    if p.y > p1.y {
        swap(&mut p, &mut p1);
    }
    if p.y > p2.y {
        swap(&mut p, &mut p2);
    }
    if p1.y > p2.y {
        swap(&mut p1, &mut p2);
    }
    let total_height = p2.y - p.y;
    let color = Color::rgb(
        rng.gen_range(50..=200),
        rng.gen_range(50..=200),
        rng.gen_range(50..=200),
    );
    for y in p.y..p2.y {
        let alpha = (y - p.y) as f32 / total_height as f32;
        let (chosen_p, seg_height) = if y < p1.y {
            (p, p1.y - p.y + 1)
        } else {
            (p1, p2.y - p1.y + 1)
        };
        let beta = (y - chosen_p.y) as f32 / seg_height as f32;
        let mut pa = calc_xy(p, p2, alpha);
        let mut pb = if y < p1.y {
            calc_xy(p, p1, beta)
        } else {
            calc_xy(p1, p2, beta)
        };
        if pa.x > pb.x {
            swap(&mut pa, &mut pb);
        }
        for j in pa.x..=pb.x {
            image.set_pixel(y, j, &color);
        }
    }
}

pub fn draw() {
    let mut image = TargaImage::new(1024, 1024);
    let mut model = Model::load("model.obj");

    for f in &model.faces {
        triangle_bc(
            &mut image,
            Vec3 {
                x: (model.vertices[f.x as usize].x * 1023.0),
                y: (model.vertices[f.x as usize].y * 1023.0),
                z: (model.vertices[f.x as usize].z * 1023.0),
            },
            Vec3 {
                x: (model.vertices[f.y as usize].x * 1023.0),
                y: (model.vertices[f.y as usize].y * 1023.0),
                z: (model.vertices[f.y as usize].z * 1023.0),
            },
            Vec3 {
                x: (model.vertices[f.z as usize].x * 1023.0),
                y: (model.vertices[f.z as usize].y * 1023.0),
                z: (model.vertices[f.z as usize].z * 1023.0),
            },
        );
        // println!("------------");
    }
    // triangle(
    //     &mut image,
    //     Vec2 { x: 201, y: 201 },
    //     Vec2 { x: 700, y: 400 },
    //     Vec2 { x: 500, y: 700 },
    // );

    // let f = Vec3::newi(1, 2, 3);
    // let s = Vec3::new(4.0, 5.0, 6.0);
    // let bc_ = barycenteric(
    //     &[
    //         Vec2 { x: 201, y: 201 },
    //         Vec2 { x: 700, y: 400 },
    //         Vec2 { x: 500, y: 700 },
    //     ],
    //     Vec2 { x: 201, y: 201 },
    // );
    // println!("{:?} {:?}", f.cross(&s), bc_);

    // triangle_bc(
    //     &mut image,
    //     Vec2 { x: 201, y: 201 },
    //     Vec2 { x: 700, y: 400 },
    //     Vec2 { x: 500, y: 700 },
    // );

    let value = Vec3::newi(1, 3, -5);
    let value2 = Vec3::newi(4, -2, -1);

    // value.normalize();
    println!("{:?} {:?}", value, value.dot(&value2));

    let _ = image.save_file("trinangles.tga");
}
