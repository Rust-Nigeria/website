mod button_backdrop;
mod shaders;

use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

use leptos::{either::Either, ev::MouseEvent, html, prelude::*};
use tailwind_fuse::*;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::js_sys::Date;

use crate::{
    components::button::button_backdrop::{
        ButtonBackdropBuilder, ButtonBackdropCfg, ButtonBackdropInstance,
    },
    hooks::use_in_view::{use_in_view, ElementVisibilityData, InViewOptions},
    icons::right_arrow::RightArrow,
    utils::render_loop::RenderLoop,
};
use simple_bezier_easing::bezier;

#[derive(Clone)]
pub enum ButtonUsecase {
    Button { on_click: Callback<MouseEvent> },
    Link { href: String },
}

#[derive(Copy, Clone)]
pub enum ButtonIconTypes {
    RightArrow,
}

pub struct StateRgbs {
    pub base: [f32; 4],
    pub hover: [f32; 4],
}

// Variant for color
#[derive(TwVariant)]
pub enum ButtonColorVariants {
    #[tw(default, class = "bg-grey-100 hover:bg-grey-80 text-grey-10")]
    White,
    #[tw(class = "bg-grey-10 text-grey-90 hover:bg-neutral-800")]
    Black,
    #[tw(class = "bg-grey-30 text-grey-100 hover:bg-grey-50")]
    Grey,
    #[tw(class = "bg-transparent text-grey-10 hover:opacity-60")]
    Transparent,
}

impl ButtonColorVariants {
    pub fn get_state_rgbs(&self) -> StateRgbs {
        match self {
            ButtonColorVariants::Black => StateRgbs {
                base: [10.0, 10.0, 10.0, 1.0],
                hover: [38.0, 38.0, 38.0, 1.0],
            },
            ButtonColorVariants::Grey => StateRgbs {
                base: [79.0, 79.0, 79.0, 1.0], // TODO - Not the same as bg-grey-30. Fix
                hover: [99.0, 99.0, 99.0, 1.0],
            },
            ButtonColorVariants::White => StateRgbs {
                base: [244.0, 244.0, 244.0, 1.0],
                hover: [227.0, 227.0, 227.0, 1.0],
            },
            ButtonColorVariants::Transparent => StateRgbs {
                base: [0.0, 0.0, 0.0, 0.0],
                hover: [0.0, 0.0, 0.0, 0.0],
            },
        }
    }
}

// Variant for size
#[derive(TwVariant)]
#[tw(class = "rounded-full [&_>_.btn-inner]:duration-500 [&_>_.btn-icon]:duration-500")]
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
#[tw(
    class = "inline-flex relative gap-x-1 cursor-pointer items-center font-medium overflow-visible duration-500"
)]
struct ButtonVariants {
    size: ButtonSizeVariants,
    color: ButtonColorVariants,
}

#[derive(Clone, Copy)]
enum AnimationDirection {
    Forwards,
    Backwards,
}

const TOTAL_ANIMATION_DURATION_MS: f64 = 500.0;

type RenderLoopPtr = Rc<RefCell<RenderLoop>>;

/// Public handles to a running loop. These live *beside* the RenderLoop, not inside it, so
/// calling one only borrows the loop's RefCell fresh — nothing else is holding it at the time.
/// (Putting these on RenderLoop itself deadlocks: reaching the field needs a borrow, and the
/// handle then wants borrow_mut on the same cell.)
struct AnimationControls {
    wake: Box<dyn Fn()>,
    sleep: Box<dyn Fn()>,
    // Tears the loop down when the component drops this (via the StoredValue). Replaces
    // on_cleanup, whose Send + Sync bound an Rc/RefCell can't satisfy.
    _handle: LoopHandle,
}

/// Owns the loop and cancels + frees it on drop.
struct LoopHandle(RenderLoopPtr);

impl Drop for LoopHandle {
    fn drop(&mut self) {
        let mut render_loop = self.0.borrow_mut();
        render_loop.cancel();
        // Drop the Closure so its captured Rc<RefCell<RenderLoop>> clone is released — otherwise
        // the RenderLoop <-> Closure cycle keeps the loop alive forever.
        render_loop.closure = None;
    }
}

