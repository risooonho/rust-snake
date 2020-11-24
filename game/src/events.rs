pub enum Event {
    Noop,
    SnakeEatFood { entity: hecs::Entity, pos: glam::Vec2 },
}
