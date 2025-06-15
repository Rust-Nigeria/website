use leptos::prelude::*;
use stylance::*;

use crate::cn;

import_style!(classes, "member_pointers.module.scss");

const MEMBERS: [(&str, &str, &str); 9] = [
    ("Chinedu", "/assets/images/members/chinedu.png", "#17A300"),
    ("Chinedu", "/assets/images/members/chinedu.png", "#A24CE6"),
    ("Chinedu", "/assets/images/members/chinedu.png", "#FF7233"),
    ("Chinedu", "/assets/images/members/chinedu.png", "#699688"),
    ("Chinedu", "/assets/images/members/chinedu.png", "#333"),
    ("Chinedu", "/assets/images/members/chinedu.png", "#FF6701"),
    ("Chinedu", "/assets/images/members/chinedu.png", "#FF394E"),
    ("Chinedu", "/assets/images/members/chinedu.png", "#009480"),
    ("Chinedu", "/assets/images/members/chinedu.png", "#27A1FF"),
];

#[component]
pub fn MemberPointers() -> impl IntoView {
    view! {
        <div class=cn!(#("h-full flex items-end top-0 w-full p-6 2xl:p-10 absolute bg-red-300 max-w-[1200px]", classes::leg))>
            <div class="h-[80%] w-full bg-green-300">
                {
                    MEMBERS.into_iter().enumerate()
                        .map(|(index, (name, image_url, theme_color))|
                            view! {
                                <div class="flex w-fit flex-col items-center gap-y-1.5">
                                    <div>
                                        <div class="size-12 flex overflow-hidden justify-center rounded-full border-2">
                                            <img src=image_url alt="image" class="w-full h-full object-cover" />
                                        </div>
                                    </div>
                                    <div style=format!("background-color: {}", theme_color) class="h-8 flex text-white items-center w-fit rounded-full px-4">
                                        {name}
                                    </div>
                                </div>
                            }
                        ).collect_view()
                }
            </div>
        </div>
    }
}
