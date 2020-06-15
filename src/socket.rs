use std::{
    sync::{Arc, Mutex},
    io::Read,
    net::{SocketAddr, SocketAddrV4, TcpListener, TcpStream},
    thread::spawn
};

pub struct SocketServer {
    address: SocketAddrV4
}

impl SocketServer {
    pub fn new(address: SocketAddrV4) -> Self {
        Self {
            address
        }
    }

    fn handle_conn(&self, socket: TcpStream) {
        spawn(move || {
            let mut client = SocketClient::new(socket);
            info!("Connection: {}", client.address);
            client.handle()
        });
    }

    pub fn listen(&self) {
        let listener = TcpListener::bind(self.address).unwrap();
        info!("Listening : {}", self.address);

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => self.handle_conn(stream),
                Err(e) => error!("failed to handle stream: {}", e)
            };
        }
        drop(listener);
    }
}

pub struct SocketClient {
    address: SocketAddr,
    socket: Arc<Mutex<TcpStream>>
}

impl SocketClient {
    pub fn new(socket: TcpStream) -> Self {
        Self {
            address: socket.peer_addr().unwrap(),
            socket: Arc::new(Mutex::new(socket))
        }
    }

    pub fn handle(&mut self) {
        let mut sock = self.socket.lock().expect("failed to obtain socket");
        loop {
            let mut buf = [0; 10];
            match sock.peek(&mut buf) {
                Ok(length) => {
                    let mut buf = vec![0; length];
                    if let Err(e) = sock.read(&mut buf) {
                        error!("{:?}", e);
                        continue
                    }
                    if buf.len() > 0 {
                        debug!("{}: {:?}", self.address, &buf);
                    }
                },
                Err(e) => error!("failed to find length of data: {}", e)
            }
        }
    }
}
