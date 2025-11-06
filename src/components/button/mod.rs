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
    #[tw(default, class = "bg-grey-100 hover:bg-grey-80 text-grey-10")]
    White,
    #[tw(class = "bg-grey-10 text-grey-100 hover:bg-neutral-800")]
    Black,
    #[tw(class = "bg-grey-30 text-grey-100 hover:bg-grey-50")]
    Grey,
    #[tw(class = "bg-transparent text-grey-10 hover:opacity-60")]
    Transparent,
}

// Variant for size
#[derive(TwVariant)]
#[tw(class = "rounded-full [&_>_.btn-inner]:duration-300 [&_>_.btn-icon]:duration-300")]
pub enum ButtonSizeVariants {
    #[tw(class = "group/size-sm text-base [&_>_.btn-icon]:size-5")]
    Thin,
    #[tw(
        default,
        class = "group/size-md py-4 px-6 xl:px-8 [&_>_.btn-icon]:size-6"
    )]
    Md,
    #[tw(class = "group/size-lg py-6 px-10 text-lg 2xl:text-2xl [&_>_.btn-icon]:size-8")]
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
            ButtonIconTypes::RightArrow => Some(
                view! { <RightArrow {..} class="btn-icon group-hover/size-sm:translate-x-12 group-hover/size-md:translate-x-16 group-hover/size-lg:translate-x-20" /> },
            ),
        },
    };

    let additional_class = if icon_el.is_some() {
        "group/with-icon"
    } else {
        ""
    };

    match use_as {
        ButtonUsecase::Button { on_click } => Either::Left(view! {
          <button class=tw_merge!(additional_class, class) on:click=on_click><span class="btn-inner group-hover/with-icon:translate-x-4">{children()}</span>{icon_el}</button>
        }),
        ButtonUsecase::Link { href } => Either::Right(view! {
          <a class=tw_merge!(additional_class, class) href=href><span class="btn-inner group-hover/with-icon:translate-x-4">{children()}</span>{icon_el}</a>
        }),
    }
}
