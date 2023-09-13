use crate::frame::{Error, Frame};
use bytes::{Buf, BytesMut};
use std::io::{self, Cursor};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::{io::BufWriter, net::TcpStream};
use tracing::debug;

pub struct Buffer {
    stream: BufWriter<TcpStream>,

    buffer: BytesMut,
}

impl Buffer {
    pub fn new(socket: TcpStream) -> Self {
        return Buffer {
            stream: BufWriter::new(socket),
            buffer: BytesMut::with_capacity(4 * 1024),
        };
    }

    pub async fn read_frame(&mut self) -> crate::Result<Option<Frame>> {
        loop {
            if let Some(frame) = self.parse_frame()? {
                debug!("read frame [{:?}]", frame);
                return Ok(Some(frame));
            }
            if 0 == self.stream.read_buf(&mut self.buffer).await? {
                return if self.buffer.is_empty() {
                    Ok(None)
                } else {
                    Err("connection reset by peer".into())
                };
            }
        }
    }

    fn parse_frame(&mut self) -> crate::Result<Option<Frame>> {
        let mut buf = Cursor::new(&self.buffer[..]);
        return match Frame::parse(&mut buf) {
            Ok(frame) => {
                self.buffer.advance(buf.position() as usize);
                Ok(Some(frame))
            }
            Err(Error::Incomplete) => Ok(None),
            Err(Error::Other(e)) => Err(e.into()),
        };
    }

    pub async fn write_frame(&mut self, frame: &Frame) -> io::Result<()> {
        debug!("write frame [{:?}]", frame);
        match frame {
            Frame::Array(data) => {
                self.stream.write_u8(b'*').await?;
                self.stream
                    .write_all(data.len().to_string().as_bytes())
                    .await?;
                self.stream.write_all(b"\r\n").await?;
                for item in data {
                    self.write_value(item).await?;
                }
            }
            _ => self.write_value(frame).await?,
        }
        self.stream.flush().await
    }

    pub async fn write_value(&mut self, frame: &Frame) -> io::Result<()> {
        match frame {
            Frame::Simple(data) => {
                self.stream.write_u8(b'+').await?;
                self.stream.write_all(data.as_bytes()).await?;
                self.stream.write_all(b"\r\n").await?;
            }
            Frame::Error(data) => {
                self.stream.write_u8(b'-').await?;
                self.stream.write_all(data.as_bytes()).await?;
                self.stream.write_all(b"\r\n").await?;
            }
            Frame::Integer(data) => {
                self.stream.write_u8(b':').await?;
                self.stream.write_all(data.to_string().as_bytes()).await?;
                self.stream.write_all(b"\r\n").await?;
            }
            Frame::Bulk(data) => {
                self.stream.write_u8(b'$').await?;
                self.stream
                    .write_all(data.len().to_string().as_bytes())
                    .await?;
                self.stream.write_all(b"\r\n").await?;
                self.stream.write_all(data).await?;
                self.stream.write_all(b"\r\n").await?;
            }
            Frame::Null => {
                self.stream.write_all(b"$-1\r\n").await?;
            }
            Frame::Array(_) => {},
        }
        Ok(())
    }
}
