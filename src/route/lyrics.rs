use std::sync::Arc;
use serde::{ Serialize, Deserialize };
use reqwest::{ Client, header, Url, cookie::Jar };
use serde_json::Value;
use rocket::serde::json::Json;
use crate::env;

#[derive(Clone, Serialize, Deserialize)]
pub struct LyricLine {
    pub begin  : usize,
    pub content: String
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Lyrics {
    pub lines: Vec< LyricLine >
}

#[rocket::get("/lyrics?<track_id>")]
pub async fn get_lyrics(track_id: &str) -> Result< Json< Lyrics >, String > {
    const USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/101.0.4951.41 Safari/537.36";

    let jar = Arc::new(Jar::default());
    let url = "https://open.spotify.com".parse::< Url >().map_err(|_| "Failed to parse URL")?;
    jar.add_cookie_str(&format!("sp_dc={}", *env::SP_DC), &url);

    let mut headers = header::HeaderMap::new();
    headers.insert(header::USER_AGENT, header::HeaderValue::from_static(USER_AGENT ));
    headers.insert("app-platform"    , header::HeaderValue::from_static("WebPlayer"));

    let client = Client::builder()
        .cookie_provider(jar.clone())
        .default_headers(headers)
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .map_err(|_| "Failed to create client")?;

    let resp = client.get("https://open.spotify.com/get_access_token?reason=transport&productType=web_player")
        .send()
        .await
        .map_err(|_| "Failed to send request")?;

    if !resp.status().is_success() {
        return Err("Failed to get access token".to_string());
    }

    let jvalue = resp.json::< Value >()
        .await
        .map_err(|_| "Failed to parse JSON")?;

    let access_token = jvalue
        .get("accessToken")
        .and_then(|v| v.as_str())
        .ok_or("accessToken not found")?;

    let track_url = format!("https://spclient.wg.spotify.com/color-lyrics/v2/track/{}", track_id);
    let params    = [("format", "json"), ("market", "from_token")];

    let resp = client.get(&track_url)
        .header("authorization", format!("Bearer {}", access_token))
        .query(&params)
        .send()
        .await
        .map_err(|_| "Failed to send request")?;

    if !resp.status().is_success() {
        return Err("Failed to get lyrics".to_string());
    }

    let jvalue = resp.json::< Value >()
        .await
        .map_err(|_| "Failed to parse JSON")?;

    let lines = jvalue
        .get("lyrics")
        .ok_or("lyrics not found")?
        .get("lines")
        .ok_or("lines not found")?
        .as_array()
        .ok_or("lines is not an array")?
        .iter()
        .map(|line| -> Result< LyricLine, String > {
            let begin = line.get("startTimeMs")
                .ok_or("startTimeMs not found")?
                .as_str()
                .ok_or("startTimeMs is not a string")?
                .parse::< usize >()
                .map_err(|_| "Failed to parse startTimeMs")?;
            let content = line.get("words")
                .ok_or("words not found")?
                .as_str()
                .ok_or("words is not a string")?;
            Ok(LyricLine { begin, content: content.to_string() })
        }).collect::< Result< Vec< LyricLine >, String > >();

    lines.map(|lines| Json(Lyrics { lines }))
}
