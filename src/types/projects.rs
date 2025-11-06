use serde::{Deserialize, Serialize};

use crate::components::cards_list::CardsListItem;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct CommunityProject {
    pub repo_url: String,
    pub banner: String,
}

impl CardsListItem for CommunityProject {
    fn get_key(&self) -> String {
        format!("project-{}", self.repo_url)
    }
}
