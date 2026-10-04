use leptonic::hooks::{ButtonElementType, ButtonType, UseButtonInput, UseButtonReturn, use_button};
use leptos::prelude::*;

#[component]
pub fn PageHookButton() -> impl IntoView {
    let presses = RwSignal::new(0u32);
    let submits = RwSignal::new(0u32);
    let disabled = RwSignal::new(false);
    let count = move |_| presses.update(|p| *p += 1);

    let (native_attrs, native_styles) = use_button(UseButtonInput {
        id: Some("test-btn-native".into()),
        on_press: Some(Callback::new(count)),
        disabled: disabled.into(),
        ..Default::default()
    })
    .props
    .into_parts();

    let (div_attrs, div_styles) = use_button(UseButtonInput {
        id: Some("test-btn-div".into()),
        element_type: ButtonElementType::Other,
        on_press: Some(Callback::new(count)),
        disabled: disabled.into(),
        ..Default::default()
    })
    .props
    .into_parts();

    let (anchor_attrs, anchor_styles) = use_button(UseButtonInput {
        id: Some("test-btn-anchor".into()),
        element_type: ButtonElementType::Anchor,
        href: Some(Signal::stored("#anchor-target".to_owned())),
        disabled: disabled.into(),
        ..Default::default()
    })
    .props
    .into_parts();

    let (excluded_attrs, excluded_styles) = use_button(UseButtonInput {
        id: Some("test-btn-excluded".into()),
        exclude_from_tab_order: Signal::stored(true),
        ..Default::default()
    })
    .props
    .into_parts();

    let (plain_in_form_attrs, plain_in_form_styles) = use_button(UseButtonInput {
        id: Some("test-btn-in-form".into()),
        on_press: Some(Callback::new(count)),
        ..Default::default()
    })
    .props
    .into_parts();

    let (submit_attrs, submit_styles) = use_button(UseButtonInput {
        id: Some("test-btn-submit".into()),
        button_type: ButtonType::Submit,
        ..Default::default()
    })
    .props
    .into_parts();

    let UseButtonReturn {
        props: hover_props,
        is_hovered,
        is_focus_visible,
        ..
    } = use_button(UseButtonInput {
        id: Some("test-btn-state".into()),
        ..Default::default()
    });
    let (hover_attrs, hover_styles) = hover_props.into_parts();

    view! {
        <div id="test-page-hook-button">
            <h1>"use_button"</h1>
            <button id="test-btn-before">"Before"</button>
            <button {..native_attrs} style=native_styles>"Native"</button>
            <div {..div_attrs} style=div_styles>"Div"</div>
            <a {..anchor_attrs} style=anchor_styles>"Anchor"</a>
            <button {..excluded_attrs} style=excluded_styles>"Excluded"</button>
            <button id="test-btn-after">"After"</button>
            <button id="test-btn-toggle-disabled" on:click=move |_| disabled.update(|d| *d = !*d)>
                "Toggle disabled"
            </button>
            <div>"Presses: " <span id="test-btn-presses">{presses}</span></div>

            <form on:submit=move |e| {
                e.prevent_default();
                submits.update(|s| *s += 1);
            }>
                <button {..plain_in_form_attrs} style=plain_in_form_styles>"Plain"</button>
                <button {..submit_attrs} style=submit_styles>"Submit"</button>
            </form>
            <div>"Submits: " <span id="test-btn-submits">{submits}</span></div>

            <button {..hover_attrs} style=hover_styles>"State"</button>
            <div>"Hovered: " <span id="test-btn-is-hovered">{move || is_hovered.get().to_string()}</span></div>
            <div>
                "Focus visible: "
                <span id="test-btn-is-focus-visible">{move || is_focus_visible.get().to_string()}</span>
            </div>
        </div>
    }
}
