use crate::{db::Db, frame::Frame};

//scan 0 MATCH * COUNT 500
pub struct Scan {
    start_idx: usize,
    match_str: String,
    count: usize,
}

impl Scan {
    pub fn parse_frames(frame: Frame) -> crate::Result<Scan> {
        if frame.get_size() < 6 {
            return Err("ERR command".into());
        }
        return Ok(Scan {
            start_idx: frame
                .get_frame_by_index(1)
                .ok_or("err")?
                .to_string()
                .parse::<usize>()?,
            match_str: frame.get_frame_by_index(3).ok_or("err")?.to_string(),
            count: frame
                .get_frame_by_index(5)
                .ok_or("err")?
                .to_string()
                .parse::<usize>()?,
        });
    }

    pub fn apply(&self, db: &mut Db) -> crate::Result<Frame> {
        let res = db.get_pattern_entry(
            self.start_idx,self.match_str.clone(),self.count
        );
        let mut idx = 0;
        let mut frame_list = vec![];
        return match res {
            Ok(item) => {
                idx = item.0;
                item.1.iter().for_each(|e| frame_list.push(Frame::Bulk(e.clone().into())));
                Ok(Frame::Array(vec![Frame::Bulk(idx.to_string().into()), Frame::Array(frame_list)]))
            }
            Err(err) => {
                Ok(Frame::Error(err.to_string()))
            }
        }
    }
}
