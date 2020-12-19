use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

// TODO(jhurstwright): Create an alignment
#[derive(Debug, Clone)]
pub struct Rect {
    positon: glam::Vec2,
    extends: glam::Vec2,
}

impl Rect {
    pub fn new(positon: glam::Vec2, extends: glam::Vec2) -> Self {
        Self {
            positon,
            extends,
        }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct AssetIdentity(pub u64);

impl From<String> for AssetIdentity {
    fn from(v: String) -> Self {
        let mut hasher = DefaultHasher::new();
        v.hash(&mut hasher);
        Self(hasher.finish())
    }
}

impl From<&'_ str> for AssetIdentity {
    fn from(v: &'_ str) -> Self {
        let mut hasher = DefaultHasher::new();
        v.hash(&mut hasher);
        Self(hasher.finish())
    }
}



#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Color([f32; 4]);

pub mod colors {
    #![allow(dead_code)]
    use super::Color;

    pub const LIGHTGRAY: Color = Color::new(0.78, 0.78, 0.78, 1.00);
    pub const GRAY: Color = Color::new(0.51, 0.51, 0.51, 1.00);
    pub const DARKGRAY: Color = Color::new(0.31, 0.31, 0.31, 1.00);
    pub const YELLOW: Color = Color::new(0.99, 0.98, 0.00, 1.00);
    pub const GOLD: Color = Color::new(1.00, 0.80, 0.00, 1.00);
    pub const ORANGE: Color = Color::new(1.00, 0.63, 0.00, 1.00);
    pub const PINK: Color = Color::new(1.00, 0.43, 0.76, 1.00);
    pub const RED: Color = Color::new(0.90, 0.16, 0.22, 1.00);
    pub const MAROON: Color = Color::new(0.75, 0.13, 0.22, 1.00);
    pub const GREEN: Color = Color::new(0.00, 0.89, 0.19, 1.00);
    pub const LIME: Color = Color::new(0.00, 0.62, 0.18, 1.00);
    pub const DARKGREEN: Color = Color::new(0.00, 0.46, 0.17, 1.00);
    pub const SKYBLUE: Color = Color::new(0.40, 0.75, 1.00, 1.00);
    pub const BLUE: Color = Color::new(0.00, 0.47, 0.95, 1.00);
    pub const DARKBLUE: Color = Color::new(0.00, 0.32, 0.67, 1.00);
    pub const PURPLE: Color = Color::new(0.78, 0.48, 1.00, 1.00);
    pub const VIOLET: Color = Color::new(0.53, 0.24, 0.75, 1.00);
    pub const DARKPURPLE: Color = Color::new(0.44, 0.12, 0.49, 1.00);
    pub const BEIGE: Color = Color::new(0.83, 0.69, 0.51, 1.00);
    pub const BROWN: Color = Color::new(0.50, 0.42, 0.31, 1.00);
    pub const DARKBROWN: Color = Color::new(0.30, 0.25, 0.18, 1.00);
    pub const RAYWHITE: Color = Color::new(0.9, 0.9, 0.9, 1.00);
    pub const WHITE: Color = Color::new(1.00, 1.00, 1.00, 1.00);
    pub const BLACK: Color = Color::new(0.00, 0.00, 0.00, 1.00);
    pub const BLANK: Color = Color::new(0.00, 0.00, 0.00, 0.00);
    pub const MAGENTA: Color = Color::new(1.00, 0.00, 1.00, 1.00);
    pub const CLEAR: Color = Color::new(0., 0., 0., 0.);
}

impl Color {
    pub const fn new(r: f32, b: f32, g: f32, a: f32) -> Self {
        Color([r, b, g, a])
    }
    pub fn as_u8(&self) -> [u8; 4] {
        [
            (self.0[0] * 255.0f32).max(0.0).min(255.0) as u8,
            (self.0[1] * 255.0f32).max(0.0).min(255.0) as u8,
            (self.0[2] * 255.0f32).max(0.0).min(255.0) as u8,
            (self.0[3] * 255.0f32).max(0.0).min(255.0) as u8,
        ]
    }
}

impl From<(u8, u8, u8, u8)> for Color {
    fn from(v: (u8, u8, u8, u8)) -> Self {
        Self([
            v.0 as f32 / 255.0,
            v.1 as f32 / 255.0,
            v.2 as f32 / 255.0,
            v.3 as f32 / 255.0,
        ])
    }
}

impl Into<[u8; 4]> for Color {
    fn into(self) -> [u8; 4] {
        self.as_u8()
    }
}

impl Into<(u8, u8, u8, u8)> for Color {
    fn into(self) -> (u8, u8, u8, u8) {
        let x = (self.0[0] * 255.0f32).max(0.0).min(255.0) as u8;
        let y = (self.0[1] * 255.0f32).max(0.0).min(255.0) as u8;
        let z = (self.0[2] * 255.0f32).max(0.0).min(255.0) as u8;
        let w = (self.0[3] * 255.0f32).max(0.0).min(255.0) as u8;
        (x, y, z, w)
    }
}

impl Into<(f32, f32, f32, f32)> for Color {
    fn into(self) -> (f32, f32, f32, f32) {
        (self.0[0], self.0[1], self.0[2], self.0[3])
    }
}