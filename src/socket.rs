use crate::extensions::CursorExt;
use crate::packets::incoming::handler::PacketIncomingHandler;

use std::{
    io::{Cursor, Read},
    net::{SocketAddr, SocketAddrV4, TcpListener, TcpStream},
    thread::spawn,
};

pub struct SocketServer {
    address: SocketAddrV4,
}

impl SocketServer {
    pub fn new(address: SocketAddrV4) -> Self {
        Self { address }
    }

    fn handle_conn(&self, socket: TcpStream) {
        spawn(move || {
            let mut client = SocketClient::new(socket);
            debug!("{}: Connected", client.address);
            client.handle();
        });
    }

    pub fn listen(&self) {
        let listener = TcpListener::bind(self.address).unwrap();
        info!("{}: Server started!", self.address);

        for stream in listener.incoming() {
            match stream {
                Ok(socket) => self.handle_conn(socket),
                Err(e) => error!("failed to handle stream: {}", e),
            };
        }
        drop(listener);
    }
}

#[derive(Debug)]
pub enum ConnectionState {
    UNKNOWN,
    STATUS,
    LOGIN,
    PLAY
}

impl ConnectionState {
    pub fn from_u16(value: i32) -> ConnectionState {
        match value {
            1 => ConnectionState::STATUS,
            2 => ConnectionState::LOGIN,
            3 => ConnectionState::PLAY,
            _ => ConnectionState::UNKNOWN
        }
    }
}

pub struct SocketClient {
    pub address: SocketAddr,
    socket: TcpStream,
    pub state: ConnectionState
}

impl SocketClient {

    pub fn new(socket: TcpStream) -> Self {
        Self {
            address: socket.peer_addr().expect("failed to get address"),
            socket,
            state: ConnectionState::UNKNOWN
        }
    }

    // Note: No negatives due to it being a u8. Will this matter or does it convert correctly?
    pub fn handle(&mut self) {
        loop {
            let mut buffer = vec![0; 2097050];
            match self.socket.read(&mut buffer) {
                Ok(length) => {
                    buffer.resize(length, 0);
                    if buffer.len() > 0 {
                        let mut data = Cursor::new(buffer.clone());
                        let length = data.read_varint();
                        let packet_id = data.read_varint();

                        if buffer.len() > 0 {
                            let _ = buffer.remove(0);
                        }
                        if buffer.len() > 0 {
                            let _ = buffer.remove(0);
                        }
                        buffer.resize(length as usize, 0);
                        PacketIncomingHandler::handle_data(self, packet_id, buffer);
                    }
                }
                Err(e) => error!("failed to find length of data: {}", e),
            }
        }
    }
}
