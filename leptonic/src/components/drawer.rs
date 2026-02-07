use leptos::prelude::*;
use leptos_use::{use_interval_fn_with_options, utils::Pausable, UseIntervalFnOptions};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrawerSide {
    #[default]
    Left,
    Right,
}

impl DrawerSide {
    pub const fn to_str(self) -> &'static str {
        match self {
            Self::Left => "left",
            Self::Right => "right",
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
#[allow(clippy::match_same_arms)]
pub fn Drawer(
    side: DrawerSide,
    #[prop(into, optional, default = Signal::from(true))] shown: Signal<bool>,
    children: Children,
) -> impl IntoView {
    let memoized_shown = Memo::new(move |_| shown.get());

    let (anim_state, set_anim_state) = signal(if memoized_shown.get_untracked() {
        DrawerAnimationState::Shown
    } else {
        DrawerAnimationState::Hidden
    });

    let target_state = Signal::derive(move || {
        if memoized_shown.get() {
            DrawerAnimationState::Shown
        } else {
            DrawerAnimationState::Hidden
        }
    });

    let Pausable {
        pause,
        resume,
        is_active: _,
    } = use_interval_fn_with_options(
        move || {
            // Advance towards target state.
            match (anim_state.get_untracked(), target_state.get_untracked()) {
                (DrawerAnimationState::Shown, DrawerAnimationState::Shown) => {}
                (DrawerAnimationState::Shown, DrawerAnimationState::Hidden) => {
                    set_anim_state.set(DrawerAnimationState::Hiding);
                }
                (DrawerAnimationState::Showing, DrawerAnimationState::Shown) => {
                    set_anim_state.set(DrawerAnimationState::Shown);
                }
                (DrawerAnimationState::Showing, DrawerAnimationState::Hidden) => {
                    set_anim_state.set(DrawerAnimationState::Hiding);
                }
                (DrawerAnimationState::Hiding, DrawerAnimationState::Shown) => {
                    set_anim_state.set(DrawerAnimationState::Showing);
                }
                (DrawerAnimationState::Hiding, DrawerAnimationState::Hidden) => {
                    set_anim_state.set(DrawerAnimationState::Hidden);
                }
                (DrawerAnimationState::Hidden, DrawerAnimationState::Shown) => {
                    set_anim_state.set(DrawerAnimationState::Showing);
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

    Effect::new(move |_| {
        let anim_state = anim_state.get();
        let target_state = target_state.get();

        if anim_state == target_state {
            pause();
        } else {
            resume();
        }
    });

    view! {
        <leptonic-drawer
            class:shown=move || anim_state.get() == DrawerAnimationState::Shown
            class:showing=move || anim_state.get() == DrawerAnimationState::Showing
            class:hiding=move || anim_state.get() == DrawerAnimationState::Hiding
            class:hidden=move || anim_state.get() == DrawerAnimationState::Hidden
            data-side=side.to_str()
        >
            {children()}
        </leptonic-drawer>
    }
}
