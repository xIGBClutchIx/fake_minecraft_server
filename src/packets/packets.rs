use std::io::Cursor;

use crate::{
    packet_ids,
    packets::{
        handler::PacketIncoming,
        incoming::{
            handshake::PacketHandshake, status::{PacketPing, PacketRequest}
        }
    },
    socket::{Direction, SocketClient, State}
};

packet_ids!(
    HANDSHAKE {
        SERVERBOUND {
            0x00 => PacketHandshake
        }
        CLIENTBOUND {
        }
    }
    STATUS {
        SERVERBOUND {
            0x00 => PacketRequest
            0x01 => PacketPing
        }
        CLIENTBOUND {
        }
    }
    LOGIN {
        SERVERBOUND {
        }
        CLIENTBOUND {
        }
    }
    PLAY {
        SERVERBOUND {
        }
        CLIENTBOUND {
        }
    }
);
