#[rocket::get("/")]
pub fn get_index() -> &'static str {
    "Hello, MusicWith!"
}
