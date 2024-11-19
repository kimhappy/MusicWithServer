use rocket_ws as ws;
use tokio::sync::broadcast;
use dashmap::DashMap;
use super::{ client, broad, Delete, Join, Leave };
use crate::env;

pub struct State {
    channels : DashMap< String, broadcast::Sender< ws::Message > >,
    histories: DashMap< String, Vec              < broad::Chat > >,
    onlines  : DashMap< String, Vec              < String      > >
}

impl State {
    pub fn new() -> Self {
        Self {
            channels : DashMap::new(),
            histories: DashMap::new(),
            onlines  : DashMap::new()
        }
    }

    pub fn sender(&self, track_id: &str) -> Option< broadcast::Sender< ws::Message > > {
        Some(self.channels
            .entry(track_id.to_string())
            .or_insert_with(|| {
                let (sender, _) = broadcast::channel(*env::BROADCAST_CAPACITY);
                sender
            })
            .clone())
    }

    pub fn add_chat(&self, track_id: &str, user_id: &str, chat: client::Chat) -> Option< ws::Message > {
        let mut history = self.histories.entry(track_id.to_string()).or_insert_with(Vec::new);
        let     bchat   = broad::Chat {
            user_id : user_id.to_string(),
            chat_id : history.len      (),
            content : Some(chat.content) ,
            time    : chat.time          ,
            reply_to: chat.reply_to
        };
        let bmsg = broad::Msg::Chat(bchat.clone());
        let jmsg = serde_json::to_string(&bmsg).ok()?;
        history.push(bchat);
        Some(ws::Message::Text(jmsg))
    }

    pub fn delete_chat(&self, track_id: &str, user_id: &str, delete: Delete) -> Option< ws::Message > {
        let mut history = self.histories.entry(track_id.to_string()).or_insert_with(Vec::new);
        let     chat    = history.iter_mut().find(|c| c.chat_id == delete.chat_id)?;

        if chat.user_id != user_id || chat.content.is_none() {
            return None
        }

        let bmsg     = broad::Msg::Delete(delete.clone());
        let jmsg     = serde_json::to_string(&bmsg).ok()?;
        chat.content = None;
        Some(ws::Message::Text(jmsg))
    }

    pub fn add_online(&self, track_id: &str, user_id: &str) -> Option< ws::Message > {
        let mut online = self.onlines.entry(track_id.to_string()).or_insert_with(Vec::new);

        if online.iter().any(|id| id == user_id) {
            return None
        }

        let bmsg = broad::Msg::Join(Join {
            user_id: user_id.to_string()
        });
        let jmsg = serde_json::to_string(&bmsg).ok()?;
        online.push(user_id.to_string());
        Some(ws::Message::Text(jmsg))
    }

    pub fn remove_online(&self, track_id: &str, user_id: &str) -> Option< ws::Message > {
        let mut online = self.onlines.entry(track_id.to_string()).or_insert_with(Vec::new);
        let     pos    = online.iter().position(|id| *id == user_id)?;
        let     bmsg   = broad::Msg::Leave(Leave {
            user_id: user_id.to_string()
        });
        let jmsg = serde_json::to_string(&bmsg).ok()?;
        online.remove(pos);
        Some(ws::Message::Text(jmsg))
    }

    pub fn get_history(&self, track_id: &str, _history: client::History) -> Option< ws::Message > {
        let history = self.histories.entry(track_id.to_string()).or_insert_with(Vec::new);
        let hmsg    = broad::Msg::History(broad::History {
            items: history.clone()
        });
        let jmsg = serde_json::to_string(&hmsg).ok()?;
        Some(ws::Message::Text(jmsg))
    }

    pub fn get_online(&self, track_id: &str, _online: client::Online) -> Option< ws::Message > {
        let online = self.onlines.entry(track_id.to_string()).or_insert_with(Vec::new);
        let omsg   = broad::Msg::Online(broad::Online {
            items: online.clone()
        });
        let jmsg = serde_json::to_string(&omsg).ok()?;
        Some(ws::Message::Text(jmsg))
    }
}
