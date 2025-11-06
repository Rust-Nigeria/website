use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Person {
    pub image: String,
    pub name: String,
    pub portfolio: Option<String>,
}
