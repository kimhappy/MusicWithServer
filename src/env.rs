fn load_env_var(name: &str) -> String {
    std::env::var(name).expect(&format!("{} not set", name))
}

lazy_static::lazy_static! {
    pub static ref CHAT_HISTORY_DB    : String = load_env_var("CHAT_HISTORY_DB"            );
    pub static ref CLIENT_ID          : String = load_env_var("SPOTIFY_CLIENT_ID"          );
    pub static ref CLIENT_SECRET      : String = load_env_var("SPOTIFY_CLIENT_SECRET"      );
    pub static ref REDIRECT_SERVER_URI: String = load_env_var("SPOTIFY_REDIRECT_SERVER_URI");
    pub static ref REDIRECT_APP_URI   : String = load_env_var("SPOTIFY_REDIRECT_APP_URI"   );
    pub static ref SP_DC              : String = load_env_var("SP_DC"                      );
    pub static ref BROADCAST_CAPACITY : usize  = load_env_var("BROADCAST_CAPACITY"         ).parse().expect("BROADCAST_CAPACITY must be a number");
}
