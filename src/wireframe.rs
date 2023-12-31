use crate::utils::{draw_line, math::Vec2, model::Model};
use tgars::{Color, TargaImage};

pub fn draw(model_path: &str) {
    let green: Color = Color::rgb(0, 255, 0);
    let red: Color = Color::rgb(255, 0, 0);
    let blue: Color = Color::rgb(0, 0, 255);
    let mut image = TargaImage::new(1024, 1024);

    let model = Model::load(model_path);
    for f in &model.faces.polys {
        draw_line(
            &mut image,
            Vec2 {
                x: ((model.vertices[f.y as usize].x) * 1023.0) as u16,
                y: ((model.vertices[f.y as usize].y) * 1023.0) as u16,
            },
            Vec2 {
                x: ((model.vertices[f.x as usize].x) * 1023.0) as u16,
                y: ((model.vertices[f.x as usize].y) * 1023.0) as u16,
            },
            &red,
        );
        draw_line(
            &mut image,
            Vec2 {
                x: ((model.vertices[f.z as usize].x) * 1023.0) as u16,
                y: ((model.vertices[f.z as usize].y) * 1023.0) as u16,
            },
            Vec2 {
                x: ((model.vertices[f.y as usize].x) * 1023.0) as u16,
                y: ((model.vertices[f.y as usize].y) * 1023.0) as u16,
            },
            &red,
        );
        draw_line(
            &mut image,
            Vec2 {
                x: ((model.vertices[f.x as usize].x) * 1023.0) as u16,
                y: ((model.vertices[f.x as usize].y) * 1023.0) as u16,
            },
            Vec2 {
                x: ((model.vertices[f.z as usize].x) * 1023.0) as u16,
                y: ((model.vertices[f.z as usize].y) * 1023.0) as u16,
            },
            &red,
        );
    }

    if let Err(e) = image.save_file("lines.tga") {
        println!("Failed to save the image with {:}", e);
    }

    println!("last verex {:?}", model.vertices.last());
    println!("{}", ("-0.999999".parse::<f32>().unwrap() + 1.0) / 2.0);
}
