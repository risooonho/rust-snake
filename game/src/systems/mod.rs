use glam::Quat;
use glam::Vec3;
use glam::Mat4;
use miniquad::Context;
use glam::Vec2;
use quad_rand as qrand;


use crate::{AssetType, components};
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

pub fn render_food_system(game_world: &mut GameWorld, ctx: &mut Context) {
    let GameWorld {
        camera,
        world,
        bindings,
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
