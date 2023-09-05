use crate::frame::Error::Other;
use bytes::{Buf, Bytes};
use std::io::Cursor;
use std::num::TryFromIntError;
use std::str::FromStr;
use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum Error {
    Incomplete,

    Other(crate::Error),
}

#[derive(Debug)]
pub enum Frame {
    Simple(String),
    Error(String),
    Integer(u64),
    Bulk(Bytes),
    Array(Vec<Frame>),
    Null,
}

impl Frame {
    pub fn parse(src: &mut Cursor<&[u8]>) -> Result<Frame, Error> {
        return match pop_first_u8(src)? {
            b'+' => Ok(Frame::Simple(get_line(src)?)),
            b'-' => Ok(Frame::Simple(get_line(src)?)),
            b':' => Ok(Frame::Integer(get_decimal(src)?)),
            b'$' => {
                if b'-' == peek_first_u8(src)? {
                    let line = get_line(src)?;
                    if line != "-1" {
                        return Err("protocol error; invalid frame format".into());
                    }
                    Ok(Frame::Null)
                } else {
                    let len: usize = get_decimal(src)?.try_into()?;
                    if src.remaining() < len + 2 {
                        return Err(Error::Incomplete);
                    }
                    let bytes = Bytes::copy_from_slice(&src.chunk()[..len]);
                    skip(src, len + 2)?;
                    Ok(Frame::Bulk(bytes))
                }
            }
            b'*' => {
                if b'-' == peek_first_u8(src)? {
                    let line = get_line(src)?;
                    if line != "-1" {
                        return Err("protocol error; invalid frame format".into());
                    }
                    Ok(Frame::Null)
                } else {
                    let len: usize = get_decimal(src)?.try_into()?;
                    let mut frame_array = Vec::with_capacity(len);
                    for _ in 0..len {
                        frame_array.push(Frame::parse(src)?);
                    }
                    Ok(Frame::Array(frame_array))
                }
            }
            actual => Err(format!("protocol error; invalid frame type byte `{}`", actual).into()),
        };
    }

    pub fn to_string(&self) -> String {
        return match self {
            Frame::Simple(simple) => simple.to_string(),
            Frame::Error(error) => error.to_string(),
            Frame::Integer(integer) => integer.to_string(),
            Frame::Bulk(bulk) => std::str::from_utf8(bulk.chunk()).unwrap().to_string(),
            Frame::Array(array) => {
                let mut temp = String::from("[");
                for item in array.iter().enumerate() {
                    temp.push_str(&item.1.to_string()[..]);
                    if item.0 < array.len() - 1 {
                        temp.push_str("||");
                    }
                }
                temp.push_str("]");
                temp
            }
            Frame::Null => String::from("NULL"),
        };
    }
}

fn pop_first_u8(src: &mut Cursor<&[u8]>) -> Result<u8, Error> {
    if !src.has_remaining() {
        return Err(Error::Incomplete);
    }
    return Ok(src.get_u8());
}

fn peek_first_u8(src: &Cursor<&[u8]>) -> Result<u8, Error> {
    if !src.has_remaining() {
        return Err(Error::Incomplete);
    }
    return Ok(src.chunk()[0]);
}

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
    return Err(Error::Incomplete);
}

fn get_decimal(src: &mut Cursor<&[u8]>) -> Result<u64, Error> {
    let str = get_line(src)?;
    return match str.parse::<u64>() {
        Ok(u64) => Ok(u64),
        Err(_) => Err(Other("convert decimal error".into())),
    };
}

fn skip(src: &mut Cursor<&[u8]>, n: usize) -> Result<(), Error> {
    if src.remaining() < n {
        return Err(Error::Incomplete);
    }
    src.advance(n);
    Ok(())
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

impl From<TryFromIntError> for Error {
    fn from(_src: TryFromIntError) -> Error {
        "protocol error; invalid frame format".into()
    }
}
