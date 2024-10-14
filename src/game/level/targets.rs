use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use rand::*;
use rngs::ThreadRng;
use std::f32::consts::PI;

use crate::game::player::player_shooting::Shootable;

pub struct TargetPlugin;

impl Plugin for TargetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_grid_shot);
        app.add_systems(Update, update_targets);
    }
}

#[derive(Component)]
pub struct Target;

#[derive(Component)]
pub struct DeadTarged;

#[derive(Resource, Clone, Copy)]
pub struct GridShot {
    pub grid_size: i32,
    pub cell_size: f32,
    pub max_targets: i32,
}

impl GridShot {
    pub fn generate_new_position(&self, rand: &mut ThreadRng) -> Vec2 {
        (Vec2::new(
            rand.gen_range(0..self.grid_size) as f32,
            rand.gen_range(0..self.grid_size) as f32,
        ) - Vec2::new(self.grid_size as f32 / 2., 0.)
            + (Vec2::Y * 0.5))
            * self.cell_size
    }
}

fn init_grid_shot(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let grid_shot: GridShot = GridShot {
        grid_size: 5,
        cell_size: 5.,
        max_targets: 5,
    };

    let target_material: Handle<StandardMaterial> = materials.add(StandardMaterial {
        base_color: Color::srgb(1., 0., 0.),
        ..Default::default()
    });

    commands.insert_resource(grid_shot);

    let target_radius: f32 = grid_shot.cell_size / 8.;
    let collider_radius: f32 = target_radius * f32::sin(PI / 4.);

    for _i in 0..grid_shot.max_targets {
        commands.spawn((
            Collider::cuboid(collider_radius, collider_radius, collider_radius),
            PbrBundle {
                transform: Transform::from_xyz(0., 0., -30.), // -40
                mesh: meshes.add(Sphere::new(target_radius)),
                material: target_material.clone(),
                ..Default::default()
            },
            Target,
            DeadTarged,
            Shootable,
        ));
    }
}

fn update_targets(
    mut commands: Commands,
    mut dead_targets: Query<(Entity, &mut Transform), (With<DeadTarged>, With<Target>)>,
    grid_shot: Res<GridShot>,
    alive_targets: Query<&Transform, (With<Target>, Without<DeadTarged>)>,
) {
    let mut alive_target_position: Vec<Vec2> = Vec::new();

    let mut rand: ThreadRng = thread_rng();

    for transform in alive_targets.iter() {
        alive_target_position.push(transform.translation.xy());
    }

    for (entity, mut transform) in dead_targets.iter_mut() {
        let mut found_spot: bool = false;
        let old_position: Vec2 = transform.translation.xy();
        let mut new_position: Vec2 = grid_shot.generate_new_position(&mut rand);

        while !found_spot {
            found_spot = true;

            while new_position == old_position {
                new_position = grid_shot.generate_new_position(&mut rand);
            }

            for position in alive_target_position.iter() {
                if *position == new_position {
                    found_spot = false;
                    new_position = grid_shot.generate_new_position(&mut rand);
                    break;
                }
            }
        }

        commands.entity(entity).remove::<DeadTarged>();

        transform.translation.x = new_position.x;
        transform.translation.y = new_position.y;
        alive_target_position.push(new_position);
    }
}
