use std::{
    collections::VecDeque,
    net::{SocketAddr, TcpStream},
};

use bevy::prelude::*;
use shared::networking::messages::message_container::MessageContainer;

#[derive(Debug, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct MyLocalClient {
    pub network_client: Entity,
}

#[derive(Debug, Component)]
pub struct MyNetworkClient {
    pub name: Option<String>,
    pub address: SocketAddr,
    pub stream: TcpStream,
    pub my_local_client: Option<Entity>,
    pub outgoing_messages_queue: VecDeque<MessageContainer>,
}

impl MyNetworkClient {
    pub fn new(address: SocketAddr, stream: TcpStream) -> Self {
        Self {
            name: None,
            address,
            stream,
            my_local_client: None,
            outgoing_messages_queue: VecDeque::new(),
        }
    }
}

#[derive(Event, Deref, DerefMut)]
pub struct ClientConnectedTrigger(pub Entity);

#[derive(Event, Deref, DerefMut)]
pub struct ClientDisconnectedTrigger(pub Entity);

#[derive(Event)]
pub struct ClientHasBeenDespawnedTrigger;
