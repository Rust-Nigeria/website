use crate::illustrations::{
    smoke_layer_1::SmokeLayer1, smoke_layer_2::SmokeLayer2, smoke_layer_3::SmokeLayer3,
};
use leptos::html::Div;
use leptos::prelude::*;
use stylance::*;
use tailwind_fuse::tw_join;

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

    let rocket_class = Memo::new(move |_| {
        let class_name = if section_in_view() {
            classes::rocketReveal
        } else {
            ""
        };

        tw_join!(class_name, classes::rocket)
    });

    let rocket_inner_class = Memo::new(move |_| {
        let class_name = if section_in_view() {
            classes::rocketInnerReveal
        } else {
            ""
        };

        tw_join!(class_name)
    });

    view! {
        <div class="w-full max-w-7xl flex h-[700px] items-end justify-center bg-grey-20 rounded-3xl overflow-hidden">
           <div node_ref=section_ref class="w-full h-full relative flex items-center justify-center">

             <div>
                <Rocket
                    class=rocket_class.into()
                    inner_class=rocket_inner_class.into()
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
