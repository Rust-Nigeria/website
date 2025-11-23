use std::time::Duration;

use leptos::prelude::*;

use crate::types::{contexts::CommunityDialogState, dialog::DialogState};

pub fn use_join_community_dialog() -> (impl Fn(), impl Fn(), ReadSignal<DialogState>) {
    let CommunityDialogState { state } = expect_context::<CommunityDialogState>();

    let open = move || {
        state.set(DialogState::Opening);
        set_timeout(
            move || state.set(DialogState::Opened),
            Duration::from_millis(350),
        );
    };

    let close = move || {
        state.set(DialogState::Closing);
        set_timeout(
            move || state.set(DialogState::Closed),
            Duration::from_millis(350),
        );
    };

    (open, close, state.read_only())
}
