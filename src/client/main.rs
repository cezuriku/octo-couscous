use std::net::UdpSocket;
use std::time::SystemTime;

use bevy::prelude::*;
use bevy_renet::renet::transport::{ClientAuthentication, NetcodeClientTransport};
use bevy_renet::renet::{ConnectionConfig, DefaultChannel, RenetClient};
use bevy_renet::transport::NetcodeClientPlugin;
use bevy_renet::RenetClientPlugin;
use map_plugin::{GraphicalMapPlugin, MapPlugin};
use rand::Rng;

pub struct OctoCouscousClientPlugin;

impl Plugin for OctoCouscousClientPlugin {
    fn build(&self, app: &mut App) {
        // let mut rng = rand::thread_rng();
        // let client = RenetClient::new(ConnectionConfig::default());
        // app.add_plugins(RenetClientPlugin).insert_resource(client);

        // // Setup the transport layer
        // app.add_plugins(NetcodeClientPlugin);

        // let authentication = ClientAuthentication::Unsecure {
        //     server_addr: "127.0.0.1:5000".parse().unwrap(),
        //     client_id: rng.gen(),
        //     user_data: None,
        //     protocol_id: 0,
        // };
        // let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
        // let current_time = SystemTime::now()
        //     .duration_since(SystemTime::UNIX_EPOCH)
        //     .unwrap();
        // let transport = NetcodeClientTransport::new(current_time, authentication, socket).unwrap();
        // app.insert_resource(transport);
        // app.add_systems(Update, (send_message_system, receive_message_system));

        app.add_plugins(GraphicalMapPlugin);
    }
}

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_plugins(OctoCouscousClientPlugin)
        .run();
}

// Systems

fn send_message_system(mut _client: ResMut<RenetClient>) {
    // Send a text message to the server
    // client.send_message(DefaultChannel::ReliableOrdered, "server message");
}

fn receive_message_system(mut client: ResMut<RenetClient>) {
    while let Some(message) = client.receive_message(DefaultChannel::ReliableOrdered) {
        // Handle received message
        println!("{}", std::str::from_utf8(&message).unwrap())
    }
}
