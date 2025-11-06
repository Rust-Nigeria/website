use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use server_fn::codec::JsonEncoding;

use crate::types::projects::CommunityProject;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProjectsErrors {
    ServerFnError(ServerFnErrorErr),
    Other(String),
}

impl FromServerFnError for ProjectsErrors {
    type Encoder = JsonEncoding;

    fn from_server_fn_error(value: ServerFnErrorErr) -> Self {
        ProjectsErrors::ServerFnError(value)
    }
}

impl From<String> for ProjectsErrors {
    fn from(value: String) -> Self {
        ProjectsErrors::Other(value)
    }
}

#[server]
pub async fn get_projects() -> Result<Vec<CommunityProject>, ProjectsErrors> {
    use chrono::DateTime;
    use std::fs;

    let body = fs::read_to_string("data/projects.json")
        .map_err(|e| ProjectsErrors::Other(format!("Failed to read articles: {}", e)))?;

    let mut projects: Vec<CommunityProject> = serde_json::from_str(&body).map_err(|e| {
        return ProjectsErrors::ServerFnError(ServerFnErrorErr::Response(format!(
            "Invalid JSON format: {}",
            e
        )));
    })?;

    Ok(projects)
}
