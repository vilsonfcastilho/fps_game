use core::f32;

use bevy::ecs::system::*;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::{
    camera_controller,
    input::*,
    player_movement,
    player_shooting::{self, TracerSpawnSpot},
};

use crate::game::{math::coordinates::blender_to_world, shooting::tracer};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(tracer::TracerPlugin)
            .init_resource::<PlayerInput>()
            .add_systems(Startup, init_player)
            .add_systems(
                Update,
                (
                    camera_controller::update_camera_controller,
                    player_shooting::update_player,
                    player_movement::update_movement_input,
                ),
            )
            .add_systems(FixedUpdate, player_movement::update_movement); // physics timestamp
    }
}

#[derive(Component)]
pub struct Player {
    pub velocity: Vec3,
    pub gravity: f32,
    pub speed: f32,
}

fn init_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera
    // let fov: f32 = 103.0_f32.to_radians();
    let fov: f32 = 75.0_f32.to_radians();
    let camera_entity: Entity = commands
        .spawn((
            Camera3dBundle {
                transform: Transform::IDENTITY,
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
        ))
        .id();

    // Camera - Gun
    let gun_model: Handle<Scene> = asset_server.load("models/ak.glb#Scene0");
    let gun_entity: Entity = commands
        .spawn(SceneBundle {
            scene: gun_model,
            transform: Transform::IDENTITY,
            ..Default::default()
        })
        .id();

    // Camera - Tracer Spawn Spot
    let spawn_spot: Vec3 = blender_to_world(Vec3::new(0.530462, 2.10557, -0.466568));
    let tracer_spawn_entity: Entity = commands
        .spawn((
            TransformBundle {
                local: Transform::from_translation(spawn_spot),
                ..Default::default()
            },
            TracerSpawnSpot,
        ))
        .id();

    // Player
    let player_entity: Entity = commands
        .spawn((
            Player {
                velocity: Vec3::ZERO,
                gravity: 9.8,
                speed: 20.,
            },
            SpatialBundle {
                transform: Transform::from_translation(Vec3::new(0., 30., 0.)),
                ..Default::default()
            },
            Collider::cuboid(1., 10., 1.),
            RigidBody::KinematicPositionBased,
            KinematicCharacterController {
                up: Vec3::Y,
                offset: CharacterLength::Absolute(0.01),
                ..Default::default()
            },
        ))
        .id();

    commands
        .entity(camera_entity)
        .push_children(&[gun_entity, tracer_spawn_entity]);
    commands.entity(player_entity).add_child(camera_entity);
}
