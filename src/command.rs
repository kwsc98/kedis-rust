use crate::buffer::Buffer;
use crate::cmd::get::Get;
use crate::cmd::set::Set;
use crate::cmd::unknown::Unknown;
use crate::db::Db;
use crate::frame::Frame;
use crate::shutdown::Shutdown;

pub enum Command<'a> {
    Get(Get<'a>),
    Set(Set<'a>),
    Unknown(Unknown),
}

impl<'a> Command<'a> {
    pub fn from_frame(frame: &Frame) -> crate::Result<Command> {
        let command_name = get_command_name(&frame)?.to_lowercase();
        let command = match &command_name[..] {
            "get" => Command::Get(Get::parse_frames(frame)?),
            "set" => Command::Set(Set::parse_frames(frame)?),
            _ => Command::Unknown(Unknown::parse_frames(frame)?)
        };
        return Ok(command);
    }
    pub(crate) async fn apply(
        self,
        _db: &Db,
        _dst: &mut Buffer,
        _shutdown: &mut Shutdown,
    ) {}
}

pub fn get_frame_by_index(frame: &Frame, index: usize) -> crate::Result<&Frame> {
    return if let Frame::Array(array) = frame {
        Ok(&array[index])
    } else {
        Ok(frame)
    };
}

pub fn get_command_name(frame: &Frame) -> crate::Result<String> {
    return match get_frame_by_index(frame, 0)? {
        Frame::Simple(str) => Ok(str.clone()),
        Frame::Bulk(str) => Ok(str.clone()),
        _ => Err("frame is error type".into()),
    };
}

