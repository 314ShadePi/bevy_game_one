use bevy::prelude::*;
use crate::plugins::main::components::player_id::PlayerId;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub id: PlayerId
}