use crate::db::{Db, KedisKey};
use crate::frame::Frame;

pub struct Exists {
    key: String,
}

impl Exists {
    pub fn parse_frames(frame: Frame) -> crate::Result<Exists> {
        let key = frame
            .get_frame_by_index(1)
            .ok_or("err command 'exists'")?
            .to_string();
        return Ok(Exists { key });
    }

    pub fn apply(self, db: &mut Db) -> crate::Result<Frame> {
        let entry = db.get_entry(&KedisKey::new(self.key));
        let exists = match entry {
            Some(_entry) => "1",
            None => "0",
        };
        return Ok(Frame::Simple(exists.into()));
    }
}
