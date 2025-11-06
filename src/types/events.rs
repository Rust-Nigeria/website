use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::components::cards_list::CardsListItem;
use crate::types::person::Person;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct CommunityEvent {
    pub banner: String,
    pub name: String,
    pub description: String,
    pub speakers: Vec<Person>,
    pub event_link: String,
    pub date: DateTime<Utc>,
}

impl CardsListItem for CommunityEvent {
    fn get_key(&self) -> String {
        format!("event-{}-{}", self.name, self.banner)
    }
}
