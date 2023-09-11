use crate::frame::Frame;

pub struct Info {}

impl Info {

    pub fn parse_frames(_frame: Frame) -> crate::Result<Info> {
        return Ok(Info{});
    }

    pub fn apply(&self) -> crate::Result<Frame> {
        let mut info_str = String::new();
        info_str.push_str("redis_version:kedis-rust_1.0.0");
        info_str.push_str("\r\n");
        info_str.push_str("os:unknown");
        return Ok(Frame::Bulk(info_str.into()));
    }
    
}