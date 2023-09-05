use crate::db::Db;
use crate::frame::Frame;
use crate::command;

pub struct Set {
    _key: String,
    _value: String,
}

impl Set {
    pub fn parse_frames(frame: Frame) -> crate::Result<Set> {
        let _key = command::get_frame_by_index(&frame, 1)?.to_string();
        let _value = command::get_frame_by_index(&frame, 2)?.to_string();
        return Ok(Set { _key, _value });
    }

    pub fn apply(self, _db: &Db) -> Option<Frame> {
        Some(Frame::Null)
    }
}
