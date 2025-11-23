use leptos::prelude::*;

use crate::types::{contexts::CommunityDialogState, dialog::DialogState};

#[component]
pub fn AppProviders(children: Children) -> impl IntoView {
    let community_dialog_state = RwSignal::new(DialogState::Closed);

    provide_context(CommunityDialogState {
        state: community_dialog_state,
    });

    view! {
      {children()}
    }
}
