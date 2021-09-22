use std::fmt::{Display, Formatter, Result};

#[derive(Clone, Copy, Debug)]
pub struct VarInt {
    value: i32,
}

impl From<i32> for VarInt {
    fn from(item: i32) -> Self {
        VarInt { value: item }
    }
}


impl Display for VarInt {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Clone, Copy, Debug)]
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
        write!(f, "{}", self.value)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Long {
    pub value: i64,
}

impl From<i64> for Long {
    fn from(item: i64) -> Self {
        Long { value: item }
    }
}

impl Display for Long {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.value)
    }
}
