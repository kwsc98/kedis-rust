use crate::db::Db;
use crate::db::KedisKey;
use crate::db::Structure;
use crate::frame::Frame;

pub struct Get {
    key: String,
}

impl Get {
    pub fn parse_frames(frame: Frame) -> crate::Result<Get> {
        let key = frame
            .get_frame_by_index(1)
            .ok_or("err command 'get'")?
            .to_string();
        return Ok(Get { key });
    }

    pub fn apply(self, db: &mut Db) -> crate::Result<Frame> {
        let value = db.get(&KedisKey::new(self.key));
        let frame = match value {
            Some(value) => match value {
                Structure::String(var) => Frame::Bulk(var.clone().into()),
                _ => Frame::Error(
                    "WRONGTYPE Operation against a key holding the wrong kind of value".into(),
                ),
            },
            None => Frame::Null,
        };
        return Ok(frame);
    }
}
