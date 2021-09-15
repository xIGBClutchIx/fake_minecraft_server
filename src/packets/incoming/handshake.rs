/*
use crate::{extensions::*, packets::{handler::PacketIncoming, types::{VarInt, Short}}, socket::*};

use async_trait::async_trait;
use std::io::Cursor;

pub struct PacketHandshake;

#[async_trait]
impl PacketIncoming for PacketHandshake {

    async fn handle(socket: &mut SocketClient, data: &mut Cursor<Vec<u8>>) {
        // TODO
        // Auto print details?

        let protocol: VarInt = data.read_varint().await.into();
        let server_address = data.read_string().await;
        let port: Short = data.read_short().await.into();
        let state = State::from_i32(data.read_varint().await);

        trace!("{}: (Handshake) Last State > {:?}", socket.address, socket.state);
        trace!("{}: (Handshake) Protocol > {}", socket.address, protocol);
        trace!("{}: (Handshake) Address > {}", socket.address, server_address);
        trace!("{}: (Handshake) Port > {}", socket.address, port);
        trace!("{}: (Handshake) State > {:?}", socket.address, state);
        // socket.state = State::from_u16(data[3] as i32)
        socket.state = state;
        debug!("{}: {} = {:?}", socket.address, "State", state);
        socket.send_string(0x00, "Handshake Response", ServerStatus::status()).await;
    }
}
*/
