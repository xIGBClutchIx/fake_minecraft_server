#[macro_use] extern crate log;

mod extensions;
mod logger;
mod packets;
mod socket;

use std::net::{Ipv4Addr, SocketAddrV4};
use socket::*;

fn main() {
    logger::create_logger();

    let socket = SocketServer::new(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 25565));
    socket.listen();
}
