use leptos::html::Div;
use leptos::prelude::*;

use crate::cn;
use crate::hooks::use_in_view::{use_in_view, ElementVisibilityData};
use crate::icons::{
    github_logo::GitHubLogo, linkedin_logo::LinkedInLogo, rust_nigeria_logo::RustNigeriaLogo,
    whatsapp_logo::WhatsAppLogo, x_logo::XLogo,
};

const LINKS: [(&str, &str); 3] = [
    ("Blogs", "/blogs"),
    ("Showcase", "/showcase"),
    ("Join Us 🔥", "/"),
];

#[component]
pub fn Footer() -> impl IntoView {
    let footer_el = NodeRef::<Div>::new();

    let ElementVisibilityData { in_view } = use_in_view(footer_el, None);

    view! {
        <footer
            class="w-full flex justify-center px-6 py-6 md:py-12 bg-background-main duration-300"
        >
            <div
                node_ref={footer_el}
                class="w-full max-w-6xl flex flex-col md:flex-row items-center gap-10 md:gap-0 md:justify-between duration-300"
            >
                <a href="/" class="animate-scale-in">
                    <RustNigeriaLogo usecase_id="footer" {..} class="text-black size-12" />
                </a>

                <ul class="flex flex-wrap md:gap-x-6 justify-between md:justify-normal items-center w-full md:w-auto">
                    {
                        LINKS.into_iter().enumerate()
                            .map(|(index, (text, href))|
                                view! {
                                    <li
                                        style=format!("animation-delay: {}s", 0.3 + (index as f64 * 0.1))
                                        class=cn!(#("opacity-0", (in_view(), "animate-translate-in-from-b")))
                                    >
                                        <a class="text-xl text-grey-50 font-medium hover:underline last:text-black" href=href>
                                            {text}
                                        </a>
                                    </li>
                                }
                            ).collect_view()
                    }
                </ul>

                <div class="flex gap-x-10 md:gap-x-6">
                    <a href="https://github.com" class="animate-scale-in" target="_blank" rel="noopener noreferrer">
                        <GitHubLogo {..} class="text-grey-10 size-6"/>
                    </a>
                    <a href="https://whatsapp.com" class="animate-scale-in" target="_blank" rel="noopener noreferrer">
                        <WhatsAppLogo {..} class="text-grey-10 size-6"/>
                    </a>
                    <a href="https://x.com" class="animate-scale-in" target="_blank" rel="noopener noreferrer">
                        <XLogo {..} class="text-grey-10 size-6"/>
                    </a>
                    <a href="https://linkedin.com" class="animate-scale-in" target="_blank" rel="noopener noreferrer">
                        <LinkedInLogo {..} class="text-grey-10 size-6"/>
                    </a>
                </div>
            </div>
        </footer>
    }
}
