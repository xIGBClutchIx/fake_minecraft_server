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
        trace!("{}: (Handshake) {} > {:?}", socket.address, "Last State", socket.state);
        let state = State::from_u16(data.read_varint());

        trace!("{}: (Handshake) {} > {:?}", socket.address, "Protocol", protocol);
        trace!("{}: (Handshake) {} > {}", socket.address, "Address", server_address);
        trace!("{}: (Handshake) {} > {}", socket.address, "Port", port);
        trace!("{}: (Handshake) {} > {:?}", socket.address, "State", state);
        socket.state = state;
        socket.send_string(0x00, "Handshake Response", ServerStatus::status()).await;
    }
}