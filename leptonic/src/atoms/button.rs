use leptos::*;

use crate::{
    hooks::{
        button::{use_button, UseButtonInput},
        focus::UseFocusInput,
        prelude::UseButtonReturn,
        press::{PressEvent, UsePressInput},
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
    #[prop(attrs)]
    attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let el: NodeRef<html::Button> = create_node_ref();

    let UseButtonReturn { props } = use_button(UseButtonInput {
        node_ref: el,
        disabled: disabled.or(false),
        aria_haspopup: aria_haspopup.or_default(),
        aria_expanded: aria_expanded.or_default(),

        use_focus_input: UseFocusInput {
            disabled: disabled.or(false),
            on_focus: None,
            on_blur: None,
            on_focus_change: None,
        },

        use_press_input: UsePressInput {
            disabled: disabled.or(false),
            on_press: Callback::new(move |e| {
                on_press.consume(e);
            }),
            on_press_up: None,
            on_press_start: None,
            on_press_end: None,
        },
    });

    view! {
        <button
            {..props.attrs}
            {..attributes}
            node_ref=el
            id=id
            class=class
            class:leptonic-btn=true
            style=style
            on:keydown=props.on_key_down
            on:click=props.on_click
            on:pointerdown=props.on_pointer_down
            on:focus=props.on_focus
            on:blur=props.on_blur
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
