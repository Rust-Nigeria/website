use serde::{Deserialize, Serialize};
use std::fmt;

use crate::components::cards_list::CardsListItem;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy)]
#[serde(rename_all = "snake_case")]
pub enum ProjectTags {
    Wasm,
    Ai,
    Blockchain,
    Leptos,
    Crate,
}

impl fmt::Display for ProjectTags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ProjectTags::Wasm => "WASM",
            ProjectTags::Ai => "AI",
            ProjectTags::Blockchain => "Blockchain",
            ProjectTags::Leptos => "Leptos",
            ProjectTags::Crate => "Crate",
        };
        write!(f, "{}", s)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct CommunityProject {
    pub repo_url: String,
    pub banner: String,
    pub tags: Vec<ProjectTags>,
}

impl CardsListItem for CommunityProject {
    type Tag = ProjectTags;

    fn get_key(&self) -> String {
        format!("project-{}", self.repo_url)
    }
    fn get_tags(&self) -> &Vec<ProjectTags> {
        &self.tags
    }
}
