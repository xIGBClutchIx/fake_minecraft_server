use crate::{extensions::StringExt, packets::{clientbound::*, handler::{PacketClientbound}}, socket::*};

impl PacketClientbound for PacketHandshakeStatus {

    fn get_data(&self, _socket: &mut SocketClient) -> Vec<u8> {
        return serde_json::to_string(&self.status).unwrap().as_vec();
    }
}
