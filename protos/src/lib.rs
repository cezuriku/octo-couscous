use std::io::Cursor;

use prost::Message;
use protos::messages::*;

pub mod protos {
    pub mod messages {
        include!(concat!(env!("OUT_DIR"), "/protos.messages.rs"));
    }
}

pub fn serialize_client_message(client_message: ClientMessage) -> Vec<u8> {
    let mut buf = Vec::with_capacity(client_message.encoded_len());

    client_message.encode(&mut buf).unwrap();
    buf
}

pub fn serialize_server_message(server_message: ServerMessage) -> Vec<u8> {
    let mut buf = Vec::with_capacity(server_message.encoded_len());

    server_message.encode(&mut buf).unwrap();
    buf
}

pub fn deserialize_client_message(buf: &[u8]) -> Result<ClientMessage, prost::DecodeError> {
    ClientMessage::decode(&mut Cursor::new(buf))
}

#[cfg(test)]
mod test {
    use crate::protos::messages::{server_message::Message, DebugMessage, ServerMessage};

    #[test]
    fn create_server_message() {
        let message = ServerMessage {
            message: Some(Message::DebugMessage(DebugMessage {
                content: String::from("Hello"),
            })),
        };

        let inner_message = message.message;
        assert!(inner_message.is_some());
        let mut content: String = String::from("");
        if let Some(m) = inner_message {
            match m {
                Message::DebugMessage(debug_message) => content = debug_message.content,
            }
        }
        assert_eq!(content, String::from("Hello"))
    }
}
