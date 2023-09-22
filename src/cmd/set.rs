use crate::db::{Db, KedisKey};
use crate::frame::Frame;

pub struct Set {
    key: String,
    value: String,
}

impl Set {
    pub fn parse_frames(frame: Frame) -> crate::Result<Set> {
        let key = frame
            .get_frame_by_index(1)
            .ok_or("command error 'set'")?
            .to_string();
        let value = frame
            .get_frame_by_index(2)
            .ok_or("command error 'set'")?
            .to_string();
        return Ok(Set { key, value });
    }

    pub fn apply(self, db: &mut Db) -> crate::Result<Frame> {
        let _dict = db
            .insert(KedisKey::new(self.key), crate::db::Structure::String(self.value));
        return Ok(Frame::Simple("OK".into()));
    }
}
