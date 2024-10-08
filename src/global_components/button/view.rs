use leptos::*;
use tailwind_fuse::*;

use crate::icons::right_arrow::RightArrow;

pub enum ButtonUsecase {
    Button { on_click: Box<dyn Fn()> },
    Link { href: String },
}

// Variant for size
#[derive(TwVariant)]
#[tw(class = "rounded-full")]
pub enum ButtonSizeVariants {
    #[tw(default, class = "py-4 px-8")]
    Md,
    #[tw(class = "py-6 px-10")]
    Lg,
}

// Variant for color
#[derive(TwVariant)]
pub enum ButtonColorVariants {
    #[tw(default, class = "bg-grey-100 text-grey-10")]
    White,
    #[tw(class = "bg-grey-10 text-grey-100")]
    Black,
    #[tw(class = "bg-grey-30 text-grey-100")]
    Grey,
}

#[derive(TwClass)]
#[tw(class = "inline-flex")]
struct ButtonVariants {
    size: ButtonSizeVariants,
    color: ButtonColorVariants,
}

#[component]
pub fn Button(
    usecase: ButtonUsecase,
    children: Children,
    #[prop(default = ButtonSizeVariants::Md)] size: ButtonSizeVariants,
    #[prop(default = ButtonColorVariants::White)] color: ButtonColorVariants,
) -> impl IntoView {
    let class = ButtonVariants { size, color }.to_class();

    let icon = RightArrow();

    match usecase {
        ButtonUsecase::Button { on_click } => view! {
          <button class=class>{children()}{icon}</button>
        }
        .into_view(),

        ButtonUsecase::Link { href } => view! {
          <a class=class href=href>{children()}{icon}</a>
        }
        .into_view(),
    }
}
