use crate::{ auth, chat };

pub struct State {
    pub auth: auth::State,
    pub chat: chat::State
}

impl State {
    pub fn new() -> Self {
        Self {
            auth: auth::State::new(),
            chat: chat::State::new()
        }
    }
}
