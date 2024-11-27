mod state;
pub mod client;
pub mod broad;

use serde::{ Serialize, Deserialize };
pub use state::State;

#[derive(Clone, Serialize, Deserialize)]
pub struct Delete {
    pub chat_id: String
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Join {
    pub user_id: String
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Leave {
    pub user_id: String
}
