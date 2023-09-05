use crate::command;
use crate::db::Db;
use crate::frame::Frame;

pub struct Get {
    _key: String,
}

impl Get {

    pub fn parse_frames(frame: Frame) -> crate::Result<Get> {
        let _key = command::get_frame_by_index(&frame, 1)?.to_string();
        return Ok(Get{_key});
    }

    pub fn apply(self, _db: &Db) -> Option<Frame> {
        Some(Frame::Null)
    }
}