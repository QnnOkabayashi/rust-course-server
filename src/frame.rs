use crate::error::ParseError;
use cursor::{byte, integer, line, size, slice};
use std::io::Cursor;
use std::str::from_utf8;

#[derive(Debug, PartialEq)]
pub enum Frame {
    Simple(String),
    // todo: fill in variants
}

impl Frame {
    pub fn check(src: &mut Cursor<&[u8]>) -> Result<(), ParseError> {
        todo!("implement Frame::check")
    }

    pub fn decode(src: &mut Cursor<&[u8]>) -> Result<Self, ParseError> {
        todo!("implement Frame::decode")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check() {
        let mut src = Cursor::new(":100\r\n".as_bytes());

        assert!(Frame::check(&mut src).is_ok());
    }

    // todo: write tests here
}

