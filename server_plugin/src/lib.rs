use std::net::UdpSocket;
use std::time::SystemTime;

use bevy::prelude::*;
use bevy_renet::renet::transport::{NetcodeServerTransport, ServerAuthentication, ServerConfig};
use bevy_renet::renet::{ConnectionConfig, DefaultChannel, RenetServer, ServerEvent};
use bevy_renet::transport::NetcodeServerPlugin;
use bevy_renet::RenetServerPlugin;

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        let server = RenetServer::new(ConnectionConfig::default());

        let server_addr = "0.0.0.0:5000".parse().unwrap();
        let socket = UdpSocket::bind(server_addr).unwrap();
        let server_config = ServerConfig {
            current_time: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap(),
            max_clients: 64,
            protocol_id: 0,
            public_addresses: vec![server_addr],
            authentication: ServerAuthentication::Unsecure,
        };
        let transport = NetcodeServerTransport::new(server_config, socket).unwrap();

        app.add_plugins(RenetServerPlugin)
            .insert_resource(server)
            .add_plugins(NetcodeServerPlugin)
            .insert_resource(transport)
            .add_systems(
                Update,
                (
                    send_message_system,
                    receive_message_system,
                    handle_events_system,
                ),
            );
    }
}

fn send_message_system(mut _server: ResMut<RenetServer>) {
    // Do nothing
}

fn receive_message_system(mut server: ResMut<RenetServer>) {
    // Receive message from all clients
    for client_id in server.clients_id() {
        while let Some(message) = server.receive_message(client_id, DefaultChannel::ReliableOrdered)
        {
            println!(
                "Message from id:{client_id} {}",
                std::str::from_utf8(&message).unwrap()
            )
            // Handle received message
        }
    }
}

fn handle_events_system(
    mut server_events: EventReader<ServerEvent>,
    mut server: ResMut<RenetServer>,
) {
    for event in server_events.read() {
        println!("Event received");
        match event {
            ServerEvent::ClientConnected { client_id } => {
                println!("Client {client_id} connected");
                server.send_message(*client_id, DefaultChannel::ReliableOrdered, "Hello");
            }
            ServerEvent::ClientDisconnected { client_id, reason } => {
                println!("Client {client_id} disconnected: {reason}");
            }
        }
    }
}
