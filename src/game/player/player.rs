use core::f32;

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier3d::prelude::*;

use super::camera_controller;
use crate::game::{
    level::targets::{DeadTarged, Target},
    shooting::tracer,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(tracer::TracerPlugin);
        app.add_systems(Startup, init_player);
        app.add_systems(
            Update,
            (camera_controller::update_camera_controller, update_player),
        );
    }
}

#[derive(Component)]
pub struct Player;

fn init_player(mut commands: Commands) {
    let fov: f32 = 103.0_f32.to_radians();

    // Spawn the player camera
    commands.spawn((
        Player,
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0., 10., 0.)),
            projection: Projection::Perspective(PerspectiveProjection {
                fov,
                ..Default::default()
            }),
            ..Default::default()
        },
        camera_controller::CameraController {
            sensitivity: 0.05,
            rotation: Vec2::ZERO,
            rotation_lock: 88.0,
        },
    ));
}

fn update_player(
    mut commands: Commands,
    mut player_query: Query<(
        &mut Player,
        &mut Transform,
        &mut GlobalTransform,
        &mut Camera,
    )>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    rapier_context: Res<RapierContext>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    target_query: Query<Entity, With<Target>>,
) {
    let window: &Window = window_query.get_single().unwrap();

    if let Ok((_player, transform, global_transform, camera)) = player_query.get_single_mut() {
        if mouse_input.just_pressed(MouseButton::Left) {
            let Some(ray) = camera.viewport_to_world(
                &global_transform,
                Vec2::new(window.width() / 2., window.height() / 2.),
            ) else {
                return;
            };

            let hit: Option<(Entity, RayIntersection)> = rapier_context.cast_ray_and_get_normal(
                ray.origin,
                ray.direction.into(),
                f32::MAX,
                true,
                QueryFilter::default(),
            );

            if let Some((entity, ray_intersection)) = hit {
                if let Ok(_entity) = target_query.get(entity) {
                    commands.entity(entity).insert(DeadTarged);
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
