use crate::frame::Frame;
pub struct Quit {}

impl Quit {
    pub fn parse_frames(_frame: Frame) -> crate::Result<Quit> {
        return Ok(Quit{});
    }
}