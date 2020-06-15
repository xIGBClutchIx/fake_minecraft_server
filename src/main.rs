#[macro_use] extern crate log;

mod logger;
mod socket;

use std::net::{Ipv4Addr, SocketAddrV4};
use socket::SocketServer;

fn main() {
    logger::create_logger();

    let socket = SocketServer::new(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 25565));
    socket.listen();
}
