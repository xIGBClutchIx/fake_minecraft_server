use crate::extensions::CursorExt;
use crate::packets::incoming::handler::PacketIncomingHandler;

use std::{
    io::{Cursor, Read},
    net::{SocketAddr, SocketAddrV4, TcpListener, TcpStream},
    sync::{Arc, Mutex},
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
            debug!("Connection: {}", client.address);
            client.handle();
        });
    }

    pub fn listen(&self) {
        let listener = TcpListener::bind(self.address).unwrap();
        info!("Listening: {}", self.address);

        for stream in listener.incoming() {
            match stream {
                Ok(socket) => self.handle_conn(socket),
                Err(e) => error!("failed to handle stream: {}", e),
            };
        }
        drop(listener);
    }
}

pub struct SocketClient {
    pub address: SocketAddr,
    socket: Arc<Mutex<TcpStream>>,
}

impl SocketClient {

    pub fn new(socket: TcpStream) -> Self {
        Self {
            address: socket.peer_addr().expect("failed to get address"),
            socket: Arc::new(Mutex::new(socket)),
        }
    }

    // Note: No negatives due to it being a u8. Will this matter or does it convert correctly?
    pub fn handle(&mut self) {
        let socket = &mut self.socket.lock().expect("failed to obtain socket");
        loop {
            let mut buffer = vec![0; 2097050];
            match socket.read(&mut buffer) {
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
