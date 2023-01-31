use crate::plugins::main::components::named::Named;
use crate::plugins::main::components::player_id::PlayerId;
use crate::plugins::main::resources::hello_player_timer::HelloPlayerTimer;
use bevy::prelude::*;

pub fn hello_players(
    time: Res<Time>,
    mut timer: ResMut<HelloPlayerTimer>,
    query: Query<(&Named, &PlayerId)>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for player in query.iter() {
            println!(
                "Hello player: \"{}\" with id: {}!",
                player.0 .0, player.1 .0
            );
        }
    }
}
