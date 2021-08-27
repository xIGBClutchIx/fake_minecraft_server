use crate::extensions::*;
use crate::packets::incoming::handler::PacketIncoming;
use crate::socket::*;

use serde_json::json;
use std::io::Cursor;

pub struct Packet0x00;
pub struct Packet0x01;

impl PacketIncoming for Packet0x00 {

    fn handle_unknown(&self, socket: &mut SocketClient, data: &mut Cursor<Vec<u8>>) {
        let protocol = data.read_varint();
        let server_address = data.read_string();
        let port = data.read_short();
        debug!("{}: (Handshake) {} > {:?}", socket.address, "Last State", socket.state);
        let state = ConnectionState::from_u16(data.read_varint());

        debug!("{}: (Handshake) {} > {:?}", socket.address, "Protocol", protocol);
        debug!("{}: (Handshake) {} > {}", socket.address, "Address", server_address);
        debug!("{}: (Handshake) {} > {}", socket.address, "Port", port);
        debug!("{}: (Handshake) {} > {:?}", socket.address, "State", state);
        socket.state = state;

        // TODO Better configuration
        if socket.state == ConnectionState::STATUS {
            let response = json!({
                "version": {
                    "name": "1.17.1",
                    "protocol": 756
                },
                "players": {
                    "max": 64,
                    "online": 1,
                    "sample": [
                        {
                            "name": "Clutch",
                            "id": "2a8e267f-88d7-4175-8825-00e81a680076"
                        }
                    ]
                },
                "description": {
                    "text": "A Fake Minecraft Server"
                },
                "favicon": "data:image/png;base64,<data>"
            }).to_string();

            socket.send_string(0x00i32, "Status Response", response);
        } else if socket.state == ConnectionState::LOGIN {
            // TODO do login process
        }
    }
}

impl PacketIncoming for Packet0x01 {

    fn handle_status(&self, socket: &mut SocketClient, data: &mut Cursor<Vec<u8>>) {
        let payload = data.read_long();
        debug!("{}: (Ping) {} > {:?}", socket.address, "Payload", payload);

        socket.send_i64(0x01i32, "Ping Response", payload);
    }
}
