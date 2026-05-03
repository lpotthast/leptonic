use leptonic::{ScrollBehavior, components::prelude::*, hooks::*};
use leptos::prelude::*;

#[component]
pub fn AnchorLinkDemo() -> impl IntoView {
    let (disabled, set_disabled) = signal(false);

    let UseAnchorLinkReturn {
        props,
        is_pressed: _,
        ..
    } = use_anchor_link(UseAnchorLinkInput {
        href: Href::from_str(Oco::Borrowed("#my-anchor-element")).expect("valid href"),
        scroll_behavior: Some(ScrollBehavior::Smooth),
        disabled: disabled.into(),
        element_type: LinkElementType::default(),
        description: None,
        on_press: None,
        on_press_start: None,
        on_press_end: None,
    });
    let (link_props, link_styles) = props.into_inner();
    let attrs = link_props.into_attrs();

    view! {
        <a
            {..attrs}
            style=link_styles
            class="leptonic-anchor-link"
            target="_self"
        >
            "#"
        </a>

        <FormControl classes="demo-form-row">
            <Checkbox checked=disabled set_checked=set_disabled />
            <Label>"Disabled"</Label>
        </FormControl>

        <div id="my-anchor-element" style="margin-top: 1em; padding: 1em; border: 1px solid var(--brand-color); border-radius: 4px;">
            "This is the anchor target element."
        </div>
    }
}
