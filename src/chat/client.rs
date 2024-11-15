use serde::{ Serialize, Deserialize };
use super::Delete;

#[derive(Clone, Serialize, Deserialize)]
pub struct Chat {
    pub content : String,
    pub reply_to: Option< usize >
}

#[derive(Clone, Serialize, Deserialize)]
pub struct History {

}

#[derive(Clone, Serialize, Deserialize)]
pub struct Online {

}

#[derive(Clone, Serialize, Deserialize)]
pub enum Msg {
    Chat   (Chat   ),
    Delete (Delete ),
    History(History),
    Online (Online )
}
