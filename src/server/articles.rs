use crate::types::{actions::ActionErrors, articles::CommunityArticle};
use leptos::prelude::*;

#[server]
pub async fn get_articles() -> Result<Vec<CommunityArticle>, ActionErrors> {
    use crate::types::actions::ActionServerErr;
    use std::fs;

    let body = fs::read_to_string("data/articles.json").map_err(|e| ActionErrors {
        client_err: "An Error Occured when getting articles".to_string(),
        server_err: ActionServerErr::Other(format!("Failed to read articles: {}", e)),
    })?;

    let mut articles: Vec<CommunityArticle> =
        serde_json::from_str(&body).map_err(|e| ActionErrors {
            client_err: "An Error Occured when parsing articles".to_string(),
            server_err: ActionServerErr::Other(format!("Invalid JSON format: {}", e)),
        })?;

    articles.sort_by_key(|e| e.date);

    articles.reverse();

    Ok(articles)
}
