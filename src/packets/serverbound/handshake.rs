use crate::{packets::{clientbound::PacketHandshakeStatus, handler::{PacketClientbound, PacketServerbound}, serverbound::*}, socket::*};

use async_trait::async_trait;

#[async_trait]
impl PacketServerbound for PacketHandshakeRequest {

    async fn handle(&self, socket: &mut SocketClient) {
        trace!("{} > (Handshake) State Change: {:?} > {:?}", socket.address, socket.state, self.state);
        socket.state = self.state;

        let test = PacketHandshakeStatus {
            status: Status {
                version: StatusVersion {
                    name: "1.17.1".to_string(),
                    protocol: 756
                },
                players: StatusPlayers {
                    max: 64,
                    online: 1,
                    sample: vec![StatusPlayer {
                        name: "Clutch".to_string(),
                        id: "2a8e267f-88d7-4175-8825-00e81a680076".to_string()
                    }]
                },
                description: StatusDescription {
                    text: "A Fake Minecraft Server".to_string()
                }
            }
        };
        test.send(0x00, socket).await;
    }
}
