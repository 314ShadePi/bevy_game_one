use bevy::prelude::*;
use crate::plugins::main::components::bundles::player::PlayerBundle;
use crate::plugins::main::components::player_id::PlayerId;

pub fn add_player(mut commands: Commands) {
    commands.spawn(PlayerBundle { id: PlayerId(16) });
}