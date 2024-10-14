use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::game::player::player_shooting::Shootable;

use super::targets;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(targets::TargetPlugin);
        app.add_systems(Startup, init_level);
    }
}

fn init_level(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let level_material: Handle<StandardMaterial> = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        ..Default::default()
    });

    // Spawn the floor
    commands.spawn((
        Collider::cuboid(1000., 0., 1000.),
        PbrBundle {
            material: level_material.clone(),
            transform: Transform::IDENTITY,
            mesh: meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(1000.))),
            ..Default::default()
        },
        Shootable,
    ));

    // Spawn the wall
    commands.spawn((
        Collider::cuboid(30., 30., 30.),
        PbrBundle {
            material: level_material.clone(),
            transform: Transform::from_xyz(0., 0., -70.), // -100
            mesh: meshes.add(Cuboid::from_length(60.)),
            ..Default::default()
        },
        Shootable,
    ));

    // Spanw the light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(100., 200., 100.).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}
