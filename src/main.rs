mod env;
mod util;
mod route;
mod chat;

use std::sync::Arc;
use rocket::{ launch, routes };
use route::{ get_login, get_callback, get_chat };

#[launch]
fn rocket() -> _ {
    dotenvy::dotenv().ok();
    let chat_state = Arc::new(chat::State::new());

    rocket::build()
        .manage(chat_state)
        .mount("/", routes![get_login, get_callback, get_chat])
}
