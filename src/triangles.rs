use std::mem::swap;

use super::utils::{draw_line, load_obj, point};
use tgars::{Color, TargaImage};

fn triangle(image: &mut TargaImage, mut p: point, mut p1: point, mut p2: point) {
    if p.y < p1.y {
        swap(&mut p, &mut p1);
    }
    if p1.y < p2.y {
        swap(&mut p1, &mut p2);
    }
    if p2.y < p.y {
        swap(&mut p2, &mut p);
    }
    // p  to p1
    // p1 to p2
    let all_y_1 = draw_line(image, p.clone(), p1.clone(), &Color::rgb(0, 200, 0));
    let all_y_2 = draw_line(image, p1.clone(), p2.clone(), &Color::rgb(0, 200, 200));
    println!("{:?} \n{:?}", all_y_1, all_y_2);

    let temp = if all_y_2.len() > all_y_1.len() {
        &all_y_1
    } else {
        &all_y_2
    };
    for i in 0..temp.len() - 1 {
        draw_line(
            image,
            all_y_2[i].clone(),
            all_y_1[i].clone(),
            &Color::rgb(i.pow(2) as u8 % 255, 0, 200),
        );
    }
    // p2 to p0
    draw_line(image, p2.clone(), p.clone(), &Color::rgb(200, 200, 0));
}

pub fn draw() {
    let mut image = TargaImage::new(1024, 1024);
    let mut model = load_obj("model.obj");

    for f in &model.faces[0..1] {
        triangle(
            &mut image,
            point {
                x: (model.vertices[f.x as usize].x * 1023.0) as u16,
                y: (model.vertices[f.x as usize].y * 1023.0) as u16,
            },
            point {
                x: (model.vertices[f.y as usize].x * 1023.0) as u16,
                y: (model.vertices[f.y as usize].y * 1023.0) as u16,
            },
            point {
                x: (model.vertices[f.z as usize].x * 1023.0) as u16,
                y: (model.vertices[f.z as usize].y * 1023.0) as u16,
            },
        );
    }

    image.save_file("trinangles.tga");
}
