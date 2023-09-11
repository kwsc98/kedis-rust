use crate::db::Db;
use crate::frame::Frame;

pub struct Get {
    key: String,
}

impl Get {

    pub fn parse_frames(frame: Frame) -> crate::Result<Get> {
        let key = frame.get_frame_by_index(1).ok_or("err")?.to_string();
        return Ok(Get{key});
    }

    pub fn apply(self, _db: &Db) -> crate::Result<Frame> {
        Ok(Frame::Null)
    }

}