use leptonic::{components::prelude::Checkbox, hooks::*};
use leptos::prelude::*;

#[component]
pub fn SearchFieldDemo() -> impl IntoView {
    let query = use_text_field_state(UseTextFieldStateInput::default());
    let (submitted, set_submitted) = signal(None::<String>);
    let disabled = RwSignal::new(false);

    let UseSearchFieldReturn {
        text_field,
        clear_button,
    } = use_search_field(UseSearchFieldInput {
        on_submit: Some(Callback::new(move |value| set_submitted.set(Some(value)))),
        ..UseSearchFieldInput::new(UseTextFieldInput {
            has_label: true.into(),
            is_disabled: disabled.into(),
            placeholder: "Search\u{2026}".into(),
            ..UseTextFieldInput::new(query)
        })
    });
    // The clear button's configuration goes to `use_button`. It is labelled "Clear search".
    let (clear_attrs, clear_styles) = use_button(clear_button).props.into_parts();
    let is_empty = move || query.value.with(String::is_empty);

    view! {
        <div class="demo-field">
            <label class="demo-field-label" {..text_field.label_props.into_attrs()}>"Search"</label>
            <div class="demo-input-row">
                <input class="demo-input demo-text-input" {..text_field.input_props.into_attrs()}/>
                // Nothing to clear while the field is empty.
                <button {..clear_attrs} style=clear_styles class="demo-btn" hidden=is_empty>
                    <span aria-hidden="true">"\u{2715}"</span>
                </button>
            </div>
        </div>

        <p class="demo-status">
            {move || match submitted.get() {
                Some(query) => format!("Submitted: \u{201c}{query}\u{201d}"),
                None => "Nothing submitted yet.".to_owned(),
            }}
        </p>

        <div class="demo-controls">
            <Checkbox is_selected=disabled set_selected=disabled>"Disabled"</Checkbox>
        </div>
    }
}
