use crate::{ auth, chat };

pub struct State {
    pub auth: auth::State,
    pub chat: chat::State
}

impl State {
    pub fn new(chat_history_path: &str) -> Self {
        Self {
            auth: auth::State::new(),
            chat: chat::State::new(chat_history_path)
        }
    }
}
