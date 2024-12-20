#![feature(let_chains, if_let_guard, binary_heap_into_iter_sorted)]

mod env;
mod route;
mod lyric;
mod chat;
mod state;

use std::sync::Arc;
use rocket::{ launch, routes, config::Config };
use route::*;
use state::State;

#[launch]
fn rocket() -> _ {
    dotenvy::dotenv().ok();

    let state = Arc::new(State::new(
        &*env::LYRICS_CACHE_DB,
        &*env::CHAT_HISTORY_DB));

    let config = Config {
        port   : *env::PORT   ,
        address: *env::ADDRESS,
        ..Config::default()
    };

    rocket::custom(config)
        .manage(state)
        .mount("/", routes![get_index, get_hot, get_lyrics, get_chat])
}
