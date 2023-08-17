use std::{io::Cursor};
use std::string::FromUtf8Error;

use bytes::Buf;

#[derive(Debug)]
pub enum Error {

    Incomplete,

    Other(crate::Error),
}


pub enum Frame {
    Simple(String),
    Error(String),
    Integer(u64),
    Bulk(String),
    Array(Vec<Frame>),
    Null
}

impl Frame {

}


#[allow(dead_code)]
fn pop_first_u8(src: &mut Cursor<&[u8]>) -> Result<u8, Error> {
    if !src.has_remaining() {
        return Err(Error::Incomplete);
    }
    return Ok(src.get_u8());
}
#[allow(dead_code)]
fn peek_first_u8(src: &mut Cursor<&[u8]>) -> Result<u8, Error> {
    if !src.has_remaining() {
        return Err(Error::Incomplete);
    }
    return Ok(src.chunk()[0]);
}
#[allow(dead_code)]
fn get_line(src: &mut Cursor<&[u8]>) -> Result<String, Error> {
    let start = src.position() as usize;
    let array = src.get_ref();
    let end = array.len() - 1;
    for i in start..end {
        if array[i] == b'\r' && array[i + 1] == b'\n' {
            src.set_position((i + 2) as u64);
            let str_vec = (&src.get_ref()[start..i]).to_vec();
            let string = String::from_utf8(str_vec)?;
            return Ok(string);
        }
    }
    return Err(Error::Incomplete)
}

impl From<String> for Error {
    fn from(src: String) -> Error {
        return Error::Other(src.into());
    }
}
impl From<&str> for Error {
    fn from(src: &str) -> Error {
        src.to_string().into()
    }
}

impl From<FromUtf8Error> for Error {
    fn from(_src: FromUtf8Error) -> Error {
        return "protocol error; invalid frame format".into();
    }
}