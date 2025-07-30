pub mod card_tops;

use leptos::prelude::*;

use card_tops::rocket_lift::RocketLift;

// w-[383px]

#[component]
pub fn Cards() -> impl IntoView {
    view! {
        <div class="w-full max-w-7xl flex gap-2 mt-8">
            <div class="w-[1000px] aspect-[383/208]">
                <RocketLift />
            </div>
        </div>
    }
}
