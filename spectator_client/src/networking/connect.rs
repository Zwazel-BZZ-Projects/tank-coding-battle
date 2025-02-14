use std::net::TcpStream;

use bevy::prelude::*;
use shared::asset_handling::config::ClientConfigSystemParam;

use crate::networking::MyNetworkStream;

pub fn connect_to_server(client_config: ClientConfigSystemParam, mut commands: Commands) {
    let client_config = client_config.client_config();

    info!(
        "Trying to connect to server at {}:{}...",
        client_config.ip, client_config.port
    );

    // Connect to tcp stream
    let stream = match TcpStream::connect((client_config.ip.as_str(), client_config.port)) {
        Ok(stream) => stream,
        Err(e) => {
            error!("Failed to connect to server: {}", e);
            return;
        }
    };

    info!(
        "Connected to server at {}:{}!",
        client_config.ip, client_config.port
    );

    commands.spawn((Name::new("LocalClient"), MyNetworkStream(stream)));
}
