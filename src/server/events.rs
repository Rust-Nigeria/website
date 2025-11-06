use crate::types::events::CommunityEvent;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use server_fn::codec::JsonEncoding;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventsErrors {
    ServerFnError(ServerFnErrorErr),
    Other(String),
}

impl FromServerFnError for EventsErrors {
    type Encoder = JsonEncoding;

    fn from_server_fn_error(value: ServerFnErrorErr) -> Self {
        EventsErrors::ServerFnError(value)
    }
}

impl From<String> for EventsErrors {
    fn from(value: String) -> Self {
        EventsErrors::Other(value)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CategorisedEvents {
    pub upcoming: Vec<CommunityEvent>,
    pub past: Vec<CommunityEvent>,
}

#[server]
pub async fn get_events(today_str: String) -> Result<CategorisedEvents, EventsErrors> {
    use chrono::DateTime;
    use std::fs;

    let body = fs::read_to_string("data/events.json")
        .map_err(|e| EventsErrors::Other(format!("Failed to read file: {}", e)))?;

    let events: Vec<CommunityEvent> = serde_json::from_str(&body).map_err(|e| {
        return EventsErrors::ServerFnError(ServerFnErrorErr::Response(format!(
            "Invalid JSON format: {}",
            e
        )));
    })?;

    let today = DateTime::parse_from_rfc3339(&today_str).ok().unwrap();
    let mut upcoming = Vec::new();
    let mut past = Vec::new();

    for ev in events {
        let event_date = ev.date;
        if event_date >= today {
            upcoming.push(ev);
        } else {
            past.push(ev);
        }
    }

    // sort upcoming soonest-first, past most-recent-first
    upcoming.sort_by_key(|e| e.date);
    past.sort_by_key(|e| e.date);
    past.reverse();

    Ok(CategorisedEvents { upcoming, past })
}
