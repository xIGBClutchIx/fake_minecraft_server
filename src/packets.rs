use crate::extensions::*;
use crate::socket::SocketClient;

use std::io::Cursor;

pub trait Packet {
    fn handle_data(&self, socket: &SocketClient, data: &mut Cursor<Vec<u8>>);
}

pub struct Packet0;

impl Packet for Packet0 {

    fn handle_data(&self, socket: &SocketClient, data: &mut Cursor<Vec<u8>>) {
        debug!("{}: {} > {:?}", socket.address, "Protocol", data.read_varint());
    }
}