// Animaiton loop should pass in a value to render funciton that ping pongs between 0 and 1 depending on the direction
// of the animation. Frowards goes from 0 -> 1 and backwards goes from 1 -> 0. The idea is that its basically just playing a
// predefined animation but giving us a normalised time value between 0 and 1
fn create_render_loop(
    backdrop: ButtonBackdropInstance,
    direction: Rc<Cell<AnimationDirection>>,
) -> AnimationControls {
    let render_loop: RenderLoopPtr = Rc::new(RefCell::new(RenderLoop::default()));
    let timing = bezier(0.42, 0.0, 0.58, 1.0).unwrap();
    let time: Rc<Cell<f64>> = Rc::new(Cell::new(Date::now()));
    let progression: Rc<Cell<f64>> = Rc::new(Cell::new(0.0));
    let running: Rc<Cell<bool>> = Rc::new(Cell::new(false));

    let render_fn: Rc<dyn Fn()> = {
        let (progression, time, direction) = (progression.clone(), time.clone(), direction.clone());
        Rc::new(move || {
            let now = Date::now();
            let dt = now - time.get();
            let multiplier = match direction.get() {
                AnimationDirection::Backwards => -1.0,
                AnimationDirection::Forwards => 1.0,
            };

            let time_progression =
                (progression.get() + (dt * multiplier)).clamp(0.0, TOTAL_ANIMATION_DURATION_MS);
            progression.set(time_progression);

            let normalised_time_progression = time_progression / TOTAL_ANIMATION_DURATION_MS;
            let eased_progression = timing(normalised_time_progression as f32).unwrap();

            time.set(now);

            backdrop.render(eased_progression);
        })
    };

    let closure: Closure<dyn Fn(f64)> = {
        let window = web_sys::window().unwrap();
        let (render_loop, render_fn) = (render_loop.clone(), render_fn.clone());

        Closure::wrap(Box::new(move |_| {
            render_fn();

            let mut render_loop = render_loop.borrow_mut();
            render_loop.animation_id = render_loop.closure.as_ref().map(|closure| {
                window
                    .request_animation_frame(closure.as_ref().unchecked_ref())
                    .expect("cannot set animation frame")
            })
        }))
    };

    render_loop.borrow_mut().closure = Some(closure);

    let wake: Box<dyn Fn()> = {
        let window = web_sys::window().unwrap();
        let (render_loop, progression, time, running) = (
            render_loop.clone(),
            progression.clone(),
            time.clone(),
            running.clone(),
        );

        Box::new(move || {
            // Guard first: a redundant wake must not reset progression mid-animation or start a
            // parallel rAF chain (which would run the animation at double speed).
            if running.replace(true) {
                return;
            }

            // Fresh start from 0; reset the clock so the first frame doesn't see a stale dt.
            progression.set(0.0);
            time.set(Date::now());

            let mut render_loop_borrow = render_loop.borrow_mut();
            if let Some(closure) = &render_loop_borrow.closure {
                render_loop_borrow.animation_id = Some(
                    window
                        .request_animation_frame(closure.as_ref().unchecked_ref())
                        .expect("cannot set animation frame"),
                );
            }
        })
    };

    let sleep: Box<dyn Fn()> = {
        let (render_loop, progression, time, direction, running, render_fn) = (
            render_loop.clone(),
            progression.clone(),
            time.clone(),
            direction.clone(),
            running.clone(),
            render_fn.clone(),
        );

        Box::new(move || {
            running.set(false);
            progression.set(0.0);
            time.set(Date::now());
            direction.set(AnimationDirection::Backwards);

            // Paint the resting frame once so the canvas isn't left mid-animation / blank.
            render_fn();

            render_loop.borrow().cancel();
        })
    };

    // No on_cleanup: teardown rides on LoopHandle's Drop, which runs when the StoredValue holding
    // these controls is disposed on unmount (or replaced if this effect re-runs).
    AnimationControls {
        wake,
        sleep,
        _handle: LoopHandle(render_loop),
    }
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
    let canvas_ref: NodeRef<html::Canvas> = NodeRef::new();
    let button_ref: NodeRef<html::Button> = NodeRef::new();
    let link_ref: NodeRef<html::A> = NodeRef::new();

    // StoredValue is Copy, so the effects below capture it directly — no Rc<RefCell<Option<..>>>
    // to clone around. new_local because the boxed closures aren't Send/Sync.
    let controls = StoredValue::new_local(None::<AnimationControls>);

    let class = ButtonVariants { size, color }.with_class(class);

    let icon_el = match icon {
        None => None,
        Some(icon_type) => match icon_type {
            ButtonIconTypes::RightArrow => Some(
                view! { <RightArrow {..} class="btn-icon relative group-hover/size-sm:translate-x-14 group-hover/size-md:translate-x-16 xl:group-hover/size-md:translate-x-[70px] group-hover/size-lg:translate-x-24" /> },
            ),
        },
    };

    let additional_class = if icon_el.is_some() {
        "group/with-icon"
    } else {
        ""
    };

    let maybe_backdrop_builder = LocalResource::new(ButtonBackdropBuilder::load);

    let (extension_dimension, set_extension_dimension) = signal::<Option<i32>>(None);

    let (is_hovering, set_is_hovering) = signal(false);

    let animation_direction = Rc::new(Cell::new(AnimationDirection::Backwards));
    let anim_dir_clone = animation_direction.clone();

    let use_as_clone = use_as.clone();

    let ElementVisibilityData {
        in_view: canvas_in_view,
    } = use_in_view(
        canvas_ref,
        Some(InViewOptions {
            trigger_once: Some(false),
            ..Default::default()
        }),
    );

    // TODO - This should respond to resizing. We might also benefit from making the backdrop
    // struct also get it's extension from here

    Effect::new(move || {
        if icon.is_none()
            || matches!(size, ButtonSizeVariants::Thin)
            || matches!(color, ButtonColorVariants::Transparent)
        {
            return;
        }

        match &use_as_clone {
            ButtonUsecase::Button { .. } => {
                if let Some(el) = button_ref.get() {
                    set_extension_dimension(Some(el.client_height()));
                }
            }
            ButtonUsecase::Link { .. } => {
                if let Some(el) = link_ref.get() {
                    set_extension_dimension(Some(el.client_height()));
                }
            }
        }
    });

    // Build the loop once canvas + backdrop builder are ready, apply the current visibility state
    // immediately (untracked, so this effect stays keyed to canvas/builder only), then store it.
    // The wake guard makes the initial wake safe even if the visibility effect also fires true.
    Effect::new(move || {
        if let Some(canvas) = canvas_ref.get() {
            if let Some(backdrop_builder) = maybe_backdrop_builder.get() {
                let backdrop =
                    backdrop_builder.create_backdrop(canvas, ButtonBackdropCfg { color });
                let ctrls = create_render_loop(backdrop, animation_direction.clone());

                if canvas_in_view.get_untracked() {
                    (ctrls.wake)();
                } else {
                    (ctrls.sleep)();
                }

                controls.set_value(Some(ctrls));
            }
        }
    });

    Effect::new(move || {
        if is_hovering.get() {
            anim_dir_clone.set(AnimationDirection::Forwards);
        } else {
            anim_dir_clone.set(AnimationDirection::Backwards);
        }
    });

    // Drive wake/sleep off visibility. Read the signal FIRST and unconditionally, or the effect
    // registers no dependency on its early-exit run and never fires again.
    Effect::new(move || {
        let in_view = canvas_in_view.get();
        controls.with_value(|c| {
            if let Some(c) = c {
                if in_view {
                    (c.wake)();
                } else {
                    (c.sleep)();
                }
            }
        });
    });

    match use_as {
        ButtonUsecase::Button { on_click } => Either::Left(view! {
          <button
            node_ref=button_ref
            on:mouseenter=move |_| set_is_hovering(true)
            on:mouseleave=move |_| set_is_hovering(false)
            class=tw_merge!(additional_class, class)
            on:click=move |e| on_click.run(e)
          >

            {move || extension_dimension().map(move |ext| {
                view! {
                    <canvas
                        style=move || format!("width: calc(100% + {}px)", ext)
                        node_ref=canvas_ref class="absolute pointer-events-none hidden sm:block top-0 h-full left-0"
                    >
                    </canvas>
                }
            })}

            <span class="btn-inner relative group-hover/with-icon:translate-x-4">
                {children()}
            </span>

            {icon_el}
          </button>
        }),
        ButtonUsecase::Link { href } => Either::Right(view! {
            <a
                node_ref=link_ref
                on:mouseenter=move |_| set_is_hovering(true)
                on:mouseleave=move |_| set_is_hovering(false)
                class=tw_merge!(additional_class, class)
                href=href
            >

            {move || extension_dimension().map(move |ext| {
                view! {
                    <canvas
                        style=move || format!("width: calc(100% + {}px)", ext)
                        node_ref=canvas_ref class="absolute pointer-events-none hidden sm:block top-0 h-full left-0"
                    >
                    </canvas>
                }
            })}


                <span class="btn-inner relative group-hover/with-icon:translate-x-4">
                    {children()}
                </span>

                {icon_el}
            </a>
        }),
    }
}
