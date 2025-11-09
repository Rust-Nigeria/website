use crate::types::{actions::ActionErrors, projects::CommunityProject};
use leptos::prelude::*;

#[server]
pub async fn get_projects() -> Result<Vec<CommunityProject>, ActionErrors> {
    use crate::types::actions::ActionServerErr;
    use chrono::DateTime;
    use std::fs;

    let body = fs::read_to_string("data/projects.json").map_err(|e| ActionErrors {
        client_err: "An Error Occured when getting projects".to_string(),
        server_err: ActionServerErr::Other(format!("Failed to read projects: {}", e)),
    })?;

    let mut projects: Vec<CommunityProject> =
        serde_json::from_str(&body).map_err(|e| ActionErrors {
            client_err: "An Error Occured when parsing projects".to_string(),
            server_err: ActionServerErr::Other(format!("Invalid JSON format: {}", e)),
        })?;

    Ok(projects)
}
