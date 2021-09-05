use crate::extensions::*;
use crate::packets::incoming::handler::PacketIncoming;
use crate::socket::*;

use std::io::Cursor;

pub struct Packet0x00;
pub struct Packet0x01;

impl PacketIncoming for Packet0x00 {

    fn handle_unknown(&self, socket: &mut SocketClient, data: &mut Cursor<Vec<u8>>) {
        let protocol = data.read_varint();
        let server_address = data.read_string();
        let port = data.read_short();
        debug!("{}: (Handshake) {} > {:?}", socket.address, "Last State", socket.state);
        let state = ConnectionState::from_u16(data.read_varint());

        debug!("{}: (Handshake) {} > {:?}", socket.address, "Protocol", protocol);
        debug!("{}: (Handshake) {} > {}", socket.address, "Address", server_address);
        debug!("{}: (Handshake) {} > {}", socket.address, "Port", port);
        debug!("{}: (Handshake) {} > {:?}", socket.address, "State", state);
        socket.state = state;
        socket.send_string(0x00i32, "Handshake Response", ServerStatus::get_status());
    }

    fn handle_status(&self, socket: &mut SocketClient, _data: &mut Cursor<Vec<u8>>) {
        socket.send_string(0x00i32, "Status Response", ServerStatus::get_status());
    }
}

impl PacketIncoming for Packet0x01 {

    fn handle_status(&self, socket: &mut SocketClient, data: &mut Cursor<Vec<u8>>) {
        let payload = data.read_long();
        debug!("{}: (Ping) {} > {:?}", socket.address, "Payload", payload);

        socket.send_i64(0x01i32, "Ping Response", payload);
    }
}
