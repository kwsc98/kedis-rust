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
        let value = db.get(&KedisKey::new(self.key));
        let type_name = match value {
            Some(value) => value.get_type(),
            None => "none",
        };
        return Ok(Frame::Simple(type_name.into()));
    }
}
