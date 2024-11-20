use rocket::response::Redirect;
use serde::{ Serialize, Deserialize };
use crate::env;

#[derive(Serialize, Deserialize)]
struct TokenResponse {
    access_token : String,
    refresh_token: String
}

// TODO: Verify state
#[rocket::get("/callback?<code>&<state>")]
pub async fn get_callback(code: &str, state: &str) -> Result< Redirect, String > {
    let params              = [
        ("grant_type"  , "authorization_code"     ),
        ("code"        , code                     ),
        ("redirect_uri", &env::REDIRECT_SERVER_URI)
    ];
    let auth_str       = base64::encode(format!("{}:{}", *env::CLIENT_ID, *env::CLIENT_SECRET));
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
        .map_err(|_| "Failed to parse token data")?;
    let params = [
        ("access_token" , &token_data.access_token ),
        ("refresh_token", &token_data.refresh_token)
    ];
    let qs           = serde_urlencoded::to_string(params).map_err(|_| "Failed to serialize token data")?;
    let redirect_uri = format!("{}/?{}", *env::REDIRECT_APP_URI, qs);
    Ok(Redirect::to(redirect_uri))
}
