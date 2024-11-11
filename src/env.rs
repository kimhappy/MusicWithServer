fn load_env_var(name: &str) -> String {
    std::env::var(name).expect(&format!("{} not set", name))
}

lazy_static::lazy_static! {
    pub static ref CLIENT_ID          : String = load_env_var("SPOTIFY_CLIENT_ID"          );
    pub static ref CLIENT_SECRET      : String = load_env_var("SPOTIFY_CLIENT_SECRET"      );
    pub static ref REDIRECT_SERVER_URI: String = load_env_var("SPOTIFY_REDIRECT_SERVER_URI");
    pub static ref REDIRECT_APP_URI   : String = load_env_var("SPOTIFY_REDIRECT_APP_URI"   );
}
