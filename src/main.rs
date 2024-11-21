mod env;
mod util;
mod route;
mod auth;
mod chat;
mod state;

use std::sync::Arc;
use rocket::{ launch, routes };
use route::{ get_login, get_callback, get_lyrics, get_chat };
use state::State;

#[launch]
fn rocket() -> _ {
    dotenvy::dotenv().ok();
    let chat_state = Arc::new(State::new());

    rocket::build()
        .manage(chat_state)
        .mount("/", routes![get_login, get_callback, get_lyrics, get_chat])
}
