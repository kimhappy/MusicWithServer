use serde::{ Serialize, Deserialize };

#[derive(Clone, Serialize, Deserialize, PartialEq, PartialOrd, Eq, std::cmp::Ord)]
pub struct Hot {
    pub num_comments: usize,
    pub track_id    : String
}
