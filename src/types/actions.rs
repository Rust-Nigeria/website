use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use server_fn::codec::JsonEncoding;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionServerErr {
    Other(String),
    ServerFnError(ServerFnErrorErr),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionErrors {
    pub server_err: ActionServerErr,
    pub client_err: String,
}

impl FromServerFnError for ActionErrors {
    type Encoder = JsonEncoding;

    fn from_server_fn_error(value: ServerFnErrorErr) -> Self {
        ActionErrors {
            server_err: ActionServerErr::ServerFnError(value),
            client_err: "Oops! An Error Occured".to_string(),
        }
    }
}

impl From<String> for ActionErrors {
    fn from(value: String) -> Self {
        ActionErrors {
            server_err: ActionServerErr::Other(value.clone()),
            client_err: value,
        }
    }
}
