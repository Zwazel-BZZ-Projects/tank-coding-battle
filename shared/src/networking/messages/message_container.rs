use bevy::prelude::*;
use proc_macros::{auto_trigger_message_received, generate_message_data_triggers};
use serde::{Deserialize, Serialize};

use crate::{
    game::game_state::GameState,
    networking::{
        lobby_management::lobby_management::{LobbyManagementArgument, LobbyManagementSystemParam},
        messages::message_queue::{InMessageQueue, OutMessageQueue},
    },
};

use super::message_data::{
    first_contact::FirstContactData, game_starts::GameStarts,
    message_error_types::ErrorMessageTypes, text_data::TextDataWrapper,
};

#[derive(Serialize, Deserialize, Default, Reflect, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
#[auto_trigger_message_received(
    target = {
        #[derive(Serialize, Deserialize, Default, Reflect, Clone, Debug, PartialEq)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE", tag = "type", content = "clientId")]
        pub enum MessageTarget {
            #[default]
            #[get_targets(targets_get_players_in_lobby_team)]
            // To everyone in the same team in the same lobby
            Team,
            #[get_targets(targets_get_empty)]
            // To the server directly, no lobby or client. Used for first contact
            ServerOnly,
            #[get_targets(targets_get_players_in_lobby)]
            // To everyone in the same lobby
            AllInLobby,
            #[get_targets(targets_get_single_player)]
            // To a single player
            Client(Entity),
            #[get_targets(targets_get_lobby_directly)]
            // To the lobby itself (is there even a usecase for that?)
            ToLobbyDirectly,
        }
    },
    message = {
        #[derive(Serialize, Deserialize, Reflect, Clone, Debug, PartialEq)]
        #[serde(tag = "message_type")]
        #[generate_message_data_triggers]
        pub enum NetworkMessageType {
            #[target(ServerOnly)]
            FirstContact(FirstContactData),
            GameState(GameState),
            #[serde(rename = "SimpleTextMessage")]
            #[target(Client, Team, AllInLobby)]
            SimpleTextMessage(TextDataWrapper),
            MessageError(ErrorMessageTypes),
            #[serde(rename = "GameConfig")]
            GameStarts(GameStarts),
            #[serde(rename = "SuccessfullyJoinedLobby")]
            SuccessFullyJoinedLobby(TextDataWrapper),
        }
    }
)]
pub struct MessageContainer {
    pub target: MessageTarget,
    pub message: NetworkMessageType,

    #[serde(skip)]
    pub sender: Option<Entity>,

    /// The tick when the message was sent
    pub tick_sent: u64,
    /// The tick when the message was received
    pub tick_received: u64,
}

impl MessageContainer {
    pub fn new_sent(target: MessageTarget, message: NetworkMessageType, tick: u64) -> Self {
        let mut message = MessageContainer::new(target, message);
        message.with_sent(tick);

        message
    }

    pub fn new_received(
        target: MessageTarget,
        message: NetworkMessageType,
        tick: u64,
        sender: Entity,
    ) -> Self {
        let mut message = MessageContainer::new(target, message);
        message.with_received(tick, sender);

        message
    }

    pub fn new(target: MessageTarget, message: NetworkMessageType) -> Self {
        MessageContainer {
            target,
            message,

            sender: None,

            tick_sent: 0,
            tick_received: 0,
        }
    }

    pub fn with_received(&mut self, tick: u64, sender: Entity) -> &mut Self {
        self.tick_received = tick;
        self.sender = Some(sender);
        self
    }

    pub fn with_sent(&mut self, tick: u64) -> &mut Self {
        self.tick_sent = tick;
        self
    }
}

impl Default for NetworkMessageType {
    fn default() -> Self {
        NetworkMessageType::GameState(GameState::default())
    }
}
