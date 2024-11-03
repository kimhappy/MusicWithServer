#![feature(anonymous_lifetime_in_impl_trait)]

use rocket::{ get, launch, response::Redirect, routes };
use serde::{ Serialize, Deserialize };
use std::env;
use reqwest::Client;

mod util;

#[derive(Serialize, Deserialize)]
struct TokenResponse {
    access_token : String,
    token_type   : String,
    scope        : String,
    expires_in   : u64   ,
    refresh_token: String,
}

// TODO: Error handling
#[get("/login")]
fn login() -> Result< Redirect, String > {
    let client_id           = env::var("SPOTIFY_CLIENT_ID"          ).unwrap();
    let redirect_server_uri = env::var("SPOTIFY_REDIRECT_SERVER_URI").unwrap();
    let scope               = "user-read-private user-read-email";
    let state               = &util::random_string(16);
    let params              = [
        ("response_type", "code"              ),
        ("client_id"    , &client_id          ),
        ("redirect_uri" , &redirect_server_uri),
        ("scope"        , scope               ),
        ("state"        , state               )
    ];
    let qs           = serde_urlencoded::to_string(params).unwrap();
    let auth_url     = format!("https://accounts.spotify.com/authorize?{}", qs);
    Ok(Redirect::to(auth_url))
}

// TODO: Error handling
// TODO: Verify state
#[get("/callback?<code>&<state>")]
async fn callback(code: &str, state: &str) -> Result< Redirect, String > {
    let client_id           = env::var("SPOTIFY_CLIENT_ID"          ).unwrap();
    let client_secret       = env::var("SPOTIFY_CLIENT_SECRET"      ).unwrap();
    let redirect_server_uri = env::var("SPOTIFY_REDIRECT_SERVER_URI").unwrap();
    let redirect_app_uri    = env::var("SPOTIFY_REDIRECT_APP_URI"   ).unwrap();
    let params              = [
        ("grant_type"  , "authorization_code"),
        ("code"        , code                ),
        ("redirect_uri", &redirect_server_uri)
    ];
    let client         = Client::new();
    let token_response = client
        .post("https://accounts.spotify.com/api/token")
        .header("Authorization", format!("Basic {}", base64::encode(&format!("{}:{}", client_id, client_secret))))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&params)
        .send()
        .await
        .unwrap();

    if !token_response.status().is_success() {
        panic!("Failed to get token: {:?}", token_response);
    }

    let token_data: TokenResponse = token_response
        .json()
        .await
        .unwrap();
    let params = [
        ("access_token" , &token_data.access_token ),
        ("refresh_token", &token_data.refresh_token)
    ];
    let qs           = serde_urlencoded::to_string(params).unwrap();
    let redirect_uri = format!("{}/?{}", redirect_app_uri, qs);
    Ok(Redirect::to(redirect_uri))
}

#[launch]
fn rocket() -> _ {
    dotenv::dotenv().ok();

    rocket::build()
        .mount("/", routes![login, callback])
}
