use bevy::prelude::*;
use bevy_rapier3d::plugin::{NoUserData, RapierPhysicsPlugin};

use super::{level::*, player::*, ui::*, window::*};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            RapierPhysicsPlugin::<NoUserData>::default(),
            window::WindowSettingsPlugin,
            level::LevelPlugin,
            player::PlayerPlugin,
            ui::UiPlugin,
        ));
    }
}
