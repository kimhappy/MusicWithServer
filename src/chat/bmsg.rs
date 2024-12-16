// Server -> Client messages

#![allow(non_camel_case_types)]

use serde::{ Serialize, Deserialize };

#[derive(Clone, Serialize, Deserialize)]
pub struct BJoin {
    pub user_id: String
}

#[derive(Clone, Serialize, Deserialize)]
pub struct BJoinResult {
    pub history: Vec< BChat   >,
    pub online : Vec< String >
}

#[derive(Clone, Serialize, Deserialize)]
pub struct BLeave {
    pub user_id: String
}

#[derive(Clone, Serialize, Deserialize)]
pub struct BChat {
    pub user_id : String          ,
    pub name    : String          ,
    pub chat_id : String          ,
    pub content : Option< String >,
    pub time    : Option< f64    >,
    pub reply_to: Option< String >
}

#[derive(Clone, Serialize, Deserialize)]
pub struct BDelete {
    pub chat_id: String
}

#[derive(Clone, Serialize, Deserialize)]
pub enum BMsg {
    join       (BJoin      ),
    join_result(BJoinResult),
    leave      (BLeave     ),
    chat       (BChat      ),
    delete     (BDelete    )
}
