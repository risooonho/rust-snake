#![allow(dead_code)]
use crate::components;

#[derive(Debug, Clone, Copy)]
pub enum Event {
    Noop,
    SnakeEatFood { entity: hecs::Entity, pos: glam::Vec2 },
    SpawnSnakeTail { ahead: hecs::Entity, pos: glam::Vec2, segment: usize },
    Collision {
        target: hecs::Entity,
        source: hecs::Entity,
        pos: glam::Vec2,
        kind: components::CollsionKind,
     },
    GameOver,
}
