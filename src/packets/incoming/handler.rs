use crate::packets::incoming::unknown::packets::*;
use crate::socket::SocketClient;

use phf::{phf_map};

use std::io::Cursor;

pub trait PacketIncoming {
    fn handle_data(&self, socket: &SocketClient, data: &mut Cursor<Vec<u8>>);
}

const PACKETS: phf::Map<i32, &dyn PacketIncoming> = phf_map! {
    0x00i32 => &Packet0,
};

pub struct PacketIncomingHandler;

impl PacketIncomingHandler {
    pub fn handle_data(client: &SocketClient, packet_id: i32, buffer: Vec<u8>) {
        trace!("{}: {:#04x} > {:?}", client.address, packet_id, buffer);
        
        match PACKETS.get(&packet_id) {
            Some(packet) => packet.handle_data(client, &mut Cursor::new(buffer)),
            None => error!("{}: {:#04x} > Unknown", client.address, packet_id)
        }
    }
}
