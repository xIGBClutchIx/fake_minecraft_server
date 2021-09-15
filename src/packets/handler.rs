use crate::socket::SocketClient;

use async_trait::async_trait;
use std::io::Cursor;

#[async_trait]
pub trait PacketIncoming: Sync {
    async fn handle(_socket: &mut SocketClient, _data: &mut Cursor<Vec<u8>>) {}
}

#[macro_export]
macro_rules! packet_ids {
    ($($stateName:ident {
        $($directionName:ident {
            $($id:expr => $name:ident {
                $($field:ident: $field_type:ty, )*
            })*
        })+
    })+) => {
        $(
            $(
                $(
                    #[derive(Debug)]
                    pub struct $name {
                        $($field: $field_type),*
                    }

                    #[async_trait::async_trait]
                    impl PacketIncoming for $name {
                        async fn handle(socket: &mut SocketClient, data: &mut Cursor<Vec<u8>>) {

                        }
                    }
                )*
            )+
        )+
    }
}
