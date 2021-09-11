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
        trace!("{}: (Handshake) Last State > {:?}", socket.address, socket.state);
        let state = State::from_u16(data.read_varint());

        trace!("{}: (Handshake) Protocol > {:?}", socket.address, protocol);
        trace!("{}: (Handshake) Address > {}", socket.address, server_address);
        trace!("{}: (Handshake) Port > {}", socket.address, port);
        trace!("{}: (Handshake) State > {:?}", socket.address, state);
        socket.state = state;
        debug!("{}: {} = {:?}", socket.address, "State", state);
        socket.send_string(0x00, "Handshake Response", ServerStatus::status()).await;
    }
}
