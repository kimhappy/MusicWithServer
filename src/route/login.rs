use rocket::response::Redirect;
use crate::{ util, env };

#[rocket::get("/login")]
pub fn get_login() -> Result< Redirect, String > {
    let scope  = "user-read-private user-read-email";
    let state  = util::random_string(16);
    let params = [
        ("response_type", "code"                   ),
        ("client_id"    , &env::CLIENT_ID          ),
        ("redirect_uri" , &env::REDIRECT_SERVER_URI),
        ("scope"        , scope                    ),
        ("state"        , &state                   )
    ];
    let qs           = serde_urlencoded::to_string(params).unwrap();
    let auth_url     = format!("https://accounts.spotify.com/authorize?{}", qs);
    Ok(Redirect::to(auth_url))
}
