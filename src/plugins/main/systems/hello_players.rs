use crate::plugins::main::components::named::Named;
use crate::plugins::main::components::player_id::PlayerId;
use crate::plugins::main::resources::hello_player_timer::HelloPlayerTimer;
use bevy::prelude::*;

pub fn hello_players(
    time: Res<Time>,
    mut timer: ResMut<HelloPlayerTimer>,
    query: Query<&Named, With<PlayerId>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in query.iter() {
            println!("Hello {}!", name.0);
        }
    }
}
