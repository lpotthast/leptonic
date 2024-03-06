use leptos::*;
use leptos_use::use_document;
use web_sys::ScrollIntoViewOptions;

use crate::hooks::prelude::{use_press, UsePressInput, UsePressReturn};



#[component]
pub fn AnchorLink(
    /// The anchor link. For example: "#my-anchor".
    #[prop(into)]
    href: Oco<'static, str>,

    #[prop(into, optional)]
    scroll_behavior: Option<web_sys::ScrollBehavior>,

    /// Description of this anchor for accessibility.
    /// If text is provided in children, this could be omitted.
    /// If no children are provided, this component renders a single `#`,
    /// which should be described using this field.
    #[prop(into, optional)]
    description: Option<Oco<'static, str>>,

    /// If no children are provided, this component renders a single `#` character.
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    let href_clone = href.clone();

    // We make links "use_press", so that optional PressResponder's higher up the component tree can react on link interactions
    // and so that a custom `on_press` handler can immediately work with the underlying link element.
    let UsePressReturn {
        is_pressed: _,
        props,
    } = use_press(UsePressInput {
        // Links cannot be disabled (for now).
        disabled: false.into(),
        on_press: Callback::new(move |_| {
            if let Some(document) = use_document().as_ref() {
                let elId = href_clone.replace('#', "");
                if let Some(el) = document.get_element_by_id(elId.as_str()) {
                    el.scroll_into_view_with_scroll_into_view_options(
                        ScrollIntoViewOptions::new().behavior(scroll_behavior.unwrap_or(web_sys::ScrollBehavior::Smooth)),
                    );
                } else {
                    tracing::warn!("AnchorLink could not find anchor (element) with id '{elId}'.")
                }
            }
        }),
        on_press_up: None,
        on_press_start: None,
        on_press_end: None,
    });

    view! {
        <leptonic-anchor-link
            {..props.attrs}
            on:keydown=props.on_key_down
            on:click=props.on_click
            on:pointerdown=props.on_pointer_down
            role="link"
            href=href
            aria-label=description
        >
            { match children {
                Some(children) => children().into_view(),
                None => "#".into_view(),
            } }
        </leptonic-anchor-link>
    }
}
