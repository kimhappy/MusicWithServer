// Client -> Server messages

#![allow(non_camel_case_types)]

use serde::{ Serialize, Deserialize };

#[derive(Clone, Serialize, Deserialize)]
pub struct AJoin {
    pub user_id: String,
    pub name   : String
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AChat {
    pub content : String          ,
    pub time    : Option< f64    >,
    pub reply_to: Option< String >
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ADelete {
    pub chat_id: String
}

#[derive(Clone, Serialize, Deserialize)]
pub enum AMsg {
    join  (AJoin  ),
    chat  (AChat  ),
    delete(ADelete)
}
