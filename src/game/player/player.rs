use core::f32;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::{camera_controller, input::*, player_movement, player_shooting};

use crate::game::shooting::tracer;

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

fn init_player(mut commands: Commands) {
    let fov: f32 = 103.0_f32.to_radians();

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

    let player_entity: Entity = commands
        .spawn((
            Player {
                velocity: Vec3::ZERO,
                gravity: 9.8,
                speed: 20.,
            },
            SpatialBundle {
                transform: Transform::from_translation(Vec3::new(0., 10., 0.)),
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

    commands.entity(player_entity).add_child(camera_entity);
}
