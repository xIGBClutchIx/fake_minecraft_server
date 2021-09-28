use crate::{extensions::{CursorExt, StringExt, Vec8Ext}, packets::{handle_data, server_status::{Status, StatusVersion, StatusPlayers, StatusPlayer, StatusDescription}, types::Long}};

use ansi_term::Colour::{Green, Red};
use std::{io::Cursor, net::{SocketAddr, SocketAddrV4}};
use tokio::{io::{AsyncWriteExt, AsyncReadExt, Interest}, net::{TcpStream, TcpListener}};

pub struct SocketServer {
    address: SocketAddrV4,
}

impl SocketServer {
    pub fn new(address: SocketAddrV4) -> Self {
        Self { address }
    }

    pub async fn listen(&self) {
        let listener = TcpListener::bind(self.address).await.unwrap();
        info!("{}: Server started!", self.address);

        loop {
            let (socket, address) = listener.accept().await.unwrap();
            tokio::spawn(async move {
                let mut client = SocketClient::new(address, socket);
                info!("{}: {}", address, Green.paint("Connected"));
                client.handle().await;
            });
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    SERVERBOUND,
    CLIENTBOUND,
}


#[derive(Clone, Copy, Debug)]
pub enum State {
    HANDSHAKE,
    STATUS,
    LOGIN,
    PLAY
}

impl State {
    pub fn from_i32(value: i32) -> State {
        match value {
            0 => State::HANDSHAKE,
            1 => State::STATUS,
            2 => State::LOGIN,
            3 => State::PLAY,
            _ => State::HANDSHAKE
        }
    }
}

pub struct ServerStatus;

impl ServerStatus {
    pub fn status() -> Status {
        return Status {
            version: StatusVersion {
                name: "1.17.1".to_string(),
                protocol: 756
            },
            players: StatusPlayers {
                max: 64,
                online: 1,
                sample: vec![StatusPlayer {
                    name: "Clutch".to_string(),
                    id: "2a8e267f-88d7-4175-8825-00e81a680076".to_string()
                }]
            },
            description: StatusDescription {
                text: "A Fake Minecraft Server".to_string()
            }
        };
    }
}

pub struct SocketClient {
    pub address: SocketAddr,
    socket: TcpStream,
    pub state: State
}

impl SocketClient {

    pub fn new(address: SocketAddr, socket: TcpStream) -> Self {
        Self {
            address,
            socket,
            state: State::HANDSHAKE
        }
    }

    pub async fn send_status(&mut self, packet_id: i32, packet_name: &str, status: Status) {
        self.send_data(packet_id, packet_name, serde_json::to_string(&status).unwrap().as_vec()).await;
    }

    pub async fn send_long(&mut self, packet_id: i32, packet_name: &str, data: Long) {
        self.send_data(packet_id, packet_name, data.value.to_ne_bytes().to_vec()).await;
    }

    pub async fn send_data(&mut self, packet_id: i32, packet_name: &str, response: Vec<u8>) {
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

        let write = self.socket.ready(Interest::WRITABLE).await.unwrap();
        if write.is_write_closed() {
            return
        }

        self.socket.write_all(end_data.as_mut()).await.unwrap();
        debug!("{:?} < {}", self.address, packet_name);
    }

    pub async fn handle(&mut self) {
        loop {
            let mut buffer = vec![0; 2097050];
            match self.socket.read(&mut buffer[..]).await {
                Ok(length) => {
                    buffer.resize(length, 0);
                    if buffer.len() > 0 {
                        let mut data = Cursor::new(buffer.clone());
                        let length = data.read_varint().await;
                        let packet_id = data.read_varint().await;

                        if buffer.len() > 0 {
                            let _ = buffer.remove(0);
                        }
                        if buffer.len() > 0 {
                            let _ = buffer.remove(0);
                        }
                        buffer.resize(length as usize, 0);
                        handle_data(self, Direction::SERVERBOUND, packet_id, buffer).await;
                    }
                },
                Err(_) => break,
            };

            let ready = self.socket.ready(Interest::READABLE).await.unwrap();
            if ready.is_read_closed() {
                info!("{}: {}", self.address, Red.paint("Disconnected"));
                break;
            }
        }
    }
}
