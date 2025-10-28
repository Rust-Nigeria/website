use leptos::prelude::*;
use stylance::*;

use crate::components::rocket::Rocket;

import_style!(classes, "rocket_lift.module.scss");

#[component]
pub fn RocketLift() -> impl IntoView {
    view! {
        <div class="w-full h-full flex items-end justify-center bg-blue-200">
            <Rocket class=classes::rocket />
        </div>
    }
}
