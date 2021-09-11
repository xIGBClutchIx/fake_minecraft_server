use std::{io::{Cursor, prelude::*}, str};
use byteorder::{BigEndian, ReadBytesExt};

pub trait CursorExt {
    fn read_string(&mut self) -> String;

    fn read_short(&mut self) -> u16;
    fn read_long(&mut self) -> i64;

    fn read_varint(&mut self) -> i32;

    fn length_read(&mut self, length: i32) -> Vec<u8>;
}

impl CursorExt for Cursor<Vec<u8>> {

    fn read_string(&mut self) -> String {
        let length = self.read_varint();
        let string_bytes = self.length_read(length);
        let string = str::from_utf8(&string_bytes);

        return match string {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error reading string"),
        }
    }

    fn read_short(&mut self) -> u16 {
        let short = self.read_u16::<BigEndian>();

        return match short {
            Ok(v) => v,
            Err(_) => 0
        }
    }

    fn read_long(&mut self) -> i64 {
        let long = self.read_i64::<BigEndian>();

        return match long {
            Ok(v) => v,
            Err(_) => 0
        }
    }

    fn read_varint(&mut self) -> i32 {
        let mut buf = [0];
        let mut ans = 0;
        for i in 0..4 {
            self.read_exact(&mut buf).expect("expect varint first byte");
            ans |= ((buf[0] & 0b0111_1111) as i32) << 7 * i;
            if buf[0] & 0b1000_0000 == 0 {
                break;
            }
        }
        return ans;
    }

    fn length_read(&mut self, length: i32) -> Vec<u8> {
        let mut buf = vec![0; length as usize];
        if let Err(e) = self.read(&mut buf) {
            error!("{:?}", e);
        }
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
