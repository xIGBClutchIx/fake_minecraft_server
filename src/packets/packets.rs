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
    handshake HANDSHAKE {
        serverbound SERVERBOUND {
            0x01 => PacketHandshake
        }
        clientbound CLIENTBOUND {
        }
    }
    status STATUS {
        serverbound SERVERBOUND {
            0x00 => PacketRequest
            0x01 => PacketPing
        }
        clientbound CLIENTBOUND {
        }
    }
    login LOGIN {
        serverbound SERVERBOUND {
        }
        clientbound CLIENTBOUND {
        }
    }
    play PLAY {
        serverbound SERVERBOUND {
        }
        clientbound CLIENTBOUND {
        }
    }
);
