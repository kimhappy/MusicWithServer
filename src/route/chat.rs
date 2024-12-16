use std::sync::Arc;
use rocket::futures::{ SinkExt, StreamExt };
use tokio::sync::broadcast;
use rocket_ws as ws;
use crate::{ chat::*, state::State };

#[rocket::get("/chat/<track_id>")]
pub fn get_chat(
    ws          : ws::WebSocket,
    track_id    : String       ,
    server_state: &rocket::State< Arc< State > >
) -> ws::Channel< 'static > {
    let server_state = server_state.inner().clone();

    ws.channel(move |mut stream| Box::pin(async move {
        let     sender   = server_state.chat.sender(&track_id).unwrap();
        let mut receiver = sender.subscribe();
        let mut my_join  = None::< AJoin >;

        loop { tokio::select! {
            message = stream.next() => {
                let msg = match message {
                    Some(Ok (msg)) => msg,
                    Some(Err(err)) => {
                        eprintln!("Error receiving message from: {:?}", err);
                        break
                    }
                    None => break
                };

                let json_text = match msg {
                    ws::Message::Text(text) => text,
                    _                       => {
                        eprintln!("Received non-text message from");
                        continue
                    }
                };

                let cmsg = match serde_json::from_str(&json_text) {
                    Ok (cmsg) => cmsg,
                    Err(err ) => {
                        eprintln!("Error parsing message: {:?}", err);
                        eprintln!("{:?}", json_text);
                        break
                    }
                };

                let (to_broad, to_client) = match match cmsg {
                    AMsg::join  (join  )                            => {
                        my_join = Some(join.clone());
                        server_state.chat.join       (    &track_id, join  ).map(|(b, r)| (b, Some(r)))
                    },
                    AMsg::chat  (chat  ) if let Some(mj) = &my_join => {
                        server_state.chat.add_chat   (mj, &track_id, chat  ).map(| b    | (b, None   ))
                    },
                    AMsg::delete(delete) if let Some(mj) = &my_join => {
                        server_state.chat.delete_chat(mj, &track_id, delete).map(| b    | (b, None   ))
                    },
                    _ => {
                        eprintln!("Not joined yet");
                        break
                    }
                } {
                    Some(msgs) => msgs,
                    None         => {
                        eprintln!("Error processing message");
                        break
                    }
                };

                if let Err(err) = sender.send(to_broad) {
                    eprintln!("Error broadcasting message: {:?}", err);
                    break
                }

                if let Some(msg) = to_client &&
                   let Err (err) = stream.send(msg).await {
                    eprintln!("Error sending history & online to: {:?}", err);
                    break
                }
            }

            result = receiver.recv() => {
                let msg = match result {
                    Ok (msg                                         ) => msg  ,
                    Err(broadcast::error::RecvError::Closed         ) => break,
                    Err(broadcast::error::RecvError::Lagged(skipped)) => {
                        eprintln!("Skipped {} messages", skipped);
                        continue
                    }
                };

                if let Err(err) = stream.send(msg).await {
                    eprintln!("Error sending message: {:?}", err);
                    break
                }
            }
        } }

        if let Some(my_join) = my_join {
            let msg = match server_state.chat.leave(&my_join, &track_id) {
                Some(msg) => msg,
                None      => {
                    eprintln!("Error removing online");
                    return Ok(())
                }
            };

            if let Err(err) = sender.send(msg) {
                eprintln!("Error broadcasting leave message: {:?}", err);
            }
        }

        Ok(())
    }))
}
