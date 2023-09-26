use crate::common::date_util::get_now_date_time_as_millis;
use crate::db::{Db, KedisKey};
use crate::frame::Frame;

pub struct Set {
    key: String,
    value: String,
    ttl : i128,
    pre : Option<String>
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
        let mut idx = 2;
        let mut ttl = -1;
        let mut pre = None;
        while idx < frame.get_size() {
            let str = &frame
                .get_frame_by_index(idx)
                .ok_or("command error 'set'")?
                .to_string().to_uppercase()[..];
            if str == "NX" || str == "XX" {
                pre = Some(str.to_string());
            }
            if str == "EX" || str == "PX" {
                let mut ttl_temp : i128 = frame.get_frame_by_index(idx + 1)
                    .ok_or("command error 'set'")?.to_string().parse()?;
                if str == "EX" {
                    ttl_temp = ttl_temp * 1000;
                }
                ttl = ttl_temp;
                idx += 1;
            }
            idx += 1;
        }
        return Ok(Set { key, value ,ttl, pre});
    }

    pub fn apply(self, db: &mut Db) -> crate::Result<Frame> {
        let mut key = KedisKey::new(self.key);
        if self.ttl > -1 {
            key.set_ttl(self.ttl + get_now_date_time_as_millis());
        }
        if let Some(pre) = self.pre {
            let entry = db.get_mut_entry(&key);
            if pre == "NX" && entry.is_some(){
                return Ok(Frame::Null);
            }
            if pre == "XX" && entry.is_none(){
                return Ok(Frame::Null);
            }
        }
        let _dict = db
            .insert(key, crate::db::Structure::String(self.value));
        return Ok(Frame::Simple("OK".into()));
    }
}
