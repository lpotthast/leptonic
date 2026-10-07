use leptonic::hooks::*;
use leptos::prelude::*;

#[component]
pub fn ToolbarVerticalDemo() -> impl IntoView {
    let toolbar = use_toolbar(UseToolbarInput {
        aria_label: "File".into(),
        orientation: Orientation::Vertical.into(),
        ..UseToolbarInput::default()
    });
    let last_action = RwSignal::new(None::<&'static str>);

    // ArrowUp and ArrowDown move focus between the buttons; Tab leaves the toolbar.
    view! {
        <div {..toolbar.props.into_attrs()} class="demo-toolbar demo-toolbar-vertical">
            <ActionButton name="New" last_action/>
            <ActionButton name="Save" last_action/>
            <ActionButton name="Export" last_action/>
        </div>
        <p class="demo-status">
            {move || match last_action.get() {
                Some(action) => format!("Last action: {action}."),
                None => "No action yet.".to_owned(),
            }}
        </p>
    }
}

/// A toolbar button recording its name as the last action when pressed.
#[component]
fn ActionButton(name: &'static str, last_action: RwSignal<Option<&'static str>>) -> impl IntoView {
    let button = use_button(UseButtonInput {
        on_press: Some(Callback::new(move |_| last_action.set(Some(name)))),
        ..UseButtonInput::default()
    });
    let (attrs, styles) = button.props.into_parts();
    view! {
        <button
            {..attrs}
            style=styles
            class="demo-toolbar-button"
            data-hovered=button.is_hovered
            data-focus-visible=button.is_focus_visible
        >
            {name}
        </button>
    }
}
