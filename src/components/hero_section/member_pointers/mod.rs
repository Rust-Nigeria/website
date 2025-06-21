use leptos::prelude::*;
use stylance::*;

use crate::icons::arrow_head::ArrowHead;

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
pub fn MemberPointers(reveal: ReadSignal<bool>) -> impl IntoView {
    view! {
        <div class="hidden lg:flex items-end justify-center overflow-hidden absolute bottom-0 w-full p-6 2xl:p-10">
            <div class="w-full aspect-[1.8/1] max-w-[900px] xl:max-w-[1000px] translate-y-[30%] relative">
                {
                    MEMBERS.into_iter().enumerate()
                        .map(|(index, (name, image_url, theme_color))|
                            view! {
                                <div class=cn!(#(
                                        "flex duration-300 p-2 absolute origin-top w-fit flex-col justify-center items-center gap-y-1.5",
                                        &format!("member-pointer-{}", index + 1),
                                        (reveal(), "reveal p-8")
                                    ))>
                                   <div class="flex rounded-full justify-center absolute h-full aspect-square origin-center arrow-axis">
                                     <ArrowHead {..} class="size-5 arrow-head origin-center -rotate-[45deg]" />
                                   </div>
                                    <div>
                                        <div class=cn!("size-12 relative flex overflow-hidden justify-center rounded-full border-2 border-white", classes::memberAvatarWrapper)>
                                            <img src=image_url alt="image" class="w-full h-full rounded-full border-2 object-cover" />
                                        </div>
                                    </div>
                                    <div style=format!("background-color: {}", theme_color) class="h-8 flex relative text-white items-center w-fit rounded-full px-4">
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
