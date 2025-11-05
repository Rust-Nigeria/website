use crate::types::articles::CommunityArticle;
use leptos::prelude::*;
use reqwest;
use serde::{Deserialize, Serialize};
use server_fn::codec::JsonEncoding;

const DATA_URL: &str = "https://raw.githubusercontent.com/AkinAguda/test-data/main/events.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArticlesErrors {
    ServerFnError(ServerFnErrorErr),
    Other(String),
}

impl FromServerFnError for ArticlesErrors {
    type Encoder = JsonEncoding;

    fn from_server_fn_error(value: ServerFnErrorErr) -> Self {
        ArticlesErrors::ServerFnError(value)
    }
}

impl From<String> for ArticlesErrors {
    fn from(value: String) -> Self {
        ArticlesErrors::Other(value)
    }
}

#[server]
pub async fn get_articles() -> Result<Vec<CommunityArticle>, ArticlesErrors> {
    let response = reqwest::get(DATA_URL).await.map_err(|e| {
        return ArticlesErrors::ServerFnError(ServerFnErrorErr::Response(format!(
            "Failed to fetch data: {}",
            e
        )));
    })?;

    if !response.status().is_success() {
        return Err(ArticlesErrors::ServerFnError(ServerFnErrorErr::Response(
            format!("Failed to fetch data: HTTP {}", response.status()),
        )));
    }

    let body = response.text().await.map_err(|e| {
        return ArticlesErrors::ServerFnError(ServerFnErrorErr::Response(format!(
            "Failed to read response body: {}",
            e
        )));
    })?;

    let mut articles: Vec<CommunityArticle> = serde_json::from_str(&body).map_err(|e| {
        return ArticlesErrors::ServerFnError(ServerFnErrorErr::Response(format!(
            "Invalid JSON format: {}",
            e
        )));
    })?;

    articles.sort_by_key(|e| e.date);

    articles.reverse();

    Ok(articles)
}
