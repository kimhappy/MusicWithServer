use dashmap::DashMap;
use chrono::prelude::*;

struct Session {
    refresh_token: String,
    expires      : chrono::DateTime< chrono::Utc >
}

pub struct State {
    sessions: DashMap< String, Session >
}

impl State {
    pub fn new() -> Self {
        Self {
            sessions: DashMap::new()
        }
    }

    pub fn make_session(
        &self,
        refresh_token: &str
    ) -> String {
        let session_id = uuid::Uuid::new_v4().to_string();
        let session    = Session {
            refresh_token: refresh_token.to_string(),
            expires      : Utc::now() + chrono::Duration::hours(24)
        };
        self.sessions.insert(session_id.clone(), session);
        session_id
    }

    pub fn verify_session(
        &self,
        session_id: &str
    ) -> bool {
        self.sessions.get(session_id).map(|session| session.expires > Utc::now()).unwrap_or(false)
    }

    pub fn delete_session(
        &self,
        session_id: &str
    ) -> bool {
        self.sessions.remove(session_id).is_some()
    }

    pub fn cleanup_sessions(&self) {
        self.sessions.retain(|_, session| session.expires > Utc::now());
    }

    pub fn get_refresh_token(
        &self,
        session_id: &str
    ) -> Option< String > {
        self.sessions.get(session_id).map(|session| session.refresh_token.clone())
    }

    pub fn update_refresh_token(
        &self,
        session_id   : &str,
        refresh_token: &str
    ) -> bool {
        self.sessions.get_mut(session_id).map(|mut session| {
            session.refresh_token = refresh_token.to_string();
            true
        }).unwrap_or(false)
    }

    pub fn extend_session(
        &self,
        session_id: &str
    ) -> bool {
        self.sessions.get_mut(session_id).map(|mut session| {
            session.expires = Utc::now() + chrono::Duration::hours(24);
            true
        }).unwrap_or(false)
    }
}
