use crate::cmd::client::Client;
use crate::cmd::config::Config;
use crate::cmd::string::get::Get;
use crate::cmd::ping::Ping;
use crate::cmd::quit::Quit;
use crate::cmd::r#type::Type;
use crate::cmd::scan::Scan;
use crate::cmd::select::Select;
use crate::cmd::string::set::Set;
use crate::cmd::ttl::Ttl;
use crate::cmd::unknown::Unknown;
use crate::frame::Frame;
use crate:: cmd::info::Info;
use crate::cmd::exists::Exists;
use crate::cmd::expire::Expire;

pub enum Command {
    Get(Get),
    Set(Set),
    Unknown(Unknown),
    Info(Info),
    Ping(Ping),
    Select(Select),
    Config(Config),
    Scan(Scan),
    Quit(Quit),
    Client(Client),
    Type(Type),
    Ttl(Ttl),
    Exists(Exists),
    Expire(Expire),
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
            "quit" => Command::Quit(Quit::parse_frames(frame)?),
            "client" => Command::Client(Client::parse_frames(frame)?),
            "type" => Command::Type(Type::parse_frames(frame)?),
            "ttl" => Command::Ttl(Ttl::parse_frames(frame)?),
            "exists" => Command::Exists(Exists::parse_frames(frame)?),
            "expire" => Command::Expire(Expire::parse_frames(frame)?),
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
