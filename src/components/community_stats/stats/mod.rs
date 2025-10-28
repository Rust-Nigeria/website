use crate::illustrations::{
    smoke_layer_1::SmokeLayer1, smoke_layer_2::SmokeLayer2, smoke_layer_3::SmokeLayer3,
};
use leptos::prelude::*;
use stylance::*;

use crate::components::rocket::Rocket;

import_style!(classes, "rocket_lift.module.scss");

#[component]
pub fn Stats() -> impl IntoView {
    view! {
        <div class="w-full max-w-7xl flex h-[700px] items-end justify-center bg-grey-20 rounded-3xl overflow-hidden">
           <div class="w-full h-full relative flex items-center justify-center">
             <div>
                <Rocket class=classes::rocket />
             </div>
             <div class="absolute w-full bottom-0 left-0">
                <SmokeLayer1 {..} class="absolute w-full bottom-0 left-0" />
                <SmokeLayer2 {..} class="absolute w-full bottom-0 left-0" />
                <SmokeLayer3 {..} class="absolute w-full bottom-0 left-0" />
             </div>
           </div>
        </div>
    }
}
