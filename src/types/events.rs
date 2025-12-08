use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::components::cards_list::CardsListItem;
use crate::types::person::Person;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy)]
#[serde(rename_all = "snake_case")]
pub enum EventTags {
    Workshop,
    Meetup,
    Webinar,
    InPerson,
    Hackathon,
}

impl fmt::Display for EventTags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            EventTags::Workshop => "Workshop",
            EventTags::Meetup => "Meetup",
            EventTags::Webinar => "Webinar",
            EventTags::InPerson => "In-Person",
            EventTags::Hackathon => "Hackathon",
        };
        write!(f, "{}", s)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct CommunityEvent {
    pub banner: String,
    pub name: String,
    pub description: String,
    pub speakers: Vec<Person>,
    pub event_link: String,
    pub date: DateTime<Utc>,
    pub tags: Vec<EventTags>,
}

impl CardsListItem for CommunityEvent {
    type Tag = EventTags;

    fn get_key(&self) -> String {
        format!("event-{}-{}", self.name, self.banner)
    }
    fn get_tags(&self) -> &Vec<EventTags> {
        &self.tags
    }
}
