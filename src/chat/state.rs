use std::collections::BinaryHeap;
use rocket_ws as ws;
use tokio::sync::broadcast;
use dashmap::DashMap;
use polodb_core::{ Database, CollectionT, bson::{ Bson, doc } };
use super::*;
use crate::env;

pub struct State {
    channels : DashMap< String, broadcast::Sender< ws::Message    > >,
    onlines  : DashMap< String, DashMap          < String, String > >,
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
        &self           ,
        my_join : &AJoin,
        track_id: &str  ,
        cchat   : AChat
    ) -> Option< ws::Message > {
        let history = self.histories.collection(track_id);
        let bchat   = BChat {
            user_id : my_join.user_id.clone()         ,
            name    : my_join.name   .clone()         ,
            chat_id : uuid::Uuid::new_v4().to_string(),
            content : Some(cchat.content)             ,
            time    : cchat.time                      ,
            reply_to: cchat.reply_to
        };
        let bmsg    = BMsg::chat(bchat.clone());
        let jmsg    = serde_json::to_string(&bmsg).ok()?;
        history.insert_one(bchat).ok()?;
        Some(ws::Message::Text(jmsg))
    }

    pub fn delete_chat(
        &self           ,
        my_join : &AJoin,
        track_id: &str  ,
        delete  : ADelete
    ) -> Option< ws::Message > {
        let history = self.histories.collection::< AMsg >(track_id);
        let bmsg    = BMsg::delete(BDelete {
            chat_id: delete.chat_id.clone()
        });
        let jmsg    = serde_json::to_string(&bmsg).ok()?;
        let result  = history.update_one(doc! {
            "user_id": Bson::String(my_join.user_id.clone()),
            "chat_id": Bson::String(delete .chat_id        )
        }, doc! { "$set": {
            "content": None::< String >
        } }).ok()?;

        if result.modified_count == 0 {
            return None
        }

        Some(ws::Message::Text(jmsg))
    }

    pub fn join(
        &self         ,
        track_id: &str,
        join    : AJoin
    ) -> Option< (ws::Message, ws::Message) > {
        let history = self.histories.collection(track_id);
        let online  = self.onlines  .entry(track_id.to_string()).or_insert_with(DashMap::new);
        online.insert(join.user_id.clone(), join.name);
        let bmsg  = BMsg::join(BJoin {
            user_id: join.user_id
        });
        let rmsg  = BMsg::join_result(BJoinResult {
            history: history.find(doc! {}).run().ok()?.into_iter().map(Result::ok).collect::< Option< Vec< _ > > >()?,
            online : online.iter().map(|x| x.key().clone()).collect()
        });
        let bjmsg = serde_json::to_string(&rmsg).ok()?;
        let rjmsg = serde_json::to_string(&bmsg).ok()?;
        Some((ws::Message::Text(bjmsg), ws::Message::Text(rjmsg)))
    }

    pub fn leave(
        &self           ,
        my_join : &AJoin,
        track_id: &str
    ) -> Option< ws::Message > {
        let online = self.onlines.entry(track_id.to_string()).or_insert_with(DashMap::new);
        let bmsg   = BMsg::leave(BLeave {
            user_id: my_join.user_id.clone()
        });
        let jmsg   = serde_json::to_string(&bmsg).ok()?;
        online.remove(&my_join.user_id)?;
        Some(ws::Message::Text(jmsg))
    }

    pub fn hot(
        &self,
        n: usize
    ) -> Option< Vec< Hot > > {
        let track_ids = self.histories.list_collection_names().ok()?;
        let nts       = track_ids.iter().filter_map(|track_id| {
            let num_comments = self.histories.collection::< BChat >(&track_id).count_documents().ok()? as usize;
            Some(Hot { num_comments, track_id: track_id.clone() })
        });
        let hots      = nts.collect::< BinaryHeap< _ > >().into_iter().take(n).collect::< Vec< _ > >();
        Some(hots)
    }
}
