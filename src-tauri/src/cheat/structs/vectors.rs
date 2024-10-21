use serde::Serialize;


#[derive(Debug, Clone, Copy)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
