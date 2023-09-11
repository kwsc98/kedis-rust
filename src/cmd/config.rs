use std::vec;

use crate::{frame::Frame, server::Handler};

pub struct Config {
    config_type: ConfigType,
}

enum ConfigType {
    Get(Vec<String>),
    Unknown(String),
}

impl Config {
    pub fn parse_frames(frame: Frame) -> crate::Result<Config> {
        let mut commands = vec![];
        for idx in 0..frame.get_size() {
            commands.push(frame.get_frame_by_index(idx).ok_or("err")?.to_string());
        }
        let config_type = match &commands.get(1).ok_or("err")?[..] {
            "get" => ConfigType::Get(commands),
            unknown_config_type => ConfigType::Unknown(
                format!(
                    "ERR command config {} not support for normal user",
                    unknown_config_type
                )
                .into(),
            ),
        };
        return Ok(Config { config_type });
    }

    pub fn apply(&self, handler: &mut Handler) -> crate::Result<Frame> {
        return match &self.config_type {
            ConfigType::Get(commands) => ConfigType::get_apply(commands, handler),
            ConfigType::Unknown(config_type) => Ok(Frame::Error(config_type.to_string())),
        };
    }
}

impl ConfigType {
    fn get_apply(commands: &Vec<String>, handler: &mut Handler) -> crate::Result<Frame> {
        return match &commands.get(2).ok_or("err")?[..] {
            "databases" => {
                let res_frame = vec![
                    Frame::Bulk("databases".into()),
                    Frame::Bulk(handler.get_db_size().to_string().into()),
                ];
                Ok(Frame::Array(res_frame))
            }
            _ => Ok(Frame::Error("ERR command".to_string())),
        };
    }
}
