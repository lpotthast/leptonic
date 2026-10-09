use leptonic::{
    IntoAttrs,
    hooks::{
        button::{UseButtonInput, UseToggleButtonInput, use_button, use_toggle_button},
        form::{ToggleState, UseToggleStateInput, use_toggle_state},
        toolbar::{UseToolbarInput, use_toolbar},
    },
};
use leptos::prelude::*;

/// The formats the toolbar toggles.
#[derive(Debug, Clone, Copy)]
enum Format {
    Bold,
    Italic,
    Underline,
}

#[component]
pub fn ToolbarHorizontalDemo() -> impl IntoView {
    let toolbar = use_toolbar(UseToolbarInput {
        aria_label: "Text formatting".into(),
        ..UseToolbarInput::default()
    });

    let bold = use_toggle_state(UseToggleStateInput::default());
    let italic = use_toggle_state(UseToggleStateInput::default());
    let underline = use_toggle_state(UseToggleStateInput::default());

    // The arrow keys move focus between the buttons; Tab leaves the toolbar.
    view! {
        <div {..toolbar.props.into_attrs()} class="demo-toolbar">
            <FormatButton state=bold format=Format::Bold/>
            <FormatButton state=italic format=Format::Italic/>
            <FormatButton state=underline format=Format::Underline/>
        </div>

        <p
            class="demo-toolbar-preview"
            data-bold=bold.is_selected
            data-italic=italic.is_selected
            data-underline=underline.is_selected
        >
            "The quick brown fox jumps over the lazy dog."
        </p>
    }
}

/// A toggle button showing its format on a letter. Its accessible name is the format's name.
#[component]
fn FormatButton(state: ToggleState, format: Format) -> impl IntoView {
    let (name, letter) = match format {
        Format::Bold => ("Bold", view! { <b>"B"</b> }.into_any()),
        Format::Italic => ("Italic", view! { <i>"I"</i> }.into_any()),
        Format::Underline => ("Underline", view! { <u>"U"</u> }.into_any()),
    };
    let button = use_button(use_toggle_button(UseToggleButtonInput {
        button: UseButtonInput {
            aria_label: name.into(),
            ..UseButtonInput::default()
        },
        state,
    }));
    let (attrs, styles) = button.props.into_parts();
    // Pressed buttons are styled through `aria-pressed`, keyboard focus through the hook's `is_focus_visible`.
    view! {
        <button
            {..attrs}
            style=styles
            class="demo-toolbar-button"
            data-hovered=button.is_hovered
            data-focus-visible=button.is_focus_visible
        >
            {letter}
        </button>
    }
}
