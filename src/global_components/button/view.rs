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
    #[tw(class = "bg-grey-10 text-grey-100")]
    Black,
    #[tw(class = "bg-grey-30 text-grey-100")]
    Grey,
}

// Variant for size
#[derive(TwVariant)]
#[tw(class = "rounded-full")]
pub enum ButtonSizeVariants {
    #[tw(default, class = "py-4 px-6 xl:px-8")]
    Md,
    #[tw(class = "py-6 px-10 text-lg")]
    Lg,
}

#[derive(TwVariant)]
#[tw(class = "rounded-full")]
pub enum IconSizeVariants {
    #[tw(default, class = "w-5")]
    Md,
    #[tw(class = "w-6")]
    Lg,
}

trait IntoIconSize {
    fn into_icon_size(&self) -> IconSizeVariants;
}

impl IntoIconSize for ButtonSizeVariants {
    fn into_icon_size(&self) -> IconSizeVariants {
        match self {
            ButtonSizeVariants::Lg => IconSizeVariants::Lg,
            ButtonSizeVariants::Md => IconSizeVariants::Md,
        }
    }
}

#[derive(TwClass)]
#[tw(class = "inline-flex gap-x-1 font-medium")]
struct ButtonVariants {
    size: ButtonSizeVariants,
    color: ButtonColorVariants,
}

#[derive(TwClass)]
#[tw(class = "ml-1")]
struct IconVariants {
    size: IconSizeVariants,
}

#[component]
pub fn Button(
    use_as: ButtonUsecase,
    children: Children,
    #[prop(default = ButtonSizeVariants::Md)] size: ButtonSizeVariants,
    #[prop(default = ButtonColorVariants::White)] color: ButtonColorVariants,
    #[prop(optional)] icon: Option<ButtonIconTypes>,
) -> impl IntoView {
    let class = ButtonVariants { size, color }.to_class();
    let icon_class = IconVariants {
        size: size.into_icon_size(),
    }
    .to_class();

    let icon_el = match icon {
        None => None,
        Some(icon_type) => match icon_type {
            ButtonIconTypes::RightArrow => Some(view! { <RightArrow {..} class=icon_class /> }),
        },
    };

    match use_as {
        ButtonUsecase::Button { on_click } => Either::Left(view! {
          <button class=class on:click=on_click>{children()}{icon_el}</button>
        }),
        ButtonUsecase::Link { href } => Either::Right(view! {
          <a class=class href=href>{children()}{icon_el}</a>
        }),
    }
}
