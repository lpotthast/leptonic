use leptos::*;
use leptos_use::{use_interval_fn_with_options, utils::Pausable, UseIntervalFnOptions};

use crate::{hooks::*, Transparent};

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
    #[prop(into, optional, default = true.into())] shown: MaybeSignal<bool>,
    #[prop(into, optional)] disable_interact_outside_tracking_when: MaybeSignal<bool>,
    #[prop(into, optional)] on_interact_outside: Option<Callback<InteractOutsideEvent>>,
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    children: Children,
) -> impl IntoView {
    let memoized_shown = create_memo(move |_| shown.get());

    let (anim_state, set_anim_state) = create_signal(match memoized_shown.get_untracked() {
        true => DrawerAnimationState::Shown,
        false => DrawerAnimationState::Hidden,
    });

    let target_state = Signal::derive(move || match memoized_shown.get() {
        true => DrawerAnimationState::Shown,
        false => DrawerAnimationState::Hidden,
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

    create_effect(move |_| {
        let anim_state = anim_state.get();
        let target_state = target_state.get();

        if anim_state == target_state {
            pause();
        } else {
            resume();
        }
    });

    let el: NodeRef<leptos::html::Custom> = create_node_ref();
    let interact_outside = use_interact_outside(
        el,
        UseInteractOutsideInput::builder()
            .disabled(disable_interact_outside_tracking_when)
            .on_interact_outside(on_interact_outside.unwrap_or_else(|| Callback::new(|_e| {})))
            .build(),
    );

    view! {
        <Transparent>
            <leptonic-drawer
                {..interact_outside.props.attrs}
                {..interact_outside.props.handlers}
                ref=el
                id=id
                class=class
                class:shown=move || anim_state.get() == DrawerAnimationState::Shown
                class:showing=move || anim_state.get() == DrawerAnimationState::Showing
                class:hiding=move || anim_state.get() == DrawerAnimationState::Hiding
                class:hidden=move || anim_state.get() == DrawerAnimationState::Hidden
                style=style
                data-side=side.to_str()
            >
                { children() }
            </leptonic-drawer>
        </Transparent>
    }
}
