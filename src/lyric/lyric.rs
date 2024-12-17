use serde::{ Serialize, Deserialize };

#[derive(Clone, Serialize, Deserialize)]
pub struct Lyric {
    pub begin  : f64,
    pub content: String
}
