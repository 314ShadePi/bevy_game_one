mod components;
mod resources;
mod systems;

use crate::plugins::main::resources::hello_player_timer::HelloPlayerTimer;
use bevy::prelude::*;

pub struct Main;

impl Plugin for Main {
    fn build(&self, app: &mut App) {
        app.insert_resource(HelloPlayerTimer(Timer::from_seconds(
            3.0,
            TimerMode::Repeating,
        )))
        .add_startup_system(systems::hello::hello)
        .add_startup_system(systems::add_player::add_players)
        .add_system(systems::hello_players::hello_players);
    }
}
