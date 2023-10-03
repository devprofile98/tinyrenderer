use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use super::math::Vec3;

pub struct Faces {
    pub polys: Vec<Vec3>,
    pub texs: Vec<Vec3>,
    // pub _dk: Vec<Vec3>,
}

impl Faces {
    pub fn new(poly_mapping: Vec<Vec3>, texture_mapping: Vec<Vec3>) -> Faces {
        Faces {
            polys: poly_mapping,
            texs: texture_mapping,
        }
    }
}

pub struct Model {
    pub vertices: Vec<Vec3>,
    pub faces: Faces,
    pub texture: Vec<Vec3>,
}

impl Model {
    pub fn new(vertices: Vec<Vec3>, faces: Faces, texture: Vec<Vec3>) -> Model {
        Model {
            vertices,
            faces,
            texture,
        }
    }

    pub fn load(path: &str) -> Model {
        match File::open(path) {
            Ok(file) => {
                let mut line = String::new();
                let mut buf = BufReader::new(file);
                let mut vertices = Vec::new();
                let mut faces = Faces::new(Vec::new(), Vec::new());
                let mut texture = Vec::new();
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
                            let len = parsed_line.len();
                            let parsed_mpg: Vec<Vec<f32>> = parsed_line[len - 3..len]
                                .iter()
                                .map(|&f| f.split("/").map(|d| d.parse::<f32>().unwrap()).collect())
                                .collect();
                            faces.polys.push(Vec3::new(
                                parsed_mpg[0][0] - 1.0,
                                parsed_mpg[1][0] - 1.0,
                                parsed_mpg[2][0] - 1.0,
                            ));
                            faces.texs.push(Vec3::new(
                                parsed_mpg[0][1] - 1.0,
                                parsed_mpg[1][1] - 1.0,
                                parsed_mpg[2][1] - 1.0,
                            ));
                            // println!("{:?}", parsed_mpg);
                        } else if parsed_line[0] == "vt" {
                            let len = parsed_line.len();
                            texture.push(Vec3::new(
                                parsed_line[len - 3].parse::<f32>().unwrap(),
                                parsed_line[len - 2].parse::<f32>().unwrap(),
                                0.0,
                            ))
                        }
                    }
                }
                return Model::new(vertices, faces, texture);
            }
            Err(_e) => {
                println!("Failed to load the models with error: {}", _e);
                return Model::new(
                    vec![Vec3 {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                    }],
                    Faces::new(Vec::new(), Vec::new()),
                    vec![Vec3::new(0.0, 0.0, 0.0)],
                );
            }
        };
    }
}
