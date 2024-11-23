use std::sync::Arc;
use crate::{ env, state::State };
use serde::{ Serialize, Deserialize };
use rocket::serde::json::Json;

#[derive(Serialize, Deserialize)]
pub struct TokenRequest {
    code      : Option< String >,
    session_id: Option< String >
}

#[derive(Serialize, Deserialize)]
pub struct SpotifyTokenResponse {
    access_token :         String,
    refresh_token: Option< String >
}

#[derive(Serialize, Deserialize)]
pub struct TokenResponse {
    access_token:         String,
    session_id  : Option< String >
}

#[rocket::post("/token", format = "json", data = "<token_request>")]
pub async fn post_token(
    token_request: Json< TokenRequest >,
    server_state : &rocket::State< Arc< State > >
) -> Result< Json< TokenResponse >, String > {
    async fn get_token_data< P: Serialize + ?Sized >(params: &P) -> Result< SpotifyTokenResponse, String > {
        let auth_str       = base64::encode(format!("{}:{}", *env::CLIENT_ID, *env::CLIENT_SECRET));
        let client         = reqwest::Client::new();
        let token_response = client
            .post("https://accounts.spotify.com/api/token")
            .header("Authorization", format!("Basic {}", auth_str))
            .header("Content-Type" , "application/x-www-form-urlencoded")
            .form(&params)
            .send()
            .await
            .map_err(|_| "Failed to get response from Spotify")?;

        if !token_response.status().is_success() {
            return Err(format!("{}: Failed to get token from Spotify", token_response.status()));
        }

        token_response
            .json()
            .await
            .map_err(|_| "Failed to parse token data".to_string())
    }

    if let Some(code) = &token_request.code {
        let params = [
            ("grant_type"  , "authorization_code"     ),
            ("code"        , code                     ),
            ("redirect_uri", &env::REDIRECT_SERVER_URI)
        ];
        let token_data    = get_token_data(&params).await?;
        let refresh_token = token_data.refresh_token.ok_or("No refresh token found")?;
        let session_id    = server_state.auth.make_session(&refresh_token);

        Ok(TokenResponse {
            access_token: token_data.access_token,
            session_id  : Some(session_id)
        })
    }
    else if let Some(session_id) = &token_request.session_id {
        if !server_state.auth.verify_session(session_id) {
            return Err("Invalid session_id".to_string());
        }

        let refresh_token = server_state.auth.get_refresh_token(session_id).ok_or("No refresh token found")?;
        server_state.auth.extend_session(session_id);

        let params = [
            ("grant_type"   , "refresh_token"),
            ("refresh_token", &refresh_token )
        ];
        let token_data = get_token_data(&params).await?;

        if let Some(refresh_token) = token_data.refresh_token {
            server_state.auth.update_refresh_token(session_id, &refresh_token);
        }

        Ok(TokenResponse {
            access_token: token_data.access_token,
            session_id  : None
        })
    }
    else {
        Err("No code or session_id provided".to_string())
    }
    .map(Json)
}
