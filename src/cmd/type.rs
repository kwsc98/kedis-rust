use crate::db::Db;
use crate::db::KedisKey;
use crate::frame::Frame;

pub struct Type {
    key: String,
}

impl Type {
    pub fn parse_frames(frame: Frame) -> crate::Result<Type> {
        let key = frame
            .get_frame_by_index(1)
            .ok_or("err command 'type'")?
            .to_string();
        return Ok(Type { key });
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
