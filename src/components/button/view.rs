use leptos::{either::Either, ev::MouseEvent, prelude::*};
use tailwind_fuse::*;

use crate::icons::right_arrow::RightArrow;

pub enum ButtonUsecase {
    Button {
        on_click: Box<dyn FnMut(MouseEvent)>,
    },
    Link {
        href: String,
    },
}

pub enum ButtonIconTypes {
    RightArrow,
}

// Variant for color
#[derive(TwVariant)]
pub enum ButtonColorVariants {
    #[tw(default, class = "bg-grey-100 text-grey-10")]
    White,
    #[tw(class = "bg-grey-10 text-grey-100 hover:bg-neutral-800")]
    Black,
    #[tw(class = "bg-grey-30 text-grey-100")]
    Grey,
}

// Variant for size
#[derive(TwVariant)]
#[tw(class = "rounded-full group [&_>_.btn-inner]:duration-300 [&_>_.btn-icon]:duration-300")]
pub enum ButtonSizeVariants {
    #[tw(
        default,
        class = "py-4 px-6 xl:px-8 [&_>_.btn-icon]:size-6 [&_>_.btn-inner]:hover:translate-x-4 [&_>_.btn-icon]:hover:translate-x-16"
    )]
    Md,
    #[tw(class = "py-6 px-10 text-lg [&_>_.btn-icon]:size-8")]
    Lg,
}

#[derive(TwClass)]
#[tw(class = "inline-flex gap-x-1 items-center font-medium overflow-hidden duration-300")]
struct ButtonVariants {
    size: ButtonSizeVariants,
    color: ButtonColorVariants,
}

#[component]
pub fn Button(
    use_as: ButtonUsecase,
    children: Children,
    #[prop(default = ButtonSizeVariants::Md)] size: ButtonSizeVariants,
    #[prop(default = ButtonColorVariants::White)] color: ButtonColorVariants,
    #[prop(default = "")] class: &'static str,
    #[prop(optional)] icon: Option<ButtonIconTypes>,
) -> impl IntoView {
    let class = ButtonVariants { size, color }.with_class(class);

    let icon_el = match icon {
        None => None,
        Some(icon_type) => match icon_type {
            ButtonIconTypes::RightArrow => Some(view! { <RightArrow {..} class="btn-icon" /> }),
        },
    };

    match use_as {
        ButtonUsecase::Button { on_click } => Either::Left(view! {
          <button class=class on:click=on_click><span class="btn-inner">{children()}</span>{icon_el}</button>
        }),
        ButtonUsecase::Link { href } => Either::Right(view! {
          <a class=class href=href><span class="btn-inner">{children()}</span>{icon_el}</a>
        }),
    }
}
