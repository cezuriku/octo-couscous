use std::{
    net::{SocketAddr, UdpSocket},
    time::{Duration, SystemTime},
};

use renet::*;
use transport::{ClientAuthentication, NetcodeClientTransport};

fn main() {
    let mut client = RenetClient::new(ConnectionConfig::default());

    // Setup transport layer
    let server_addr: SocketAddr = "127.0.0.1:5000".parse().unwrap();
    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    let authentication = ClientAuthentication::Unsecure {
        server_addr,
        client_id: 0,
        user_data: None,
        protocol_id: 0,
    };

    let mut transport = NetcodeClientTransport::new(current_time, authentication, socket).unwrap();

    let mut last_send = 0;
    // Your gameplay loop
    loop {
        let delta_time = Duration::from_millis(16);
        // Receive new messages and update client
        client.update(delta_time);
        transport.update(delta_time, &mut client).unwrap();

        if client.is_connected() {
            // Receive message from server
            while let Some(message) = client.receive_message(DefaultChannel::ReliableOrdered) {
                // Handle received message
                println!(
                    "Message from server: {}",
                    std::str::from_utf8(&message).unwrap()
                );

                // Send message
                client.send_message(DefaultChannel::ReliableOrdered, "response");
            }

            if last_send >= 20 {
                last_send = 0;
                client.send_message(DefaultChannel::ReliableOrdered, "spam");
            }
            last_send += 1;
        }

        // Send packets to server using the transport layer
        transport.send_packets(&mut client).unwrap();

        std::thread::sleep(delta_time); // Running at 60hz
    }
}
