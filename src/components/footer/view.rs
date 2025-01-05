use leptos::prelude::*;

use crate::cn;
use crate::icons::{
    github_logo::GitHubLogo, linkedin_logo::LinkedInLogo, rust_logo::RustLogo,
    whatsapp_logo::WhatsAppLogo, x_logo::XLogo,
};

#[component]
pub fn Footer() -> impl IntoView {
    let LINKS = [
        ("Blogs", "/blogs"),
        ("Showcase", "/showcase"),
        ("Join Us 🔥", "/"),
    ];

    view! {
        <footer class="w-full">
            <div
                class="w-full flex bottom-0 z-40 justify-center px-6 py-6 md:py-12 bg-background-main duration-300"
            >
                <div
                    class="w-full max-w-6xl flex flex-col md:flex-row items-center gap-10 md:gap-0 md:justify-between duration-300"
                >
                    <a href="/" class="animate-scale-in">
                        <RustLogo {..} class="text-black size-12" />
                    </a>

                    <ul class="w-full flex md:gap-x-6 justify-between md:justify-normal items-center">
                        {
                            LINKS.into_iter().enumerate()
                                .map(|(index, (text, href))|
                                    view! {
                                        <li
                                            style=format!("animation-delay: {}s", 0.3 + (index as f64 * 0.1))
                                            class="opacity-0 animate-translate-in-from-b"
                                        >
                                            <a class=cn!(#("text-[1.375rem] leading-[2rem] text-grey-50 font-medium hover:underline", (text == "Join Us 🔥", "text-black"))) href=href>
                                                {text}
                                            </a>
                                        </li>
                                    }
                                ).collect_view()
                        }
                    </ul>

                    <div class="flex gap-x-6">
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
            </div>
        </footer>
    }
}
