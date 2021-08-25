use crate::extensions::*;
use crate::packets::incoming::handler::PacketIncoming;
use crate::socket::SocketClient;

use std::io::Cursor;

pub struct Packet0;

impl PacketIncoming for Packet0 {
    fn handle_data(&self, socket: &SocketClient, data: &mut Cursor<Vec<u8>>) {
        debug!("{}: (0x00) {} > {:?}", socket.address, "Protocol", data.read_varint());
    }
}
