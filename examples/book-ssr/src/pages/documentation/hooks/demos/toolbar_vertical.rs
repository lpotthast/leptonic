use leptonic::hooks::*;
use leptos::prelude::*;

#[component]
pub fn ToolbarVerticalDemo() -> impl IntoView {
    let vertical_toolbar = use_toolbar(UseToolbarInput {
        aria_label: "Actions".into(),
        orientation: Orientation::Vertical,
        ..UseToolbarInput::default()
    });
    let last_action = RwSignal::new(None::<&'static str>);

    // ArrowUp and ArrowDown move focus between the buttons; Tab leaves the toolbar.
    view! {
        <div class="demo-flex-center-row">
            <div {..vertical_toolbar.props.into_attrs()} class="demo-toolbar demo-toolbar-vertical">
                <ActionButton name="New" last_action=last_action/>
                <ActionButton name="Save" last_action=last_action/>
                <ActionButton name="Export" last_action=last_action/>
            </div>
            <p>"Last action: " <strong>{move || last_action.get().unwrap_or("none")}</strong></p>
        </div>
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
    view! { <button {..attrs} style=styles class="demo-toolbar-button">{name}</button> }
}
