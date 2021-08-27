use crate::extensions::{CursorExt, Vec8Ext};
use crate::packets::incoming::handler::PacketIncomingHandler;

use std::{
    io::{Cursor, Read},
    net::{SocketAddr, SocketAddrV4, TcpListener, TcpStream},
    thread::spawn,
};
use std::io::Write;

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

#[derive(Debug, PartialEq, Eq, Hash)]
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

    pub fn send_i64(&mut self, packet_id: i32, packet_name: &str, data: i64) {
        self.send_data(packet_id, packet_name, data.to_ne_bytes().to_vec());
    }

    pub fn send_string(&mut self, packet_id: i32, packet_name: &str, string: String) {
        let string_data = string.into_bytes();
        let mut data: Vec<u8> = Vec::new();
        // String Size
        data.add_varint(string_data.len() as i32);
        // String and size
        let end_data = [data, string_data].concat();
        self.send_data(packet_id, packet_name, end_data);
    }

    pub fn send_data(&mut self, packet_id: i32, packet_name: &str, response: Vec<u8>) {
        // Packet ID + String Size + Data
        let mut data: Vec<u8> = Vec::new();
        // Packet ID
        data.add_varint(packet_id);
        // Data
        data = [data, response].concat();

        // Full packet size and packet
        let mut end_data: Vec<u8> = Vec::new();
        end_data.add_varint(data.len() as i32);
        end_data = [end_data, data].concat();

        match self.socket.write_all(end_data.as_mut()) {
            Ok(_) => info!("{:?} < {}", self.address, packet_name),
            Err(e) => error!("failed to send data: {}", e)
        }
    }

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
