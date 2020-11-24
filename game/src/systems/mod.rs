use glam::Mat4;
use glam::Quat;
use glam::Vec2;
use glam::Vec3;
use miniquad::Context;
use quad_rand as qrand;

use crate::assets::AssetType;
use crate::components;
use crate::GameWorld;
use crate::events::Event;

pub fn add_food_system(game_world: &mut GameWorld) {
    let GameWorld { world, .. } = game_world;
    let snake_count = world.query::<&components::Food>().iter().count();
    if snake_count >= 10 {
        return;
    }

    let x = qrand::gen_range(-24, 24);
    let y = qrand::gen_range(-15, 15);
    let pos = components::Position(Vec2::new(x as f32, y as f32));
    world.spawn((pos, components::Food));
}

pub fn update_head_direction(game_world: &mut GameWorld, direction: Vec2) {
    let GameWorld { world, .. } = game_world;
    for (_, (velocity, _)) in &mut world.query::<(&mut components::Velocity, &components::Snake)>()
    {
        velocity.0 = direction;
    }
}

pub fn movement_system(game_world: &mut GameWorld) {
    let GameWorld { world, .. } = game_world;
    for (_, (pos, velocity)) in
        &mut world.query::<(&mut components::Position, &components::Velocity)>()
    {
        pos.0 = pos.0 + velocity.0;
    }
}

pub fn food_eating_system(game_world: &mut GameWorld) {
    let GameWorld { world, events, .. } = game_world;
    let snake_pos = match world
        .query::<(&components::Snake, &components::Position)>()
        .iter()
        .map(|(_, (_, pos))| pos.0)
        .nth(0)
    {
        Some(it) => it,
        _ => return,
    };
    world
        .query::<(&components::Food, &components::Position)>()
        .iter()
        .filter_map(|(ent, (_food, pos))| {
            if pos.0 == snake_pos {
                Some((ent, pos.0))
            } else {
                None
            }
        })
        .for_each(|(entity, pos)| {
            events.push(Event::SnakeEatFood { entity, pos });
        });

}

pub fn despawn_food_system(game_world: &mut GameWorld) {
    let GameWorld { world, events, .. } = game_world;
    for event in events.iter() {
        match event {
            Event::SnakeEatFood { entity, pos: _pos } => {
                world.despawn(*entity).expect("Food Eating System should not be destroying a non-existant Entity");

            }
            _ => {}
        }
    }
}

pub fn render_snake_system(game_world: &mut GameWorld, ctx: &mut Context) {
    let GameWorld {
        camera,
        world,
        bindings,
        ..
    } = game_world;
    let mut uniform = camera.uniform();
    if let Some(binding) = bindings.get(&AssetType::Snake) {
        for (_, (_food, pos)) in &mut world.query::<(&components::Snake, &components::Position)>() {
            let model = Mat4::from_rotation_translation(
                Quat::from_axis_angle(Vec3::new(0., 0., 1.), 0.),
                Vec3::new(pos.0.x, pos.0.y, 0.),
            );
            uniform.model = model;
            ctx.apply_bindings(&binding);
            ctx.apply_uniforms(&uniform);
            ctx.draw(0, 6, 1);
        }
    }
}

pub fn render_food_system(game_world: &mut GameWorld, ctx: &mut Context) {
    let GameWorld {
        camera,
        world,
        bindings,
        ..
    } = game_world;
    let mut uniform = camera.uniform();
    if let Some(binding) = bindings.get(&AssetType::Food) {
        for (_, (_food, pos)) in &mut world.query::<(&components::Food, &components::Position)>() {
            let model = Mat4::from_rotation_translation(
                Quat::from_axis_angle(Vec3::new(0., 0., 1.), 0.),
                Vec3::new(pos.0.x, pos.0.y, 0.),
            );
            uniform.model = model;
            ctx.apply_bindings(&binding);
            ctx.apply_uniforms(&uniform);
            ctx.draw(0, 6, 1);
        }
    }
}

pub fn render_tail_system(game_world: &mut GameWorld, ctx: &mut Context) {
    let GameWorld {
        camera,
        world,
        bindings,
        ..
    } = game_world;
    let mut uniform = camera.uniform();
    if let Some(binding) = bindings.get(&AssetType::Tail) {
        for (_, (_food, pos)) in &mut world.query::<(&components::Tail, &components::Position)>() {
            let model = Mat4::from_rotation_translation(
                Quat::from_axis_angle(Vec3::new(0., 0., 1.), 0.),
                Vec3::new(pos.0.x, pos.0.y, 0.),
            );
            uniform.model = model;
            ctx.apply_bindings(&binding);
            ctx.apply_uniforms(&uniform);
            ctx.draw(0, 6, 1);
        }
    }
}
