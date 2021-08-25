use std::io::prelude::*;
use std::io::Cursor;
use std::str;

pub trait CursorExt {
    fn read_string(&mut self) -> String;

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
            Err(_) => format!(""),
        }
    }

    fn read_varint(&mut self) -> i32 {
        let mut num_read = 0;
        let mut result = 0;
        loop {
            let t_byte = self.length_read(1);
            let read = t_byte.first().expect("expect varint first byte");

            let value = read & 0b01111111;
            result |= i32::from(value) << (7 * num_read);
            num_read = num_read + 1;
            if num_read > 5 {
                break;
            }
            if read & 0b10000000 == 0 { 
                break;
            }
        } 
        return result;
    }

    fn length_read(&mut self, length: i32) -> Vec<u8> {
        let mut buf = vec![0; length as usize];
        if let Err(e) = self.read(&mut buf) {
            error!("{:?}", e);
        }
        return buf;
    }
}
