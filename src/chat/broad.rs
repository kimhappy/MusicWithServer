use serde::{ Serialize, Deserialize };
use super::{ Delete, Join, Leave };

#[derive(Clone, Serialize, Deserialize)]
pub struct Chat {
    pub user_id : String          ,
    pub chat_id : usize           ,
    pub content : Option< String >,
    pub reply_to: Option< usize  >
}

#[derive(Clone, Serialize, Deserialize)]
pub struct History {
    pub items: Vec< Chat >
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Online {
    pub items: Vec< String >
}

#[derive(Clone, Serialize, Deserialize)]
pub enum Msg {
    Chat   (Chat   ),
    Delete (Delete ),
    Join   (Join   ),
    Leave  (Leave  ),
    History(History),
    Online (Online )
}