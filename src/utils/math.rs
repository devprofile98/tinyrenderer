use std::{
    convert::From,
    fs::File,
    io::{BufRead, BufReader},
    ops,
};

use std::mem::swap;
use tgars::{Color, TargaImage};

#[derive(Debug, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn newi(x: i32, y: i32, z: i32) -> Vec3 {
        Vec3 {
            x: x as f32,
            y: y as f32,
            z: z as f32,
        }
    }

    pub fn cross(&self, rhs: &Vec3) -> Vec3 {
        Vec3 {
            x: (self.y * rhs.z) - (rhs.y * self.z),
            y: -((self.x * rhs.z) - (rhs.x * self.z)),
            z: (self.x * rhs.y) - (rhs.x * self.y),
        }
    }

    pub fn dot(&self, rhs: &Vec3) -> f32 {
        (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z)
    }

    pub fn normalize(&mut self) {
        let magnitude = (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt();
        self.x = self.x / magnitude;
        self.y = self.y / magnitude;
        self.z = self.z / magnitude;
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: (self.x - rhs.x),
            y: (self.y - rhs.y),
            z: (self.z - rhs.z),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vec2 {
    pub x: u16,
    pub y: u16,
}

impl ops::Add for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: (self.x as u32 + rhs.x as u32) as u16,
            y: (self.y as u32 + rhs.y as u32) as u16,
        }
    }
}

impl ops::Add<f32> for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: f32) -> Self::Output {
        Vec2 {
            x: (self.x as u32 + rhs as u32) as u16,
            y: (self.y as u32 + rhs as u32) as u16,
        }
    }
}

impl ops::Mul<f32> for Vec2 {
    type Output = Vec2;
    fn mul(self, rhs: f32) -> Self::Output {
        Vec2 {
            x: (self.x as f32 * rhs as f32) as u16,
            y: (self.y as f32 * rhs as f32) as u16,
        }
    }
}

impl ops::Sub for Vec2 {
    type Output = Vec2;
    fn sub(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: (self.x as i32 - rhs.x as i32) as u16,
            y: (self.y as i32 - rhs.y as i32) as u16,
        }
    }
}

impl From<&Vec3> for Vec2 {
    fn from(value: &Vec3) -> Self {
        Vec2 {
            x: value.x as u16,
            y: value.y as u16,
        }
    }
}
