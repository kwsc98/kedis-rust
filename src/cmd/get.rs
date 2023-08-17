use crate::buffer::Buffer;
use crate::command;
use crate::db::Db;
use crate::frame::Frame;

pub struct Get<'a> {
    _key: &'a Frame,
}

impl <'a>Get<'a> {

    pub fn parse_frames(frame: &Frame) -> crate::Result<Get> {
        let _key = command::get_frame_by_index(&frame, 1)?;
        return Ok(Get{_key});
    }

    pub async fn apply(self, _db: &Db, _dst: &mut Buffer) -> crate::Result<()> {
        Ok(())
    }
}