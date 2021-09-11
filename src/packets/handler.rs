use crate::socket::SocketClient;

use async_trait::async_trait;
use std::io::Cursor;

#[async_trait]
pub trait PacketIncoming: Sync {
    async fn handle(_socket: &mut SocketClient, _data: &mut Cursor<Vec<u8>>) {}
}

#[macro_export]
macro_rules! packet_ids {
    ($($state:ident $stateName:ident {
       $($direction:ident $directionName:ident {
           $($id:expr => $packet:ident)*
       })+
    })+) => {
        pub async fn handle_data(client: &mut SocketClient, packet_direction: Direction, packet_id: i32, buffer: Vec<u8>) {
            trace!("{}: {:#04x} > {:?}", client.address, packet_id, buffer);
            let cursor = &mut Cursor::new(buffer);
            match client.state {
                $(State::$stateName => {
                    match packet_direction {
                        $(Direction::$directionName => {
                            match packet_id {
                                $(
                                    $id => {
                                        debug!("{}: {:#04x} > {}", client.address, packet_id, stringify!($packet));
                                        $packet::handle(client, cursor).await;
                                    },
                                )*
                                _ => panic!("bad packet 0x{:x} in {:?} {:?}", packet_id, packet_direction, client.state),
                            }
                        })*
                    }
                })*
            }
        }
    }
}
