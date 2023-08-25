pub mod server;
pub mod db;
pub mod buffer;
pub mod frame;
pub mod shutdown;
pub mod command;
pub mod cmd;
pub mod structure;


pub const DEFAULT_PORT: &str = "6379";

pub const MAX_CONNECTIONS: usize = 256;

pub type Error = Box<dyn std::error::Error + Send + Sync>;

pub type Result<T> = std::result::Result<T, Error>;


#[cfg(test)]
mod tests {
    use std::{collections::hash_map::RandomState, hash::{Hash, BuildHasher, Hasher}};


    #[test]
    fn exploration() {
       let index = hash(&1,2);
        let index2 = hash(&1,4);
        println!();
    }

    pub(crate) fn hash<K : Hash>( val: &K ,len : u64) -> u64
    {
        let mut hasher = RandomState::new().build_hasher();
        val.hash(&mut hasher);
        return hasher.finish() & len - 1;
    }
    

}

