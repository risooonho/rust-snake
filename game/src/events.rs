#[derive(Debug, Clone, Copy)]
pub enum Event {
    Noop,
    SnakeEatFood { entity: hecs::Entity, pos: glam::Vec2 },
    SpawnSnakeTail { ahead: hecs::Entity, pos: glam::Vec2 },
}
