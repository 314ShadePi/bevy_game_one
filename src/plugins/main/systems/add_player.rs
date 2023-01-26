use crate::plugins::main::components::bundles::player::PlayerBundle;
use crate::plugins::main::components::named::Named;
use crate::plugins::main::components::player_id::PlayerId;
use bevy::prelude::*;

pub fn add_player(mut commands: Commands) {
    commands.spawn(PlayerBundle {
        id: PlayerId(16),
        name: Named("Shay".to_string()),
    });
}
