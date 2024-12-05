mod env;
mod route;
mod auth;
mod chat;
mod state;

use std::sync::Arc;
use rocket::{ config::{ TlsConfig, CipherSuite, Config }, launch, routes };
use route::{ get_login, post_token, get_callback, get_lyrics, get_chat };
use state::State;

#[launch]
fn rocket() -> _ {
    dotenvy::dotenv().ok();
    let chat_state = Arc::new(State::new(&*env::CHAT_HISTORY_DB));

    let tls_config = TlsConfig::from_paths("ssl/certs.pem", "ssl/key.pem")
        .with_ciphers(CipherSuite::TLS_V13_SET)
        .with_preferred_server_cipher_order(true);

    let config = Config {
        tls: Some(tls_config),
        ..Default::default()
    };

    rocket::custom(config)
        .manage(chat_state)
        .mount("/", routes![get_login, post_token, get_callback, get_lyrics, get_chat])
}
