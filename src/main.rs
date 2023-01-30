mod components;
mod plugins;
mod resources;
mod systems;

use bevy::prelude::*;
use crate::plugins::main::Main;

fn main() {
    App::new()
        .add_startup_system(systems::hello::hello)
        .add_plugins(DefaultPlugins)
        .add_plugin(Main)
        .run()
}
