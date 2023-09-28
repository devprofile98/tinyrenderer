use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use super::math::Vec3;

pub struct Model {
    pub vertices: Vec<Vec3>,
    pub faces: Vec<Vec3>,
}

impl Model {
    pub fn new(vertices: Vec<Vec3>, faces: Vec<Vec3>) -> Model {
        Model { vertices, faces }
    }

    pub fn load(path: &str) -> Model {
        match File::open(path) {
            Ok(file) => {
                let mut line = String::new();
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
}
