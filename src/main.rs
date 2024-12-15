#![feature(let_chains)]

mod env;
mod route;
mod lyric;
mod chat;
mod state;

use std::sync::Arc;
use rocket::{ launch, routes };
use route::{ get_lyrics, get_chat };
use state::State;

#[launch]
fn rocket() -> _ {
    dotenvy::dotenv().ok();
    let state = Arc::new(State::new(
        &*env::LYRICS_CACHE_DB,
        &*env::CHAT_HISTORY_DB));

    rocket::build()
        .manage(state)
        .mount("/", routes![get_lyrics, get_chat])
}
