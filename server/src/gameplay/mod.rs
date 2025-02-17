use bevy::prelude::*;
use handle_players::HandlePlayersPlugin;
use shared::networking::lobby_management::MyLobby;
use system_sets::MyGameplaySet;
use tick_systems::TickSystemsPlugin;

pub mod game_state_handling;
pub mod handle_players;
pub mod simulation;
pub mod start_lobby;
pub mod system_sets;
mod tick_systems;
pub mod triggers;

pub struct MyGameplayPlugin;

impl Plugin for MyGameplayPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                (MyGameplaySet::TickTimerProcessing,),
                (
                    MyGameplaySet::IncrementTick,
                    MyGameplaySet::RunSimulationStep,
                    MyGameplaySet::SimulationStepDone,
                    MyGameplaySet::UpdatingGameStates,
                )
                    .chain(),
            )
                .chain(),
        )
        .add_plugins((TickSystemsPlugin, HandlePlayersPlugin))
        .add_systems(
            Update,
            game_state_handling::check_if_client_states_are_all_up_to_date
                .in_set(MyGameplaySet::UpdatingGameStates),
        )
        .add_observer(add_observers_to_lobby);
    }
}

fn add_observers_to_lobby(trigger: Trigger<OnAdd, MyLobby>, mut commands: Commands) {
    commands
        .entity(trigger.entity())
        .observe(game_state_handling::add_current_game_state_to_message_queue)
        .observe(game_state_handling::update_lobby_state)
        .observe(simulation::process_tick_sim)
        .observe(simulation::move_dummies)
        .observe(start_lobby::check_if_lobby_should_start)
        .observe(start_lobby::start_lobby);
}
