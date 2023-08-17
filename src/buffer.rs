use std::io::Cursor;
use bytes::BytesMut;
use tokio::{io::BufWriter, net::TcpStream};
use crate::frame::Frame;

pub struct Buffer {

    _stream: BufWriter<TcpStream>,

    buffer: BytesMut,
    
}

impl Buffer {

    pub fn new(socket: TcpStream) -> Self {
        return Buffer {
            _stream: BufWriter::new(socket),
            buffer: BytesMut::with_capacity(4 * 1024),
        };
    }

    pub async fn read_frame(&mut self) -> crate::Result<Option<Frame>> {
        loop {
            //try get frame
            if let Some(frame) = self.parse_frame()? {
                return Ok(Some(frame));
            }
        }
    }

    fn parse_frame(&mut self) -> crate::Result<Option<Frame>> {
        let mut _buf = Cursor::new(&self.buffer[..]);
        Ok(None)
    }

}
