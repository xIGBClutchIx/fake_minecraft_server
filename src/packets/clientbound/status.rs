use crate::{packets::{clientbound::*, handler::PacketClientbound}, socket::*};

impl PacketClientbound for PacketPong {
    fn get_data(&self, _socket: &mut SocketClient) -> Vec<u8> {
        return self.payload.value.to_ne_bytes().to_vec();
    }
}
