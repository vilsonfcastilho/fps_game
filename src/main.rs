pub mod game;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((game::game::GamePlugin, DefaultPlugins))
        .run();
}
