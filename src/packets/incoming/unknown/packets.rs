use crate::extensions::*;
use crate::packets::incoming::handler::PacketIncoming;
use crate::socket::*;

use std::io::Cursor;

pub struct PacketHandshake;

impl PacketIncoming for PacketHandshake {

    fn handle_data(&self, socket: &mut SocketClient, data: &mut Cursor<Vec<u8>>) {
        let protocol = data.read_varint();
        let server_address = data.read_string();
        let port = data.read_short();
        let state = ConnectionState::from_u16(data.read_varint());

        debug!("{}: (Handshake) {} > {:?}", socket.address, "Protocol", protocol);
        debug!("{}: (Handshake) {} > {}", socket.address, "Address", server_address);
        debug!("{}: (Handshake) {} > {}", socket.address, "Port", port);
        debug!("{}: (Handshake) {} > {:?}", socket.address, "State", state);

        socket.state = state;
    }
}
