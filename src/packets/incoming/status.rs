use crate::{extensions::*, packets::handler::PacketIncoming, socket::*};

use async_trait::async_trait;
use std::io::Cursor;

pub struct PacketRequest;
pub struct PacketPing;

#[async_trait]
impl PacketIncoming for PacketRequest {
    async fn handle(socket: &mut SocketClient, _data: &mut Cursor<Vec<u8>>) {
        socket.send_string(0x00, "Status Response", ServerStatus::status()).await;
    }
}

#[async_trait]
impl PacketIncoming for PacketPing {
    async fn handle(socket: &mut SocketClient, data: &mut Cursor<Vec<u8>>) {
        let payload = data.read_long();
        trace!("{}: (Ping) {} > {:?}", socket.address, "Payload", payload);

        socket.send_i64(0x01, "Ping Response", payload).await;
    }
}
