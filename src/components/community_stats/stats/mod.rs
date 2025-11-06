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
        <div class="w-full max-w-7xl flex h-[500px] sm:h-[700px] items-end justify-center bg-grey-20 rounded-3xl overflow-hidden">
           <div node_ref=section_ref class="w-full h-full relative flex items-center justify-center">
            <div class="absolute text-white left-0 top-0 w-full h-full pl-6 pt-10 sm:pl-20 sm:pt-20">
                <div class="text-left w-fit">
                    <p class=cn!(#(
                        "huge-label opacity-0 translate-y-8 duration-500",
                         (section_in_view(), "opacity-100 translate-y-0")
                    ))>
                        Over 700
                    </p>
                    <p class=cn!(#(
                        "header-2 mt-3 opacity-0 translate-y-8 duration-500 delay-100",
                         (section_in_view(), "opacity-100 translate-y-0")
                    ))>
                        Community Members and growing!
                    </p>
                </div>
             </div>

             <div class=classes::rocketContainer>
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

             <div class=cn!(#(
                "absolute h-full w-full pointer-events-none bottom-0 overflow-hidden", classes::smoke,
                (section_in_view(), classes::animateSmoke)
             ))>
                <SmokeLayer1 {..} class="absolute w-full bottom-0" />
                <SmokeLayer2 {..} class="absolute w-full bottom-0" />
                <SmokeLayer3 {..} class="absolute w-full bottom-0" />
             </div>
           </div>
        </div>
    }
}
