use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::components::cards_list::CardsListItem;
use crate::types::person::Person;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct CommunityArticle {
    pub banner: String,
    pub name: String,
    pub description: String,
    pub authors: Vec<Person>,
    pub article_link: String,
    pub date: DateTime<Utc>,
}

impl CardsListItem for CommunityArticle {
    fn get_key(&self) -> String {
        format!("event-{}-{}", self.name, self.banner)
    }
}
