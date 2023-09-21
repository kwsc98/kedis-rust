use crate::common::date_util;
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
        let mut kedis_key = KedisKey::new(self.key);
        kedis_key.set_ttl(1000000 + date_util::get_now_date_time_as_millis());
        let _dict = db
            .insert(kedis_key, crate::db::Structure::String(self.value));
        return Ok(Frame::Simple("OK".into()));
    }
}
