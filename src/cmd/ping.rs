use crate::frame::Frame;
pub struct Ping {}

impl Ping {
    pub fn parse_frames(_frame: Frame) -> crate::Result<Ping> {
        return Ok(Ping{});
    }

    pub fn apply(&self) -> crate::Result<Frame> {
        return Ok(Frame::Simple("PONG".to_string()));
    }
}