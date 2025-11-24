use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::components::cards_list::CardsListItem;
use crate::types::person::Person;
use std::fmt;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum ArticleTags {
    Technical,
    DeveloperStory,
}

impl fmt::Display for ArticleTags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ArticleTags::Technical => "Technical",
            ArticleTags::DeveloperStory => "Developer Story",
        };
        write!(f, "{}", s)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct CommunityArticle {
    pub banner: String,
    pub name: String,
    pub description: String,
    pub authors: Vec<Person>,
    pub article_link: String,
    pub date: DateTime<Utc>,
    pub tags: Vec<ArticleTags>,
}

impl CardsListItem for CommunityArticle {
    type Tag = ArticleTags;

    fn get_key(&self) -> String {
        format!("event-{}-{}", self.name, self.banner)
    }
    fn get_tags(&self) -> &Vec<ArticleTags> {
        &self.tags
    }
}
