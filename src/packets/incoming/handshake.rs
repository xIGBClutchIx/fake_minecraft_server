use crate::{extensions::*, packets::handler::PacketIncoming, socket::*};

use async_trait::async_trait;
use std::io::Cursor;

pub struct PacketHandshake;

#[async_trait]
impl PacketIncoming for PacketHandshake {

    async fn handle(socket: &mut SocketClient, data: &mut Cursor<Vec<u8>>) {
        let protocol = data.read_varint();
        let server_address = data.read_string();
        let port = data.read_short();
        debug!("{}: (Handshake) {} > {:?}", socket.address, "Last State", socket.state);
        let state = State::from_u16(data.read_varint());

        debug!("{}: (Handshake) {} > {:?}", socket.address, "Protocol", protocol);
        debug!("{}: (Handshake) {} > {}", socket.address, "Address", server_address);
        debug!("{}: (Handshake) {} > {}", socket.address, "Port", port);
        debug!("{}: (Handshake) {} > {:?}", socket.address, "State", state);
        socket.state = state;
        socket.send_string(0x00i32, "Handshake Response", ServerStatus::status()).await;
    }
}