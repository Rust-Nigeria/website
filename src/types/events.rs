use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Speaker {
    pub image: String,
    pub name: String,
    pub github: Option<String>,
    pub linkedin: Option<String>,
    pub x: Option<String>,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct CommunityEvent {
    pub banner: String,
    pub name: String,
    pub description: String,
    pub speakers: Vec<Speaker>,
    pub event_link: String,
    pub date: String,
}
