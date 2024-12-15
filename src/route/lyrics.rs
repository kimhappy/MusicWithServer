use std::sync::Arc;
use crate::{ state::State, lyric::Lyric };
use rocket::serde::json::Json;

#[rocket::get("/lyrics?<isrc>")]
pub async fn get_lyrics(
    isrc        : &str,
    server_state: &rocket::State< Arc< State > >) -> Result< Json< Vec< Lyric > >, String > {
    server_state.lyric.get_lyric(isrc)
        .await
        .map(Json)
        .map_err(|e| format!("Failed to get lyrics: {}", e))
}
