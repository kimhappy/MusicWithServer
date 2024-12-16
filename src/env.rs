use std::net::IpAddr;

fn load_env_var(name: &str) -> String {
    std::env::var(name).expect(&format!("{} not set", name))
}

lazy_static::lazy_static! {
    pub static ref PORT              : u16    = load_env_var("PORT"              ).parse().expect("PORT must be a number");
    pub static ref ADDRESS           : IpAddr = load_env_var("ADDRESS"           ).parse().expect("ADDRESS must be a valid IP address");
    pub static ref CHAT_HISTORY_DB   : String = load_env_var("CHAT_HISTORY_DB"   );
    pub static ref LYRICS_CACHE_DB   : String = load_env_var("LYRICS_CACHE_DB"   );
    pub static ref SP_DC             : String = load_env_var("SP_DC"             );
    pub static ref BROADCAST_CAPACITY: usize  = load_env_var("BROADCAST_CAPACITY").parse().expect("BROADCAST_CAPACITY must be a number");
}
