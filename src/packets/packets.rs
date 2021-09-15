use std::io::Cursor;

use crate::{
    packet_ids,
    packets::{
        handler::PacketIncoming
    },
    socket::{Direction, SocketClient, State}
};

packet_ids!(
    HANDSHAKE {
        SERVERBOUND {
            0x00 => PacketHandshake {
                protocol: i32,
                server_address: String,
                port: u16,
                state: i32,
            }
        }
        CLIENTBOUND {
        }
    }
    STATUS {
        SERVERBOUND {
            0x00 => PacketRequest {
            }
        }
    }
);

