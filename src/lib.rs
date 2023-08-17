pub mod server;
pub mod db;
pub mod buffer;
pub mod frame;
pub mod shutdown;
pub mod command;
pub mod cmd;


pub const DEFAULT_PORT: &str = "6379";

pub const MAX_CONNECTIONS: usize = 256;

pub type Error = Box<dyn std::error::Error + Send + Sync>;

pub type Result<T> = std::result::Result<T, Error>;

