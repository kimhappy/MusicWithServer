use std::sync::Arc;
use rocket::futures::{ SinkExt, StreamExt };
use tokio::sync::broadcast;
use rocket_ws as ws;
use crate::chat::*;

#[rocket::get("/chat/<track_id>/<user_id>")]
pub fn get_chat(
    ws        : ws::WebSocket,
    track_id  : String       ,
    user_id   : String       ,
    chat_state: &rocket::State< Arc< State > >
) -> ws::Channel< 'static > {
    let chat_state = chat_state.inner().clone();

    ws.channel(move |mut stream| Box::pin(async move {
        let sender = match chat_state.sender(&track_id) {
            Some(sender) => sender,
            None         => {
                eprintln!("Error making sender from {}", user_id);
                return Ok(())
            }
        };

        let mut receiver = sender.subscribe();

        let msg = match chat_state.add_online(&track_id, &user_id) {
            Some(msg) => msg,
            None      => {
                eprintln!("Error adding online from {}", user_id);
                return Ok(())
            }
        };

        if let Err(err) = sender.send(msg) {
            eprintln!("Error broadcasting join message: {:?}", err);
            return Ok(())
        }

        loop { tokio::select! {
            message = stream.next() => {
                let msg = match message {
                    Some(Ok (msg)) => msg,
                    Some(Err(err)) => {
                        eprintln!("Error receiving message from {}: {:?}", user_id, err);
                        break
                    }
                    None => break
                };

                let json_text = match msg {
                    ws::Message::Text(text) => text,
                    _                       => {
                        eprintln!("Received non-text message from {}", user_id);
                        continue
                    }
                };

                let cmsg = match serde_json::from_str(&json_text) {
                    Ok (cmsg) => cmsg,
                    Err(err ) => {
                        eprintln!("Error parsing message from {}: {:?}", user_id, err);
                        break
                    }
                };

                let (is_broadcast, msg) = match match cmsg {
                    client::Msg::Chat   (chat   ) => chat_state.add_chat   (&track_id, &user_id, chat   ).map(|msg| (true , msg)),
                    client::Msg::Delete (delete ) => chat_state.delete_chat(&track_id, &user_id, delete ).map(|msg| (true , msg)),
                    client::Msg::History(history) => chat_state.get_history(&track_id          , history).map(|msg| (false, msg)),
                    client::Msg::Online (online ) => chat_state.get_online (&track_id          , online ).map(|msg| (false, msg))
                } {
                    Some(im) => im,
                    None     => {
                        eprintln!("Error processing message from {}", user_id);
                        break
                    }
                };

                if is_broadcast {
                    if let Err(err) = sender.send(msg) {
                        eprintln!("Error broadcasting message: {:?}", err);
                        break
                    }
                }
                else {
                    if let Err(err) = stream.send(msg).await {
                        eprintln!("Error sending history to {}: {:?}", user_id, err);
                        break
                    }
                }
            }

            result = receiver.recv() => {
                let msg = match result {
                    Ok (msg                                         ) => msg  ,
                    Err(broadcast::error::RecvError::Closed         ) => break,
                    Err(broadcast::error::RecvError::Lagged(skipped)) => {
                        eprintln!("{} skipped {} messages", user_id, skipped);
                        continue
                    }
                };

                if let Err(err) = stream.send(msg).await {
                    eprintln!("Error sending message to {}: {:?}", user_id, err);
                    break
                }
            }
        } }

        let msg = match chat_state.remove_online(&track_id, &user_id) {
            Some(msg) => msg,
            None      => {
                eprintln!("Error removing online from {}", user_id);
                return Ok(())
            }
        };

        if let Err(err) = sender.send(msg) {
            eprintln!("Error broadcasting leave message: {:?}", err);
        }

        Ok(())
    }))
}
