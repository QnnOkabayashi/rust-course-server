use cursor::CursorError;
use std::str::Utf8Error;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    Cursor(CursorError),
    FrameType(u8),
    // add more
}

impl From<CursorError> for ParseError {
    fn from(err: CursorError) -> Self {
        Self::Cursor(err)
    }
}

