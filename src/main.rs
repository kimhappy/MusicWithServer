#![feature(anonymous_lifetime_in_impl_trait)]

use rocket::{ get, launch, response::Redirect, routes };
use serde::{ Serialize, Deserialize };

mod util;

lazy_static::lazy_static! {
    static ref CLIENT_ID          : String = util::env_var("SPOTIFY_CLIENT_ID"          );
    static ref CLIENT_SECRET      : String = util::env_var("SPOTIFY_CLIENT_SECRET"      );
    static ref REDIRECT_SERVER_URI: String = util::env_var("SPOTIFY_REDIRECT_SERVER_URI");
    static ref REDIRECT_APP_URI   : String = util::env_var("SPOTIFY_REDIRECT_APP_URI"   );
}

#[derive(Serialize, Deserialize)]
struct TokenResponse {
    access_token : String,
    token_type   : String,
    scope        : String,
    expires_in   : u64   ,
    refresh_token: String
}

#[get("/login")]
fn login() -> Result< Redirect, String > {
    let scope  = "user-read-private user-read-email";
    let state  = util::random_string(16);
    let params = [
        ("response_type", "code"              ),
        ("client_id"    , &CLIENT_ID          ),
        ("redirect_uri" , &REDIRECT_SERVER_URI),
        ("scope"        , scope               ),
        ("state"        , &state              )
    ];
    let qs           = serde_urlencoded::to_string(params).unwrap();
    let auth_url     = format!("https://accounts.spotify.com/authorize?{}", qs);
    Ok(Redirect::to(auth_url))
}

// TODO: Verify state
#[get("/callback?<code>&<state>")]
async fn callback(code: &str, state: &str) -> Result< Redirect, String > {
    let params              = [
        ("grant_type"  , "authorization_code"),
        ("code"        , code                ),
        ("redirect_uri", &REDIRECT_SERVER_URI)
    ];
    let auth_str       = base64::encode(format!("{}:{}", *CLIENT_ID, *CLIENT_SECRET));
    let client         = reqwest::Client::new();
    let token_response = client
        .post("https://accounts.spotify.com/api/token")
        .header("Authorization", format!("Basic {}", auth_str))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&params)
        .send()
        .await
        .map_err(|_| "Failed to get response from Spotify")?;

    if !token_response.status().is_success() {
        return Err(format!("{}: Failed to get token from Spotify", token_response.status()));
    }

    let token_data: TokenResponse = token_response
        .json()
        .await
        .unwrap();
    let params = [
        ("access_token" , &token_data.access_token ),
        ("refresh_token", &token_data.refresh_token)
    ];
    let qs           = serde_urlencoded::to_string(params).map_err(|_| "Failed to serialize token data")?;
    let redirect_uri = format!("{}/?{}", *REDIRECT_APP_URI, qs);
    Ok(Redirect::to(redirect_uri))
}

#[launch]
fn rocket() -> _ {
    dotenv::dotenv().ok();

    rocket::build()
        .mount("/", routes![login, callback])
}
