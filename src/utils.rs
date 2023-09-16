use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter},
};

use std::mem::swap;
use tgars::{Color, TargaImage};

#[derive(Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug)]
pub struct point {
    pub x: u16,
    pub y: u16,
}

pub fn draw_line(image: &mut TargaImage, mut p1: point, mut p2: point, color: &Color) {
    let mut steep = false;
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
        // println!("the point is : [{},{}]", x,y);
        if steep {
            image.set_pixel(x, y, color);
        } else {
            image.set_pixel(y, x, color);
        }
    }
}

pub struct Model {
    pub vertices: Vec<Vec3>,
    pub faces: Vec<Vec3>,
}

impl Model {
    pub fn new(vertices: Vec<Vec3>, faces: Vec<Vec3>) -> Model {
        Model { vertices, faces }
    }
}

pub fn load_obj(path: &str) -> Model {
    match File::open(path) {
        Ok(file) => {
            let mut line: String = String::new();
            let mut buf = BufReader::new(file);
            let mut vertices = Vec::new();
            let mut faces = Vec::new();
            let _t = buf.read_line(&mut line);
            for l in buf.lines() {
                if let Ok(line) = l {
                    let parsed_line = line.split(' ').collect::<Vec<&str>>();
                    if parsed_line[0] == "v" {
                        vertices.push(Vec3 {
                            x: (parsed_line[1].parse::<f32>().unwrap() + 1.0) / 2.0,
                            y: (parsed_line[2].parse::<f32>().unwrap() + 1.0) / 2.0,
                            z: (parsed_line[3].parse::<f32>().unwrap() + 1.0) / 2.0,
                        });
                    } else if parsed_line[0] == "f" {
                        faces.push(Vec3 {
                            x: parsed_line[1]
                                .split_once('/')
                                .unwrap()
                                .0
                                .parse::<f32>()
                                .unwrap()
                                - 1.0,
                            y: parsed_line[2]
                                .split_once('/')
                                .unwrap()
                                .0
                                .parse::<f32>()
                                .unwrap()
                                - 1.0,
                            z: parsed_line[3]
                                .split_once('/')
                                .unwrap()
                                .0
                                .parse::<f32>()
                                .unwrap()
                                - 1.0,
                        });
                    }
                }
            }
            println!("{} {}", vertices.len(), faces.len(),);
            return Model::new(vertices, faces);
        }
        Err(_e) => {
            println!("Failed to load the models with error: {}", _e);
            return Model::new(
                vec![Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                }],
                vec![Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                }],
            );
        }
    };
}
