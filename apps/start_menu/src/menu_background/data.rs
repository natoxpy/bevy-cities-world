use bevy::prelude::Vec3;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum PointType {
    Grass,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub fn to_vec3(&self) -> Vec3 {
        Vec3::new(self.x, 0., self.y)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Point {
    pub has: PointType,
    pub position: Position,
    pub content: Vec<ContentPoint>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ContentPointType {
    Tree1,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ContentPoint {
    pub has: ContentPointType,
    pub position: Position,
}
