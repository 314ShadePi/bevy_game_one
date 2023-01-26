mod components;
mod plugins;
mod resources;
mod systems;

use crate::plugins::main::Main;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(Main)
        .add_system(bevy::window::close_on_esc)
        .run()
}
