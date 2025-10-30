use crate::cn;
use crate::illustrations::{
    smoke_layer_1::SmokeLayer1, smoke_layer_2::SmokeLayer2, smoke_layer_3::SmokeLayer3,
};
use leptos::html::Div;
use leptos::prelude::*;
use stylance::*;

use crate::{
    components::rocket::Rocket,
    hooks::use_in_view::{use_in_view, ElementVisibilityData},
};

import_style!(classes, "rocket_lift.module.scss");

#[component]
pub fn Stats() -> impl IntoView {
    let section_ref = NodeRef::<Div>::new();

    let ElementVisibilityData {
        in_view: section_in_view,
    } = use_in_view(section_ref, None);

    view! {
        <div class="w-full max-w-7xl flex h-[700px] items-end justify-center bg-grey-20 rounded-3xl overflow-hidden">
           <div node_ref=section_ref class="w-full h-full relative flex items-center justify-center">

             <div>
                <Rocket
                    class=cn!(#(
                        classes::rocket,
                        (section_in_view(), classes::rocketReveal)
                    ))
                    inner_class=cn!(#(
                        (section_in_view(), classes::rocketInnerReveal)
                    ))
                />
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
