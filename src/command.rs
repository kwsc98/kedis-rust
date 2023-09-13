use crate::cmd::config::Config;
use crate::cmd::get::Get;
use crate::cmd::ping::Ping;
use crate::cmd::scan::Scan;
use crate::cmd::select::Select;
use crate::cmd::set::Set;
use crate::cmd::unknown::Unknown;
use crate::frame::Frame;
use crate:: cmd::info::Info;

pub enum Command {
    Get(Get),
    Set(Set),
    Unknown(Unknown),
    Info(Info),
    Ping(Ping),
    Select(Select),
    Config(Config),
    Scan(Scan),
}

impl Command {
    pub fn from_frame(frame: Frame) -> crate::Result<Command> {
        let command_name = get_command_name(&frame)?.to_lowercase();
        let command = match &command_name[..] {
            "get" => Command::Get(Get::parse_frames(frame)?),
            "set" => Command::Set(Set::parse_frames(frame)?),
            "info" => Command::Info(Info::parse_frames(frame)?),
            "ping" => Command::Ping(Ping::parse_frames(frame)?),
            "select" => Command::Select(Select::parse_frames(frame)?),
            "scan" => Command::Scan(Scan::parse_frames(frame)?),
            "config" => Command::Config(Config::parse_frames(frame)?),
            _ => Command::Unknown(Unknown::parse_frames(frame)?),
        };
        return Ok(command);
    }
}

pub fn get_command_name(frame: &Frame) -> crate::Result<String> {
    return match frame.get_frame_by_index(0).ok_or("frame is empty")? {
        Frame::Simple(str) => Ok(str.clone()),
        Frame::Bulk(bytes) => {
            let str = std::str::from_utf8(&bytes[..])?;
            Ok(String::from(str))
        }
        _ => Err("frame is error type".into()),
    };
}
