use crate::{packets::{handler::PacketIncoming, *}, socket::*};

use async_trait::async_trait;

#[async_trait]
impl PacketIncoming for PacketHandshake {

    async fn handle(&self, socket: &mut SocketClient) {
        trace!("{}: Last State > {:?}", socket.address, socket.state);
        socket.state = self.state;
        trace!("{}: New State = {:?}", socket.address, self.state);

        socket.send_string(0x00, "Handshake Response", ServerStatus::status()).await;
    }
}
