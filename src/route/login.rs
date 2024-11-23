use rocket::response::Redirect;
use crate::env;

#[rocket::get("/login?<state>")]
pub fn get_login(state: &str) -> Result< Redirect, String > {
    const SCOPE: &str = "user-read-private user-read-email";

    let params = [
        ("response_type", "code"                   ),
        ("client_id"    , &env::CLIENT_ID          ),
        ("redirect_uri" , &env::REDIRECT_SERVER_URI),
        ("scope"        , SCOPE                    ),
        ("state"        , state                    )
    ];
    let qs       = serde_urlencoded::to_string(params).map_err(|_| "Failed to serialize login data")?;
    let auth_url = format!("https://accounts.spotify.com/authorize?{}", qs);
    Ok(Redirect::to(auth_url))
}
