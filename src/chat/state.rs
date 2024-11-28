use rocket_ws as ws;
use tokio::sync::broadcast;
use dashmap::DashMap;
use polodb_core::{ Database, CollectionT, bson::{ Bson, doc } };
use super::{ client, broad, Delete, Join, Leave };
use crate::env;

pub struct State {
    channels : DashMap< String, broadcast::Sender< ws::Message > >,
    onlines  : DashMap< String, Vec              < String      > >,
    histories: Database
}

impl State {
    pub fn new(chat_history_path: &str) -> Self {
        Self {
            channels : DashMap::new(),
            onlines  : DashMap::new(),
            histories: Database::open_path(chat_history_path)
                .expect("Failed to open chat history database")
        }
    }

    pub fn sender(
        &self,
        track_id: &str
    ) -> Option< broadcast::Sender< ws::Message > > {
        Some(self.channels
            .entry(track_id.to_string())
            .or_insert_with(|| {
                let (sender, _) = broadcast::channel(*env::BROADCAST_CAPACITY);
                sender
            })
            .clone())
    }

    pub fn add_chat(
        &self         ,
        track_id: &str,
        user_id : &str,
        chat    : client::Chat
    ) -> Option< ws::Message > {
        let history = self.histories.collection(track_id);
        let bchat   = broad::Chat {
            user_id : user_id.to_string()             ,
            chat_id : uuid::Uuid::new_v4().to_string(),
            content : Some(chat.content)              ,
            time    : chat.time                       ,
            reply_to: chat.reply_to
        };
        let bmsg = broad::Msg::Chat(bchat.clone());
        let jmsg = serde_json::to_string(&bmsg).ok()?;
        history.insert_one(bchat).ok()?;
        Some(ws::Message::Text(jmsg))
    }

    pub fn delete_chat(
        &self         ,
        track_id: &str,
        user_id : &str,
        delete  : Delete
    ) -> Option< ws::Message > {
        let history = self.histories.collection::< client::Msg >(track_id);
        let bmsg    = broad::Msg::Delete(delete.clone());
        let jmsg    = serde_json::to_string(&bmsg).ok()?;
        let result  = history.update_one(doc! {
            "user_id": Bson::String(user_id.to_string()),
            "chat_id": Bson::String(delete.chat_id)
        }, doc! { "$set": {
            "content": None::< String >
        } }).ok()?;

        if result.modified_count == 0 {
            return None
        }

        Some(ws::Message::Text(jmsg))
    }

    pub fn add_online(
        &self         ,
        track_id: &str,
        user_id : &str
    ) -> Option< ws::Message > {
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

    pub fn remove_online(
        &self         ,
        track_id: &str,
        user_id : &str
    ) -> Option< ws::Message > {
        let mut online = self.onlines.entry(track_id.to_string()).or_insert_with(Vec::new);
        let     pos    = online.iter().position(|id| *id == user_id)?;
        let     bmsg   = broad::Msg::Leave(Leave {
            user_id: user_id.to_string()
        });
        let jmsg = serde_json::to_string(&bmsg).ok()?;
        online.remove(pos);
        Some(ws::Message::Text(jmsg))
    }

    pub fn get_history(
        &self         ,
        track_id: &str,
        _history: client::History
    ) -> Option< ws::Message > {
        let history = self.histories.collection(track_id);
        let found   = history.find(doc! {}).run().ok()?.into_iter().map(Result::ok).collect::< Option< Vec< _ > > >()?;
        let hmsg    = broad::Msg::History(broad::History {
            items: found
        });
        let jmsg = serde_json::to_string(&hmsg).ok()?;
        Some(ws::Message::Text(jmsg))
    }

    pub fn get_online(
        &self         ,
        track_id: &str,
        _online : client::Online
    ) -> Option< ws::Message > {
        let online = self.onlines.entry(track_id.to_string()).or_insert_with(Vec::new);
        let omsg   = broad::Msg::Online(broad::Online {
            items: online.clone()
        });
        let jmsg = serde_json::to_string(&omsg).ok()?;
        Some(ws::Message::Text(jmsg))
    }
}
