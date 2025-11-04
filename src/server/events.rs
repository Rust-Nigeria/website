use crate::types::events::CommunityEvent;
use chrono::NaiveDate;
use leptos::prelude::*;
use reqwest;
use serde::{Deserialize, Serialize};
use server_fn::codec::JsonEncoding;

const DATA_URL: &str = "https://raw.githubusercontent.com/AkinAguda/test-data/main/events.json";

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
    let response = reqwest::get(DATA_URL).await.map_err(|e| {
        return EventsErrors::ServerFnError(ServerFnErrorErr::Response(format!(
            "Failed to fetch data: {}",
            e
        )));
    })?;

    if !response.status().is_success() {
        return Err(EventsErrors::ServerFnError(ServerFnErrorErr::Response(
            format!("Failed to fetch data: HTTP {}", response.status()),
        )));
    }

    let body = response.text().await.map_err(|e| {
        return EventsErrors::ServerFnError(ServerFnErrorErr::Response(format!(
            "Failed to read response body: {}",
            e
        )));
    })?;

    let events: Vec<CommunityEvent> = serde_json::from_str(&body).map_err(|e| {
        return EventsErrors::ServerFnError(ServerFnErrorErr::Response(format!(
            "Invalid JSON format: {}",
            e
        )));
    })?;

    let today = NaiveDate::parse_from_str(&today_str, "%Y-%m-%d")
        .ok()
        .unwrap();
    let mut upcoming = Vec::new();
    let mut past = Vec::new();

    for ev in events {
        if let Ok(event_date) = NaiveDate::parse_from_str(&ev.date[..10].to_string(), "%Y-%m-%d") {
            if event_date >= today {
                upcoming.push(ev);
            } else {
                past.push(ev);
            }
        } else {
            println!("Failed Parsing: {}", &ev.date);
            // fallback if date parsing fails — consider it past
            past.push(ev);
        }
    }

    // sort upcoming soonest-first, past most-recent-first
    upcoming.sort_by_key(|e| NaiveDate::parse_from_str(&e.date, "%Y-%m-%d").ok());
    past.sort_by_key(|e| NaiveDate::parse_from_str(&e.date, "%Y-%m-%d").ok());
    past.reverse();

    Ok(CategorisedEvents { upcoming, past })
}
