use crate::command;
use crate::db::Db;
use crate::frame::Frame;

pub struct Unknown {
    _command_name: String,
}

impl Unknown {
    pub fn parse_frames(frame: Frame) -> crate::Result<Unknown> {
        let _command_name = command::get_command_name(&frame)?;
        return Ok(Unknown { _command_name });
    }

    pub fn apply(self, _db: &Db) -> Option<Frame> {
        Some(Frame::Integer(1))
    }
}
