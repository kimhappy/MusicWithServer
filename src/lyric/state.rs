use crate::env;
use super::Lyric;
use std::sync::Arc;
use polodb_core::{ Database, CollectionT, bson::doc };
use reqwest::{ Client, header, Url, cookie::Jar };
use serde_json::Value;

pub struct State {
    lyric_cache: Database
}

impl State {
    pub fn new(lyrics_cache_path: &str) -> Self {
        Self {
            lyric_cache: Database::open_path(lyrics_cache_path)
                .expect("Failed to open lyrics cache database")
        }
    }

    async fn get_access_token(client: &Client) -> Result< String, String > {
        const USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/101.0.4951.41 Safari/537.36";

        let jar = Arc::new(Jar::default());
        let url = "https://open.spotify.com".parse::< Url >().map_err(|_| "Failed to parse URL")?;
        jar.add_cookie_str(&format!("sp_dc={}", *env::SP_DC), &url);

        let mut headers = header::HeaderMap::new();
        headers.insert(header::USER_AGENT, header::HeaderValue::from_static(USER_AGENT ));
        headers.insert("app-platform"    , header::HeaderValue::from_static("WebPlayer"));

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

        jvalue
            .get("accessToken")
            .and_then(|v| v.as_str())
            .map(String::from)
            .ok_or("accessToken not found".to_string())
    }

    async fn isrc_to_spotify_id(
        client      : &Client,
        access_token: &str   ,
        isrc        : &str
    ) -> Result< String, String > {
        let search_url = format!("https://api.spotify.com/v1/search").parse::< Url >().map_err(|_| "Failed to parse URL")?;
        let params     = [("type", "track"), ("q", &format!("isrc:{}", isrc))];
        let resp = client.get(search_url)
            .header("authorization", format!("Bearer {}", access_token))
            .query(&params)
            .send()
            .await
            .map_err(|_| "Failed to send request")?;

        if !resp.status().is_success() {
            return Err("Failed to search track".to_string());
        }

        let jvalue = resp.json::< Value >()
            .await
            .map_err(|_| "Failed to parse JSON")?;

        jvalue.get("tracks")
            .ok_or("tracks not found")?
            .get("items")
            .ok_or("items not found")?
            .as_array()
            .ok_or("items is not an array")?
            .get(0)
            .ok_or("item not found")?
            .get("id")
            .ok_or("id not found")?
            .as_str()
            .ok_or("id is not a string")
            .map(String::from)
            .map_err(String::from)
    }

    async fn spotify_id_to_lyrics(
        client          : &Client,
        access_token    : &str,
        spotify_track_id: &str
    ) -> Result< Vec< Lyric >, String > {
            let track_url = format!("https://spclient.wg.spotify.com/color-lyrics/v2/track/{}", spotify_track_id);
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

            jvalue
                .get("lyrics")
                .ok_or("lyrics not found")?
                .get("lines")
                .ok_or("lines not found")?
                .as_array()
                .ok_or("lines is not an array")?
                .iter()
                .map(|line| -> Result< Lyric, String > {
                    let begin = line.get("startTimeMs")
                        .ok_or("startTimeMs not found")?
                        .as_str()
                        .ok_or("startTimeMs is not a string")?
                        .parse::< usize >()
                        .map_err(|_| "Failed to parse startTimeMs")
                        .map(|x| x as f64 / 1000.0)?;
                    let content = line.get("words")
                        .ok_or("words not found")?
                        .as_str()
                        .ok_or("words is not a string")?;
                    Ok(Lyric { begin, content: content.to_string() })
                }).collect::< Result< Vec< Lyric >, String > >()
    }

    async fn get_lyrics_impl(
        &self,
        isrc: &str
    ) -> Result< Vec< Lyric >, String > {
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

        let access_token = Self::get_access_token    (&client                            ).await?;
        let spotify_id   = Self::isrc_to_spotify_id  (&client, &access_token, isrc       ).await?;
        let lyrics       = Self::spotify_id_to_lyrics(&client, &access_token, &spotify_id).await?;
        Ok(lyrics)
    }

    pub async fn get_lyric(
        &self,
        isrc: &str
    ) -> Result< Vec< Lyric >, String > {
        let collection = self.lyric_cache.collection(isrc);

        if let Ok(cursor) = collection.find(doc! {}).sort(doc! { "begin": 1 }).run() &&
           let Ok(lyrics) = cursor.collect::< Result< Vec< _ >, _ > >()              &&
           !lyrics.is_empty() {
            return Ok(lyrics);
        }

        let lyrics = self.get_lyrics_impl(isrc).await?;
        collection.insert_many(lyrics.clone()).map_err(|_| "Failed to insert lyrics".to_string())?;
        Ok(lyrics)
    }
}
