pub mod handshake;
pub mod status;

use crate::{clientbound_packets, packets::{types::*, server_status::*}};

clientbound_packets!(
    HANDSHAKE {
        0x00 => PacketHandshakeStatus {
            status: Status,
        }
    }
    STATUS {
        0x00 => PacketPong {
            payload: Long,
        }
    }
);
