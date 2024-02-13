use leptos::*;
use leptos_router::{State, ToHref, A};

use crate::{
    hooks::{
        button::{use_button, UseButtonProps},
        focus::{use_focus, UseFocusOptions},
        press::{use_press, PressEvent, UsePressInput},
    },
    prelude::Consumer,
    utils::aria::{AriaExpanded, AriaHasPopup},
    OptMaybeSignal,
};

#[component]
pub fn Button(
    #[prop(into)] on_press: Consumer<PressEvent>,
    #[prop(into, optional)] disabled: OptMaybeSignal<bool>,
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    #[prop(into, optional)] aria_haspopup: OptMaybeSignal<AriaHasPopup>,
    #[prop(into, optional)] aria_expanded: OptMaybeSignal<AriaExpanded>,
    /// Arbitrary additional attributes.
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let el: NodeRef<html::Button> = create_node_ref();

    let btn = use_button(UseButtonProps {
        node_ref: el,
        disabled: disabled.or(false),
        aria_haspopup: aria_haspopup.or_default(),
        aria_expanded: aria_expanded.or_default(),
    });

    let focus = use_focus(UseFocusOptions {
        disabled: disabled.or(false),
        on_focus: None,
        on_blur: None,
        on_focus_change: None,
    });

    let press = use_press(UsePressInput {
        on_press: Callback::new(move |e| {
            if !disabled.get_untracked() {
                //e.stop_propagation();
                on_press.consume(e);
            }
        }),
        on_press_up: None,
        on_press_start: None,
        on_press_end: None,
    });

    view! {
        <button
            {..btn.props}
            {..press.props.attrs}
            {..attributes}
            node_ref=el
            id=id
            class=class
            class:leptonic-btn=true
            style=style
            on:keydown=press.props.on_key_down
            on:click=press.props.on_click
            on:touchstart=press.props.on_touch_start
            on:touchmove=press.props.on_touch_move
            on:touchend=press.props.on_touch_end
            on:focus=focus.on_focus
            on:blur=focus.on_blur
        >
            { children() }
        </button>
    }
}

#[component]
pub fn ButtonWrapper(children: Children) -> impl IntoView {
    view! {
        <leptonic-btn-wrapper>
            { children() }
        </leptonic-btn-wrapper>
    }
}
