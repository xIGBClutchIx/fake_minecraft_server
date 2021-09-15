use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub struct VarInt {
    value: i32,
}

impl From<i32> for VarInt {
    fn from(item: i32) -> Self {
        VarInt { value: item }
    }
}

#[derive(Debug)]
pub struct Short {
    value: u16,
}

impl From<u16> for Short {
    fn from(item: u16) -> Self {
        Short { value: item }
    }
}

impl Display for Short {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
pub struct Long {
    value: u16,
}

impl From<u16> for Long {
    fn from(item: u16) -> Self {
        Long { value: item }
    }
}