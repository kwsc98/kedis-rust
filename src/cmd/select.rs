use crate::{frame::Frame, server::Handler};
pub struct Select {
    db_idx: usize,
}

impl Select {
    pub fn parse_frames(frame: Frame) -> crate::Result<Select> {
        let db_idx = frame
            .get_frame_by_index(1)
            .ok_or_else(|| "command error")?
            .to_string()
            .parse::<usize>()?;
        return Ok(Select { db_idx });
    }

    pub fn apply(&self, handler: &mut Handler) -> crate::Result<Frame> {
        handler.change_db_sender(self.db_idx)?;
        return Ok(Frame::Simple("OK".to_string()));
    }
}
