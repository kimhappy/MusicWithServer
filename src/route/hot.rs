use std::sync::Arc;
use crate::*;
use rocket::serde::json::Json;

#[rocket::get("/hot")]
pub async fn get_hot(server_state: &rocket::State< Arc< state::State > >) -> Result< Json< Vec< chat::Hot > >, String > {
    server_state.chat.hot(20)
        .map(Json)
        .ok_or("Failed to get hot chats".to_string())
}
