use crate::buffer::Buffer;
use crate::command;
use crate::db::Db;
use crate::frame::Frame;


pub struct Set<'a> {
    _key: &'a Frame,
    _value: &'a Frame,
}

impl <'a>Set<'a> {
    pub fn parse_frames(frame: &Frame) -> crate::Result<Set> {
        let _key = command::get_frame_by_index(&frame, 1)?;
        let _value = command::get_frame_by_index(&frame, 2)?;
        return Ok(Set { _key, _value });
    }

    pub async fn apply(self, _db: &Db, _dst: &mut Buffer) -> crate::Result<()> {
        todo!()
    }
}