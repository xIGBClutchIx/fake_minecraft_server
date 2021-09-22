pub mod incoming;
pub mod handler;
pub mod types;

use crate::{packet_ids, packets::{handler::{PacketIncoming, get_field}, types::*}, socket::{Direction, State, SocketClient}};

use std::io::Cursor;

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
    }
    STATUS {
        SERVERBOUND {
            0x00 => PacketRequest {
            }
            0x01 => PacketPing {
                payload: Long,
            }
        }
    }
);
