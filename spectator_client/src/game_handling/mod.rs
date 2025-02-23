use bevy::prelude::*;
use entity_mapping::MyEntityMapping;

use crate::networking::MyNetworkStream;

pub mod entity_mapping;
pub mod game_starts;
pub mod player_handling;
pub mod projectile_handling;

pub struct MyGameHandlingPlugin;

impl Plugin for MyGameHandlingPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<MyEntityMapping>()
            .add_observer(add_observers);
    }
}

fn add_observers(trigger: Trigger<OnAdd, MyNetworkStream>, mut commands: Commands) {
    commands
        .entity(trigger.entity())
        .observe(game_starts::game_starts)
        .observe(player_handling::move_players_on_game_state_update)
        .observe(projectile_handling::handle_projectile_on_game_state_update);
}
