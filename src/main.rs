#[macro_use] extern crate log;

mod extensions;
mod logger;
mod socket;
mod packets;

use std::{
    net::{Ipv4Addr, SocketAddrV4},
    thread::spawn,
};
use socket::*;

fn main() {
    logger::create_logger();

    spawn(move || {
        let socket = SocketServer::new(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 25565));
        socket.listen();
    });

    loop {

    }
}
