use crate::{packets::{clientbound::PacketPong, handler::{PacketClientbound, PacketServerbound}, serverbound::*}, socket::*};

use async_trait::async_trait;

#[async_trait]
impl PacketServerbound for PacketStatusRequest {
    async fn handle(&self, _socket: &mut SocketClient) {
        //socket.send_status(0x00, "Status Response", ServerStatus::status()).await;
    }
}

#[async_trait]
impl PacketServerbound for PacketPing {
    async fn handle(&self, socket: &mut SocketClient) {
        trace!("{} > (Ping) Payload: {}", socket.address, self.payload);

        let packet = PacketPong {
            payload: self.payload
        };
        packet.send(0x01, socket).await;
    }
}
