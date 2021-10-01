pub mod handshake;
pub mod login;
pub mod play;
pub mod status;

use crate::{serverbound_packets, packets::{handler::{PacketServerbound, get_field}, types::*, server_status::*}, socket::{State, SocketClient}};
use std::io::Cursor;

serverbound_packets!(
    HANDSHAKE {
        0x00 => PacketHandshakeRequest {
            protocol: VarInt,
            server_address: String,
            port: Short,
            state: State,
        }
    }
    STATUS {
        0x00 => PacketStatusRequest {
        }
        0x01 => PacketPing {
            payload: Long,
        }
    }
);
