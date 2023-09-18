use crate::db::Db;
use crate::db::KedisKey;
use crate::frame::Frame;

pub struct Ttl {
    key: String,
}

impl Ttl {
    pub fn parse_frames(frame: Frame) -> crate::Result<Ttl> {
        let key = frame
            .get_frame_by_index(1)
            .ok_or("err command 'get'")?
            .to_string();
        return Ok(Ttl { key });
    }

    pub fn apply(self, db: &mut Db) -> crate::Result<Frame> {
        let entry = db.get_entry(&KedisKey::new(self.key));
        let ttl = match entry {
            Some(entry) => entry.key.get_ttl(),
            None => "-2".to_string(),
        };
        return Ok(Frame::Simple(ttl));
    }
}
