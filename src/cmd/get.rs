use crate::buffer::Buffer;
use crate::command;
use crate::db::Db;
use crate::frame::Frame;

pub struct Get<'a> {
    key: &'a Frame,
}

impl <'a>Get<'a> {

    pub fn parse_frames(frame: &Frame) -> crate::Result<Get> {
        let key = command::get_frame_by_index(&frame, 1)?;
        return Ok(Get{key});
    }

    pub async fn apply(self, _db: &Db, _dst: &mut Buffer) -> crate::Result<()> {
        Ok(())
    }
}