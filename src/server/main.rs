use std::net::UdpSocket;
use std::time::SystemTime;

use bevy::prelude::*;
use bevy_renet::renet::transport::{NetcodeServerTransport, ServerAuthentication, ServerConfig};
use bevy_renet::renet::{ConnectionConfig, DefaultChannel, RenetServer, ServerEvent};
use bevy_renet::transport::NetcodeServerPlugin;
use bevy_renet::RenetServerPlugin;

#[derive(Component)]
struct Factory {
    production: i32,
}

fn add_factory(mut commands: Commands) {
    commands.spawn(Factory { production: 5 });
}

#[derive(Resource)]
struct Room {
    bananas: i32,
}

#[derive(Resource)]
struct ProductionTimer(Timer);

fn print_bananas(time: Res<Time>, room: Res<Room>, mut timer: ResMut<ProductionTimer>) {
    if timer.0.tick(time.delta()).just_finished() {
        println!("Room bananas: {}", room.bananas)
    }
}

#[derive(Resource)]
struct GreetTimer(Timer);

fn collect_production(
    time: Res<Time>,
    mut room: ResMut<Room>,
    mut timer: ResMut<GreetTimer>,
    query: Query<&Factory>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for factory in &query {
            room.bananas += factory.production
        }
    }
}

pub struct OctoCouscousServerPlugin;

impl Plugin for OctoCouscousServerPlugin {
    fn build(&self, app: &mut App) {
        let server = RenetServer::new(ConnectionConfig::default());

        let server_addr = "127.0.0.1:5000".parse().unwrap();
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
            .insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .insert_resource(ProductionTimer(Timer::from_seconds(
                1.0,
                TimerMode::Repeating,
            )))
            .insert_resource(Room { bananas: 0 })
            .add_systems(Startup, add_factory)
            .add_systems(Update, (collect_production, print_bananas))
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

fn send_message_system(mut server: ResMut<RenetServer>, room: Res<Room>) {
    if room.is_changed() {
        for client_id in server.clients_id() {
            let message = format!("Bananas: {}", room.bananas);
            server.send_message(client_id, DefaultChannel::ReliableOrdered, message)
        }
    }
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

fn handle_events_system(mut server_events: EventReader<ServerEvent>) {
    for event in server_events.read() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                println!("Client {client_id} connected");
            }
            ServerEvent::ClientDisconnected { client_id, reason } => {
                println!("Client {client_id} disconnected: {reason}");
            }
        }
    }
}

fn main() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, OctoCouscousServerPlugin))
        .run();
}
