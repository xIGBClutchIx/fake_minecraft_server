use crate::{extensions::CursorExt, socket::{SocketClient, State}, packets::types::{Long, Short, VarInt}};

use async_trait::async_trait;
use std::{any::{Any}, io::Cursor, net::SocketAddr};


#[async_trait]
pub trait PacketServerbound: Sync {
    async fn handle(&self, _socket: &mut SocketClient) {}
}

#[async_trait]
pub trait PacketClientbound: Sync {

    fn get_data(&self, socket: &mut SocketClient) -> Vec<u8>;

    // TODO: Put packet id somehow on struct?!?
    async fn send(&self, id: i32, socket: &mut SocketClient) {
        let data = self.get_data(socket);
        socket.send_data(id, print_type_of(self), data).await;
    }
}

fn print_type_of<T: ?Sized>(_: &T) -> String {
    return format!("{}", std::any::type_name::<T>());
}

#[macro_export]
macro_rules! serverbound_packets {
    ($($stateName:ident {
        $($id:expr => $name:ident {
            $($field_name:ident: $field_type:ty, )*
        })*
    })+) => {
        $(
            $(
                #[derive(Debug)]
                pub struct $name {
                    $(
                        pub $field_name: $field_type,
                    )*
                }
            )*
        )+

        pub async fn handle_serverbound_data(client: &mut SocketClient, packet_id: i32, buffer: Vec<u8>) {
            trace!("{} > ({:?}) {:#04x}: {:?}", client.address, client.state, packet_id, buffer);
            let cursor = &mut Cursor::new(buffer);
            match client.state {
                $(State::$stateName => {
                    match packet_id {
                        $($id => {
                            debug!("{} > ({:?}) {:#04x}: {}", client.address, client.state, packet_id, stringify!($name).replace("Packet", ""));
                            let packet = $name {
                                $($field_name : get_field::<$field_type>(stringify!($field_type), client.address, cursor).await.downcast_ref::<$field_type>().unwrap().clone()),*
                            };
                            packet.handle(client).await;
                        },)*
                        _ => error!("{} > ({:?}) {:#04x}: Unknown packet", client.address, client.state, packet_id),
                    }
                })*
                _ => error!("{} > ({:?}) {:#04x}: Unknown packet state", client.address, client.state, packet_id),
            }
        }
    }
}

#[macro_export]
macro_rules! clientbound_packets {
    ($($stateName:ident {
        $($id:expr => $name:ident {
            $($field_name:ident: $field_type:ty, )*
        })*
    })+) => {
        $(
            $(
                #[derive(Debug)]
                pub struct $name {

                    $(
                        pub $field_name: $field_type,
                    )*
                }
            )*
        )+
    }
}

pub async fn get_field<T>(t: &str, address: SocketAddr, cursor: &mut Cursor<Vec<u8>>) -> Box<dyn Any + Sync + Send> {
    match t {
        "VarInt" => Box::new(VarInt::from(cursor.read_varint().await)),
        "String" => Box::new(cursor.read_string().await),
        "Short" => Box::new(Short::from(cursor.read_short().await)),
        "Long" => Box::new(Long::from(cursor.read_long().await)),
        "State" => Box::new(State::from_i32(cursor.read_varint().await)),
        _ => panic!("{}: {} > Unknown field type", address, t) // TODO: Don't do this ever this is bad make this not
    }
}
