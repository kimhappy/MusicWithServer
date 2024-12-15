use crate::{ lyric, chat };

pub struct State {
    pub lyric: lyric::State,
    pub chat : chat ::State
}

impl State {
    pub fn new(
        lyric_cache_path : &str,
        chat_history_path: &str) -> Self {
        Self {
            lyric: lyric::State::new(lyric_cache_path ),
            chat : chat ::State::new(chat_history_path)
        }
    }
}
