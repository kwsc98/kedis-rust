use crate::buffer::Buffer;
use crate::command;
use crate::db::Db;
use crate::frame::Frame;

pub struct Unknown {
    command_name: String,
}

impl Unknown {
    pub fn parse_frames(frame: &Frame) -> crate::Result<Unknown> {
        let command_name = command::get_command_name(&frame)?;
        return Ok(Unknown { command_name });
    }

    pub async fn apply(self, _db: &Db, _dst: &mut Buffer) -> crate::Result<()> {
        todo!()
    }
}
