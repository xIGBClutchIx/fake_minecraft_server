use async_trait::async_trait;
use tokio::io::AsyncReadExt;
use std::{io::Cursor, str::from_utf8};

pub trait StringExt {
    fn as_vec(&mut self) -> Vec<u8>;
}

impl StringExt for String {

    fn as_vec(&mut self) -> Vec<u8> {
        let string_data = self.clone().into_bytes();
        let mut data: Vec<u8> = Vec::new();
        // String Size
        data.add_varint(string_data.len() as i32);
        // String and size
        let end_data = [data, string_data].concat();
        return end_data;
    }
}

#[async_trait]
pub trait CursorExt {
    async fn read_string(&mut self) -> String;

    async fn read_short(&mut self) -> u16;
    async fn read_long(&mut self) -> i64;

    async fn read_varint(&mut self) -> i32;

    async fn length_read(&mut self, length: i32) -> Vec<u8>;
}

#[async_trait]
impl CursorExt for Cursor<Vec<u8>> {

    async fn read_string(&mut self) -> String {
        let length = self.read_varint().await;
        let string_bytes = self.length_read(length).await;
        let string = from_utf8(&string_bytes);

        return match string {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error reading string"),
        }
    }

    async fn read_short(&mut self) -> u16 {
        return self.read_u16().await.unwrap();
    }

    async fn read_long(&mut self) -> i64 {
        return self.read_i64().await.unwrap();
    }

    async fn read_varint(&mut self) -> i32 {
        let mut buf = [0];
        let mut ans = 0;
        for i in 0..4 {
            self.read_exact(&mut buf).await.expect("expect varint first byte");
            ans |= ((buf[0] & 0b0111_1111) as i32) << 7 * i;
            if buf[0] & 0b1000_0000 == 0 {
                break;
            }
        }
        return ans;
    }

    async fn length_read(&mut self, length: i32) -> Vec<u8> {
        let mut buf = vec![0; length as usize];
        self.read(&mut buf).await.expect("expect read with length");
        return buf;
    }
}

pub trait Vec8Ext {
    fn add_varint(&mut self, value: i32);
}

impl Vec8Ext for Vec<u8> {

    fn add_varint(&mut self, mut value: i32) {
        loop {
            let mut temp: u8 = (value & 0b01111111) as u8;

            value >>= 7;
            if value != 0 {
                temp |= 0b10000000;
            }

            self.push(temp);

            if value == 0 {
                return;
            }
        }
    }
}
