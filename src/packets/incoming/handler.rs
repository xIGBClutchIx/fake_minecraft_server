use crate::packets::incoming::packets::*;
use crate::socket::{SocketClient, ConnectionState};

use phf::phf_map;

use std::io::Cursor;

pub trait PacketIncoming {
    fn handle_unknown(&self, _socket: &mut SocketClient, _data: &mut Cursor<Vec<u8>>) {}
    fn handle_status(&self, _socket: &mut SocketClient, _data: &mut Cursor<Vec<u8>>) {}
    fn handle_play(&self, _socket: &mut SocketClient, _data: &mut Cursor<Vec<u8>>) {}
    fn handle_login(&self, _socket: &mut SocketClient, _data: &mut Cursor<Vec<u8>>) {}
}

const PACKETS: phf::Map<i32, &dyn PacketIncoming> = phf_map! {
    0x00i32 => &Packet0x00,
    0x01i32 => &Packet0x01,
};

pub struct PacketIncomingHandler;

impl PacketIncomingHandler {
    pub fn handle_data(client: &mut SocketClient, packet_id: i32, buffer: Vec<u8>) {
        trace!("{}: {:#04x} > {:?}", client.address, packet_id, buffer);

        match PACKETS.get(&packet_id) {
            Some(packet) => {
                let cursor = &mut Cursor::new(buffer);
                match client.state {
                    ConnectionState::STATUS => packet.handle_status(client, cursor),
                    ConnectionState::PLAY => packet.handle_play(client, cursor),
                    ConnectionState::LOGIN => packet.handle_login(client, cursor),
                    _ => packet.handle_unknown(client, cursor)
                }
            },
            None => error!("{}: {:#04x} > Unknown", client.address, packet_id)
        }
    }
}
