use crate::common::date_util::get_now_date_time_as_millis;
use crate::db::{Db, KedisKey};
use crate::frame::Frame;

pub struct Expire {
    key : String,
    ttl : i128
}

impl Expire {
    pub fn parse_frames(frame: Frame) -> crate::Result<Expire> {
        let key = frame
            .get_frame_by_index(1)
            .ok_or("err command 'expire'")?
            .to_string();
        let ttl = frame
            .get_frame_by_index(2)
            .ok_or("err command 'expire'")?
            .to_string();
        return Ok(Expire { key, ttl : ttl.parse()?});
    }

    pub fn apply(self, db: &mut Db) -> crate::Result<Frame> {
        let key = &KedisKey::new(self.key);
        let entry = db.get_mut_entry(key);
        let mut res = "0";
        if let Some(entry) = entry {
            let ttl = self.ttl;
            if ttl <= 0 {
                 db.remove(key);
            }else {
                 entry.key.set_ttl(ttl * 1000 + get_now_date_time_as_millis())
            }
            res = "1";
        }
        return Ok(Frame::Bulk(res.into()));
    }
}