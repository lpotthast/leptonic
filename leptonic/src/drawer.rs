use leptos::*;
use leptos_use::{use_interval_fn_with_options, utils::Pausable, UseIntervalFnOptions};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrawerSide {
    #[default]
    Left,
    Right,
}

impl DrawerSide {
    pub fn to_str(self) -> &'static str {
        match self {
            DrawerSide::Left => "left",
            DrawerSide::Right => "right",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DrawerAnimationState {
    Shown,
    Showing,
    Hiding,
    Hidden,
}

#[component]
pub fn Drawer(
    cx: Scope,
    side: DrawerSide,
    #[prop(into, optional, default = true.into())] shown: MaybeSignal<bool>,
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    children: Children,
) -> impl IntoView {
    let memoized_shown = create_memo(cx, move |_| shown.get());

    let (anim_state, set_anim_state) = create_signal(
        cx,
        match memoized_shown.get_untracked() {
            true => DrawerAnimationState::Shown,
            false => DrawerAnimationState::Hidden,
        },
    );

    let target_state = Signal::derive(cx, move || match memoized_shown.get() {
        true => DrawerAnimationState::Shown,
        false => DrawerAnimationState::Hidden,
    });

    let Pausable {
        pause,
        resume,
        is_active: _,
    } = use_interval_fn_with_options(
        cx,
        move || {
            // Advance towards target state.
            match (anim_state.get_untracked(), target_state.get_untracked()) {
                (DrawerAnimationState::Shown, DrawerAnimationState::Shown) => {}
                (DrawerAnimationState::Shown, DrawerAnimationState::Hidden) => {
                    set_anim_state.set(DrawerAnimationState::Hiding)
                }
                (DrawerAnimationState::Showing, DrawerAnimationState::Shown) => {
                    set_anim_state.set(DrawerAnimationState::Shown)
                }
                (DrawerAnimationState::Showing, DrawerAnimationState::Hidden) => {
                    set_anim_state.set(DrawerAnimationState::Hiding)
                }
                (DrawerAnimationState::Hiding, DrawerAnimationState::Shown) => {
                    set_anim_state.set(DrawerAnimationState::Showing)
                }
                (DrawerAnimationState::Hiding, DrawerAnimationState::Hidden) => {
                    set_anim_state.set(DrawerAnimationState::Hidden)
                }
                (DrawerAnimationState::Hidden, DrawerAnimationState::Shown) => {
                    set_anim_state.set(DrawerAnimationState::Showing)
                }
                (DrawerAnimationState::Hidden, DrawerAnimationState::Hidden) => {}
                _ => tracing::error!("Reached an unexpected branch!"),
            }
        },
        200, // Animation speed
        UseIntervalFnOptions {
            immediate: true,
            immediate_callback: true,
        },
    );
    pause();

    create_effect(cx, move |_| {
        let anim_state = anim_state.get();
        let target_state = target_state.get();

        if anim_state != target_state {
            resume();
        } else {
            pause();
        }
    });

    view! { cx,
        <leptonic-drawer
            id=id
            class=class
            class:shown=move || anim_state.get() == DrawerAnimationState::Shown
            class:showing=move || anim_state.get() == DrawerAnimationState::Showing
            class:hiding=move || anim_state.get() == DrawerAnimationState::Hiding
            class:hidden=move || anim_state.get() == DrawerAnimationState::Hidden
            style=style
            side=side.to_str()
        >
            { children(cx) }
        </leptonic-drawer>
    }
}
