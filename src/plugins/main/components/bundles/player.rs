use crate::plugins::main::components::named::Named;
use crate::plugins::main::components::player_id::PlayerId;
use bevy::prelude::*;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub id: PlayerId,
    pub name: Named,
}
