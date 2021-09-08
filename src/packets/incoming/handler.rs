use crate::packets::incoming::packets::*;
use crate::socket::{SocketClient, ConnectionState};

use async_trait::async_trait;
use phf::phf_map;
use std::io::Cursor;

#[async_trait]
pub trait PacketIncoming: Sync {
    async fn handle_unknown(&self, _socket: &mut SocketClient, _data: &mut Cursor<Vec<u8>>) {}
    async fn handle_status(&self, _socket: &mut SocketClient, _data: &mut Cursor<Vec<u8>>) {}
    async fn handle_play(&self, _socket: &mut SocketClient, _data: &mut Cursor<Vec<u8>>) {}
    async fn handle_login(&self, _socket: &mut SocketClient, _data: &mut Cursor<Vec<u8>>) {}
}

const PACKETS: phf::Map<i32, &dyn PacketIncoming> = phf_map! {
    0x00i32 => &Packet0x00,
    0x01i32 => &Packet0x01,
};

pub struct PacketIncomingHandler;

impl PacketIncomingHandler {

    pub async fn handle_data(client: &mut SocketClient, packet_id: i32, buffer: Vec<u8>) {
        trace!("{}: {:#04x} > {:?}", client.address, packet_id, buffer);

        match PACKETS.get(&packet_id) {
            Some(packet) => {
                let cursor = &mut Cursor::new(buffer);
                match client.state {
                    ConnectionState::STATUS => packet.handle_status(client, cursor).await,
                    ConnectionState::PLAY => packet.handle_play(client, cursor).await,
                    ConnectionState::LOGIN => packet.handle_login(client, cursor).await,
                    _ => packet.handle_unknown(client, cursor).await,
                }
            },
            None => error!("{}: {:#04x} > Unknown", client.address, packet_id)
        }
    }
}
