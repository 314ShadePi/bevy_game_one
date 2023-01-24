mod components;
mod entities;
mod plugins;
mod resources;
mod systems;

use crate::plugins::example_plugin::ExamplePlugin;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ExamplePlugin)
        .run()
}
