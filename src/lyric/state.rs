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

    async fn get_access_token(client: &Client) -> Option< String > {
        const USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/101.0.4951.41 Safari/537.36";

        let jar = Arc::new(Jar::default());
        let url = "https://open.spotify.com".parse::< Url >().ok()?;
        jar.add_cookie_str(&format!("sp_dc={}", *env::SP_DC), &url);

        let mut headers = header::HeaderMap::new();
        headers.insert(header::USER_AGENT, header::HeaderValue::from_static(USER_AGENT ));
        headers.insert("app-platform"    , header::HeaderValue::from_static("WebPlayer"));

        let resp = client.get("https://open.spotify.com/get_access_token?reason=transport&productType=web_player")
            .send()
            .await
            .ok()?;

        if !resp.status().is_success() {
            return None;
        }

        let jvalue = resp.json::< Value >()
            .await
            .ok()?;

        jvalue
            .get("accessToken")
            .and_then(|v| v.as_str())
            .map(String::from)
    }

    async fn isrc_to_spotify_ids(
        client      : &Client,
        access_token: &str   ,
        isrc        : &str
    ) -> Option< Vec< String > > {
        let search_url = format!("https://api.spotify.com/v1/search").parse::< Url >().ok()?;
        let params     = [("type", "track"), ("q", &format!("isrc:{}", isrc))];
        let resp = client.get(search_url)
            .header("authorization", format!("Bearer {}", access_token))
            .query(&params)
            .send()
            .await
            .ok()?;

        if !resp.status().is_success() {
            return None;
        }

        let jvalue = resp.json::< Value >()
            .await
            .ok()?;

        Some(jvalue.get("tracks")?
            .get("items")?
            .as_array()?
            .iter()
            .flat_map(|item| item.get("id"))
            .flat_map(|id| id.as_str())
            .map(String::from)
            .collect::< Vec< _ > >())
    }

    async fn spotify_id_to_lyrics(
        client          : &Client,
        access_token    : &str   ,
        spotify_track_id: &str
    ) -> Option< Vec< Lyric > > {
        let track_url = format!("https://spclient.wg.spotify.com/color-lyrics/v2/track/{}", spotify_track_id);
        let params    = [("format", "json"), ("market", "from_token")];

        let resp = client.get(&track_url)
            .header("authorization", format!("Bearer {}", access_token))
            .query(&params)
            .send()
            .await
            .ok()?;

        if !resp.status().is_success() {
            return None;
        }

        let jvalue = resp.json::< Value >()
            .await
            .ok()?;

        jvalue
            .get("lyrics")?
            .get("lines")?
            .as_array()?
            .iter()
            .map(|line| -> Option< Lyric > {
                let begin = line.get("startTimeMs")?
                    .as_str()?
                    .parse::< usize >()
                    .ok()? as f64 / 1000.0;
                let content = line.get("words")?
                    .as_str()?;
                Some(Lyric { begin, content: content.to_string() })
            }).collect::< Option< Vec< Lyric > > >()
    }

    async fn spotify_ids_to_lyrics< I: Iterator< Item = String > >(
        client           : &Client,
        access_token     : &str   ,
        spotify_track_ids: I
    ) -> Option< Vec< Lyric > > {
        for spotify_track_id in spotify_track_ids {
            let raw = Self::spotify_id_to_lyrics(client, access_token, &spotify_track_id).await;

            println!("raw: {:?}", raw);

            if let Some(lyrics) = raw {
                return Some(lyrics);
            }
        }

        None
    }

    async fn get_lyrics_impl(
        &self,
        isrc: &str
    ) -> Option< Vec< Lyric > > {
        const USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/101.0.4951.41 Safari/537.36";

        let jar = Arc::new(Jar::default());
        let url = "https://open.spotify.com".parse::< Url >().ok()?;
        jar.add_cookie_str(&format!("sp_dc={}", *env::SP_DC), &url);

        let mut headers = header::HeaderMap::new();
        headers.insert(header::USER_AGENT, header::HeaderValue::from_static(USER_AGENT ));
        headers.insert("app-platform"    , header::HeaderValue::from_static("WebPlayer"));

        let client = Client::builder()
            .cookie_provider(jar.clone())
            .default_headers(headers)
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .ok()?;

        let access_token = Self::get_access_token     (&client                                        ).await?;
        let spotify_ids  = Self::isrc_to_spotify_ids  (&client, &access_token, isrc                   ).await?;
        let lyrics       = Self::spotify_ids_to_lyrics(&client, &access_token, spotify_ids.into_iter()).await?;

        Some(lyrics)
    }

    pub async fn get_lyric(
        &self,
        isrc: &str
    ) -> Option< Vec< Lyric > > {
        let collection = self.lyric_cache.collection(isrc);

        if let Ok(cursor) = collection.find(doc! {}).sort(doc! { "begin": 1 }).run() &&
           let Ok(lyrics) = cursor.collect::< Result< Vec< _ >, _ > >()              &&
           !lyrics.is_empty() {
            return Some(lyrics);
        }

        let lyrics = self.get_lyrics_impl(isrc).await.unwrap_or(vec![]);
        collection.insert_many(lyrics.clone()).ok()?;
        Some(lyrics)
    }
}
