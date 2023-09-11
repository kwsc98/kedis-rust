use crate::db::Db;
use crate::frame::Frame;

pub struct Set {
    _key: String,
    _value: String,
}

impl Set {
    pub fn parse_frames(frame: Frame) -> crate::Result<Set> {
        let _key = frame.get_frame_by_index(1).ok_or("command error")?.to_string();
        let _value = frame.get_frame_by_index(2).ok_or("command error")?.to_string();
        return Ok(Set { _key, _value });
    }

    pub fn apply(self, _db: &Db) -> crate::Result<Frame> {
        Ok(Frame::Null)
    }
}
