use core::f32;

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier3d::prelude::*;

use super::{camera_controller::*, player::*};

use crate::game::{
    level::targets::{DeadTarged, Target},
    shooting::tracer,
};

#[derive(Component)]
pub struct Shootable;

pub fn update_player(
    mut commands: Commands,
    mut player_query: Query<(&mut Player, &mut Transform)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<CameraController>>,
    rapier_context: Res<RapierContext>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    target_query: Query<Option<&Target>, With<Shootable>>,
) {
    let window: &Window = window_query.get_single().unwrap();
    let (camera, camera_global_transform) = camera_query.get_single().unwrap();

    if let Ok((_player, transform)) = player_query.get_single_mut() {
        if mouse_input.just_pressed(MouseButton::Left) {
            let Some(ray) = camera.viewport_to_world(
                &camera_global_transform,
                Vec2::new(window.width() / 2., window.height() / 2.),
            ) else {
                return;
            };

            let predicate = |handle| target_query.get(handle).is_ok();
            let query_filter: QueryFilter<'_> = QueryFilter::new().predicate(&predicate);

            let hit: Option<(Entity, RayIntersection)> = rapier_context.cast_ray_and_get_normal(
                ray.origin,
                ray.direction.into(),
                f32::MAX,
                true,
                query_filter,
            );

            if let Some((entity, ray_intersection)) = hit {
                if let Ok(target) = target_query.get(entity) {
                    if target.is_some() {
                        commands.entity(entity).insert(DeadTarged);
                    }
                }

                // Spawn tracer and check collisions
                let tracer_material: StandardMaterial = StandardMaterial {
                    base_color: Color::srgb(1., 1., 0.),
                    unlit: true,
                    ..Default::default()
                };

                commands.spawn((
                    PbrBundle {
                        transform: Transform::from_translation(Vec3::splat(f32::MAX)),
                        mesh: meshes.add(Cuboid::from_size(Vec3::new(0.1, 0.1, 0.1))),
                        material: materials.add(tracer_material),
                        ..Default::default()
                    },
                    tracer::BulletTracer::new(transform.translation, ray_intersection.point, 100.),
                ));
            }
        }
    }
}
