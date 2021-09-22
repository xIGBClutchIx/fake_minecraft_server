use crate::{packets::{handler::PacketIncoming, *}, socket::*};

use async_trait::async_trait;

#[async_trait]
impl PacketIncoming for PacketRequest {
    async fn handle(&self, socket: &mut SocketClient) {
        socket.send_string(0x00, "Status Response", ServerStatus::status()).await;
    }
}

#[async_trait]
impl PacketIncoming for PacketPing {
    async fn handle(&self, socket: &mut SocketClient) {
        trace!("{}: (Ping) Payload > {}", socket.address, self.payload);

        socket.send_long(0x01, "Pong Response", self.payload).await;
    }
}
