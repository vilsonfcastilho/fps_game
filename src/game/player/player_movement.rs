use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::{camera_controller::*, input::*, player::*};

pub fn update_movement_input(mut input: ResMut<PlayerInput>, keys: Res<ButtonInput<KeyCode>>) {
    input.movement = Vec2::ZERO;

    if keys.pressed(KeyCode::KeyW) {
        input.movement.x += 1.;
    }
    if keys.pressed(KeyCode::KeyA) {
        input.movement.y -= 1.;
    }
    if keys.pressed(KeyCode::KeyS) {
        input.movement.x -= 1.;
    }
    if keys.pressed(KeyCode::KeyD) {
        input.movement.y += 1.;
    }
}

pub fn update_movement(
    mut player_query: Query<(
        &mut Player,
        &mut KinematicCharacterController,
        Option<&KinematicCharacterControllerOutput>,
    )>,
    time: Res<Time<Fixed>>,
    input: Res<PlayerInput>,
    camera_query: Query<&CameraController>,
) {
    let camera: &CameraController = camera_query.get_single().unwrap();

    for (mut player, mut controller, controller_output) in player_query.iter_mut() {
        if let Some(output) = controller_output {
            if output.grounded {
                player.velocity = Vec3::ZERO;
            }
        }
        let camera_rotation_converted: f32 =
            -camera.rotation.y.to_radians() - 90.0_f32.to_radians();

        let forward: Vec2 = Vec2::new(
            f32::cos(camera_rotation_converted),
            f32::sin(camera_rotation_converted),
        );

        let right: Vec2 = Vec2::new(-forward.y, forward.x);

        if let Some(movement_direction) =
            (forward * input.movement.x + right * input.movement.y).try_normalize()
        {
            player.velocity.x = movement_direction.x * player.speed;
            player.velocity.z = movement_direction.y * player.speed;
        }

        player.velocity.y -= player.gravity * time.timestep().as_secs_f32();

        //delta
        controller.translation = Some(player.velocity * time.timestep().as_secs_f32());
    }
}
