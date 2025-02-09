use bevy::prelude::*;
use shared::{
    asset_handling::config::ServerConfigSystemParam,
    networking::{
        lobby_management::{
            lobby_management::LobbyManagementSystemParam, AwaitingFirstContact, InTeam,
        },
        messages::message_container::FirstContactTrigger,
    },
};

use crate::networking::handle_clients::lib::{ClientDisconnectedTrigger, MyNetworkClient};

pub fn handle_awaiting_first_contact(
    mut commands: Commands,
    mut clients: Query<(Entity, &mut AwaitingFirstContact)>,
    time: Res<Time>,
) {
    for (entity, mut timer) in clients.iter_mut() {
        if timer.0.tick(time.delta()).finished() {
            info!("Client {:?} timed out waiting for first contact", entity);
            commands.trigger(ClientDisconnectedTrigger(entity));
        }
    }
}

// Proof of concept for handling a message using an observer
// We can even make targeted ones and only trigger for specific clients!
pub fn handle_first_contact_message(
    trigger: Trigger<FirstContactTrigger>,
    mut commands: Commands,
    mut lobby_management: LobbyManagementSystemParam,
    mut clients: Query<(Entity, &mut MyNetworkClient)>,
    server_config: ServerConfigSystemParam,
) {
    let server_config = server_config.server_config();
    let message = &trigger.message;
    let sender = trigger.sender;
    info!(
        "Received first contact message: {:?} from {:?}",
        message, sender
    );

    // Update the client's state
    if let Ok((client_entity, mut client)) = clients.get_mut(sender) {
        client.name = Some(message.bot_name.clone());

        commands.entity(client_entity).insert(InTeam {
            team_name: message.team_name.clone(),
        });
    }

    // get or insert lobby
    if lobby_management
        .get_or_insert_lobby_entity(
            &message.lobby_id,
            sender,
            message.map_name.as_deref(),
            &mut commands,
            server_config,
        )
        .is_err()
    {
        error!("Failed to get or insert lobby for client {:?}", sender);
    }
}
