use std::io::Cursor;

use crate::{
    packet_ids,
    packets::{
        handler::PacketIncoming
    },
    socket::{Direction, SocketClient, State}
};
use crate::packets::types::{Short, VarInt};

packet_ids!(
    HANDSHAKE {
        SERVERBOUND {
            0x00 => PacketHandshake {
                protocol: VarInt,
                server_address: String,
                port: Short,
                state: State,
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

