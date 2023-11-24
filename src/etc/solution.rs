use std::fmt::{Display, Formatter, Result};
use Solution::*;

#[derive(Debug, PartialEq)]
pub enum Solution {
    I32(i32),
    I64(i64),
    I128(i128),
    ISize(isize),
    U32(u32),
    U64(u64),
    U128(u128),
    USize(usize),
    Str(String),
}

impl Display for Solution {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            I32(x) => x.fmt(f),
            I64(x) => x.fmt(f),
            I128(x) => x.fmt(f),
            ISize(x) => x.fmt(f),
            U32(x) => x.fmt(f),
            U64(x) => x.fmt(f),
            U128(x) => x.fmt(f),
            USize(x) => x.fmt(f),
            Str(x) => x.fmt(f),
        }
    }
}

impl From<i32> for Solution {
    fn from(x: i32) -> Self {
        I32(x)
    }
}

impl From<i64> for Solution {
    fn from(x: i64) -> Self {
        I64(x)
    }
}

impl From<i128> for Solution {
    fn from(x: i128) -> Self {
        I128(x)
    }
}

impl From<isize> for Solution {
    fn from(x: isize) -> Self {
        ISize(x)
    }
}

impl From<u32> for Solution {
    fn from(x: u32) -> Self {
        U32(x)
    }
}

impl From<u64> for Solution {
    fn from(x: u64) -> Self {
        U64(x)
    }
}

impl From<u128> for Solution {
    fn from(x: u128) -> Self {
        U128(x)
    }
}

impl From<usize> for Solution {
    fn from(x: usize) -> Self {
        USize(x)
    }
}

impl From<&str> for Solution {
    fn from(x: &str) -> Self {
        Str(x.to_string())
    }
}

impl From<String> for Solution {
    fn from(x: String) -> Self {
        Str(x)
    }
}