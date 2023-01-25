mod components;
mod entities;
mod resources;
mod systems;

use bevy::prelude::*;

pub struct Main;

impl Plugin for Main {
    fn build(&self, app: &mut App) {
        app.add_startup_system(systems::hello::hello);
    }
}
