use crate::command;
use crate::frame::Frame;

pub struct Unknown {
    command_name: String,
}

impl Unknown {
    pub fn parse_frames(frame: Frame) -> crate::Result<Unknown> {
        let command_name = command::get_command_name(&frame)?;
        return Ok(Unknown { command_name });
    }

    pub fn apply(&self) -> crate::Result<Frame> {
        let mut err_info = String::from("ERR unknown command '");
        err_info.push_str(&self.command_name);
        err_info.push_str("'");
        return Ok(Frame::Error(err_info));
    }
}
