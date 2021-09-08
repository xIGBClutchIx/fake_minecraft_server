#[macro_use] extern crate log;

mod extensions;
mod logger;
mod socket;
mod packets;

use log::LevelFilter;
use signal_hook::{consts::TERM_SIGNALS, flag};
use std::{
    net::{Ipv4Addr, SocketAddrV4},
    sync::{Arc, atomic::{AtomicBool, Ordering}},
};
use socket::*;

#[tokio::main]
async fn main() {
    // Start Logger
    logger::create_logger(LevelFilter::Trace);
    info!("Started");

    // Set the terminate variable to false so it allows the program to loop
    let terminate = Arc::new(AtomicBool::new(false));

    // Ask signal_hook to set the terminate variable to true when the program receives a shutdown signal
    for sig in TERM_SIGNALS {
        flag::register(*sig, Arc::clone(&terminate)).unwrap();
    }

    let socket = SocketServer::new(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 25565));

    // TCP Work
    tokio::spawn(async move {
        socket.listen().await;
    });

    // Do work until the terminate variable becomes true
    while !terminate.load(Ordering::Relaxed) {

    }

    // Shutdown time
    info!("Stopped");
}
