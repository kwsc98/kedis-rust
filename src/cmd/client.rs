use std::vec;

use crate::{frame::Frame, server::Handler};
pub struct Client {
    command : Vec<String>
}

impl Client {
    pub fn parse_frames(frame: Frame) -> crate::Result<Client> {
        let mut command = vec![];
        for idx in 0..frame.get_size(){
            command.push(
                frame
                .get_frame_by_index(idx)
                .ok_or("command 'client' err")?
                .to_string().to_lowercase());
        }
        return Ok(Client{command});
    }

    pub fn apply(&self, handler: &mut Handler) -> crate::Result<Frame> {
        if self.command[1] != "setname"  {
            return Err("not support command".into());
        }
        handler.set_handler_name(self.command[2].clone());
        return Ok(Frame::Bulk("OK".to_string().into()));
    }
}