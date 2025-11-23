use leptos::prelude::RwSignal;

use crate::types::dialog::DialogState;

#[derive(Clone, Debug)]
pub struct CommunityDialogState {
    pub state: RwSignal<DialogState>,
}
