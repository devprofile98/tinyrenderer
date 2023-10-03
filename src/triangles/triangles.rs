// use crate::utils123::Vec3;
use crate::utils::{
    math::{Vec2, Vec3},
    model::Model,
};
use itertools::izip;
use rand::Rng;
use std::{
    cmp::{max, min},
    mem::swap,
    vec,
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

fn triangle_bc(
    image: &mut TargaImage,
    texture: &TargaImage,
    zbuffer: &mut [f32],
    p: Vec3,
    p1: Vec3,
    p2: Vec3,
    dtc: Vec<Vec2>,
) {
    let light_dir = Vec3::newi(0, 0, -1); // define light_dir
    let mut bboxmin = Vec2 { x: 1023, y: 1023 };
    let mut bboxmax = Vec2 { x: 0, y: 0 };

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
            let bc_texture = barycenteric(&[dtc[0], dtc[1], dtc[2]], Vec2 { x, y });
            // let color = Color::rgb(
            //     (255.0 * bc.x) as u8,
            //     (255.0 * bc.y) as u8,
            //     (255.0 * bc.z) as u8,
            // );
            // let mut color = texture[dtc[0].y as usize * 1024 + dtc[0].x as usize].clone();
            // color = color * bc.x;
            // color = color + (texture[dtc[1].y as usize * 1024 + dtc[1].x as usize].clone() * bc.y);
            // color = color + (texture[dtc[2].y as usize * 1024 + dtc[2].x as usize].clone() * bc.z);
            let color = texture[(bc_texture.x * 1024.0 + bc_texture.y) as usize].clone();
            // + (texture[dtc[2].y as usize * 1024 + dtc[2].x as usize].clone() * bc.z);

            // println!("{} {}", dtc[0].x, dtc[0].y);
            if bc.x < 0.0 || bc.y < 0.0 || bc.z < 0.0 {
                continue;
            }
            let mut face_normal = (p2.clone() - p.clone()).cross(&(p1.clone() - p.clone()));
            face_normal.normalize();
            let intensity = face_normal.dot(&light_dir);
            if intensity > 0.0 {
                let z = p.z * bc.x + p1.z * bc.y + p2.z * bc.z;
                if z > zbuffer[(y as usize * 1024 + x as usize) as usize] {
                    zbuffer[(y as usize * 1024 + x as usize) as usize] = z;
                    image.set_pixel(
                        y,
                        x,
                        // &Color::rgb(
                        //     (intensity * 255.0) as u8,
                        //     (intensity * 255.0) as u8,
                        //     (intensity * 255.0) as u8,
                        // ),
                        &(color),
                    );
                }
            }
        }
    }
    image.set_pixel(
        p.y as u16,
        p.x as u16,
        &texture[dtc[0].y as usize * 1024 + dtc[0].x as usize],
    );
    println!("{:?}", p1);
    image.set_pixel(
        p1.y as u16,
        p1.x as u16,
        &texture[dtc[1].y as usize * 1024 + dtc[1].x as usize],
    );
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
    let diffuse_texture = TargaImage::load("african_head_diffuse.tga").unwrap();
    // image.fill(&Color::rgb(0, 0, 210));
    let mut model = Model::load("model.obj");
    let mut z_buffer = [std::f32::MIN; 1024 * 1024];

    // for i in &model.texture {
    //     println!("{:?}", i.clone() * 1024);
    // }
    for (f, t) in izip!(&model.faces.polys, &model.faces.texs) {
        triangle_bc(
            &mut image,
            &diffuse_texture,
            &mut z_buffer,
            model.vertices[f.x as usize].clone() * 1023,
            model.vertices[f.y as usize].clone() * 1023,
            model.vertices[f.z as usize].clone() * 1023,
            vec![
                Vec2::from(&(model.texture[t.x as usize].clone() * 1023)),
                Vec2::from(&(model.texture[t.y as usize].clone() * 1023)),
                Vec2::from(&(model.texture[t.z as usize].clone() * 1023)),
            ],
        );
    }

    // triangle_bc(
    //     &mut image,
    //     &mut z_buffer,
    //     Vec3::newi(123, 240, 100),
    //     Vec3::newi(272, 400, 0),
    //     // Vec3::newi(600, 1000, 290),
    //     // Vec3::newi(201, 201, 0),
    //     // Vec3::newi(800, 400, 0),
    //     Vec3::newi(500, 800, 0),
    // );

    println!("textures are {}", model.texture.len());

    let _ = image.save_file("trinangles.tga");
}
