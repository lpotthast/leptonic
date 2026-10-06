use leptonic::{hooks::*, utils::classes::Classes};
use leptos::prelude::*;

#[component]
pub fn ToolbarHorizontalDemo() -> impl IntoView {
    let toolbar = use_toolbar(UseToolbarInput {
        aria_label: "Text Formatting".into(),
        ..UseToolbarInput::default()
    });

    let bold = use_toggle_state(UseToggleStateInput::default());
    let italic = use_toggle_state(UseToggleStateInput::default());
    let underline = use_toggle_state(UseToggleStateInput::default());

    let preview_classes = Classes::from("demo-toolbar-preview")
        .add_reactive("bold", move || bold.is_selected.get())
        .add_reactive("italic", move || italic.is_selected.get())
        .add_reactive("underline", move || underline.is_selected.get());

    // The arrow keys move focus between the buttons; Tab leaves the toolbar.
    view! {
        <div {..toolbar.props.into_attrs()} class="demo-toolbar">
            <FormatButton state=bold format="bold" label="B" name="Bold"/>
            <FormatButton state=italic format="italic" label="I" name="Italic"/>
            <FormatButton state=underline format="underline" label="U" name="Underline"/>
        </div>

        <p>"Preview: " <span class=preview_classes>"Sample formatted text"</span></p>
    }
}

/// Pressed buttons are styled through `aria-pressed`. `name` is the accessible name of the one-letter button.
#[component]
fn FormatButton(
    state: ToggleState,
    format: &'static str,
    label: &'static str,
    name: &'static str,
) -> impl IntoView {
    let button = use_button(use_toggle_button(UseToggleButtonInput {
        button: UseButtonInput {
            aria_label: name.into(),
            ..UseButtonInput::default()
        },
        ..UseToggleButtonInput::new(state)
    }));
    let (attrs, styles) = button.props.into_parts();
    view! {
        <button {..attrs} style=styles class=Classes::from("demo-toolbar-button").add(format)>
            {label}
        </button>
    }
}
