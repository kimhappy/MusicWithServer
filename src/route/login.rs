use rocket::response::Redirect;
use crate::{ util, env };

#[rocket::get("/login")]
pub fn get_login() -> Result< Redirect, String > {
    const SCOPE: &str = "user-read-private user-read-email";

    let state  = util::random_string(16);
    let params = [
        ("response_type", "code"                   ),
        ("client_id"    , &env::CLIENT_ID          ),
        ("redirect_uri" , &env::REDIRECT_SERVER_URI),
        ("scope"        , SCOPE                    ),
        ("state"        , &state                   )
    ];
    let qs       = serde_urlencoded::to_string(params).map_err(|_| "Failed to serialize login data")?;
    let auth_url = format!("https://accounts.spotify.com/authorize?{}", qs);
    Ok(Redirect::to(auth_url))
}
