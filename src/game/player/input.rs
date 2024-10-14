use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct PlayerInput {
    pub movement: Vec2,
}
