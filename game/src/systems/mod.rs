use std::collections::HashMap;

use glam::Mat4;
use glam::Quat;
use glam::Vec2;
use glam::Vec3;
use miniquad::Context;
use quad_rand as qrand;
use smallvec::SmallVec;

use crate::assets::AssetType;
use crate::components;
use crate::events::Event;
use crate::GameWorld;

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

pub fn tail_movement_system(game_world: &mut GameWorld) {
    let GameWorld { world, .. } = game_world;
    let foo: HashMap<hecs::Entity, glam::Vec2> = world
        .query::<&components::Tail>()
        .iter()
        .map(|(_, tail)| {
            let pos = {
                world
                    .get::<components::Position>(tail.ahead)
                    .expect("All Ahead should has Positions")
                    .0
            };
            (tail.ahead.clone(), pos)
        })
        .collect();
    for (_, (tail, position)) in
        &mut world.query::<(&components::Tail, &mut components::Position)>()
    {
        let new_pos = foo[&tail.ahead];
        position.0 = new_pos;
    }
}

pub fn food_eating_system(game_world: &mut GameWorld) {
    let GameWorld { world, events, .. } = game_world;
    let snake_pos = match world
        .query::<(
            &components::Snake,
            &components::Position,
            &components::Velocity,
        )>()
        .iter()
        .map(|(_, (_, pos, vel))| pos.0 + vel.0)
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
                Some((ent, snake_pos))
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
    for event in events {
        match event {
            Event::SnakeEatFood { entity, pos: _pos } => {
                world
                    .despawn(*entity)
                    .expect("Food Eating System should not be destroying a non-existant Entity");
            }
            _ => {}
        }
    }
}

pub fn trigger_tail_spawn(game_world: &mut GameWorld) {
    let GameWorld { world, events, .. } = game_world;
    let mut events_to_push: SmallVec<[Event; 4]> = SmallVec::new();
    for event in events.iter() {
        match event {
            Event::SnakeEatFood { .. } => {
                if let Some((ahead, (tail, pos))) = &world
                    .query::<(&components::Tail, &components::Position)>()
                    .iter()
                    .max_by_key(|(_, (tail, _))| tail.segment)
                {
                    events_to_push.push(Event::SpawnSnakeTail {
                        ahead: ahead.clone(),
                        pos: pos.0,
                        segment: tail.segment + 1,
                    })
                }
            }
            _ => {}
        }
    }
    events_to_push
        .iter()
        .for_each(|evt| events.push(evt.clone()));
}

pub fn spawn_tail_system(game_world: &mut GameWorld) {
    let GameWorld { world, events, .. } = game_world;
    for event in events.iter() {
        match event {
            Event::SpawnSnakeTail {
                ahead,
                pos,
                segment,
            } => {
                let tail = components::Tail {
                    segment: segment.clone(),
                    ahead: ahead.clone(),
                };

                world.spawn((tail, components::Position(pos.clone())));
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

pub fn head_collision_system(game_world: &mut GameWorld) {
    let GameWorld { world, events, .. } = game_world;
    let (source_ent, source_pos): (hecs::Entity, Vec2) = match world
        .query::<(
            &components::Snake,
            &components::Position,
            &components::Velocity,
        )>()
        .iter()
        .map(|(ent, (_, pos, vel))| (ent, pos.0 + vel.0))
        .nth(0)
    {
        Some(it) => it,
        _ => return,
    };
    world
        .query::<(&components::Position, &components::Tail)>()
        .iter()
        .filter_map(|(ent, (target_pos, _tail))| {
            if target_pos.0 == source_pos {
                Some(Event::Collision {
                    target: ent,
                    source: source_ent,
                    pos: target_pos.0,
                })
            } else {
                None
            }
        })
        .for_each(|event| events.push(event));
}

pub fn handle_collision_system(game_world: &mut GameWorld) {
    let (collsions, rest): (SmallVec<[Event; 32]>, SmallVec<[Event; 32]>) = game_world.events.iter().cloned().partition(|event| {
        match event {
            Event::Collision { .. } => true,
            _ => false
        }
    });
    game_world.events = rest;
    collsions.iter().for_each(|_| game_world.events.push(Event::GameOver));
}

pub fn game_over_system(game_world: &mut GameWorld) {
    let GameWorld { world, events, .. } = game_world;

    let filter = events.iter().filter(|event| match event {
        Event::GameOver => true,
        _ => false,
    }).nth(0);
    if let Some(_) = filter {
        world.clear();
    }

}