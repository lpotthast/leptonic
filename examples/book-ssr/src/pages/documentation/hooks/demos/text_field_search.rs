use leptonic::hooks::*;
use leptos::prelude::*;

#[component]
pub fn TextFieldSearchDemo() -> impl IntoView {
    let query = use_text_field_state(UseTextFieldStateInput::default());
    let (submitted, set_submitted) = signal(None::<String>);

    let UseSearchFieldReturn {
        text_field,
        clear_button,
    } = use_search_field(UseSearchFieldInput {
        on_submit: Some(Callback::new(move |value| set_submitted.set(Some(value)))),
        ..UseSearchFieldInput::new(UseTextFieldInput {
            has_label: true,
            placeholder: "Search\u{2026}".into(),
            ..UseTextFieldInput::new(query)
        })
    });
    let clear = use_button(clear_button);
    let (clear_attrs, clear_styles) = clear.props.into_parts();
    let is_empty = move || query.value.with(String::is_empty);

    view! {
        <div class="demo-field">
            <label class="demo-field-label" {..text_field.label_props.into_attrs()}>"Search"</label>
            <div class="demo-inline-controls demo-no-margin">
                <input class="demo-input demo-text-input" {..text_field.input_props.into_attrs()}/>
                <button {..clear_attrs} style=clear_styles class="demo-btn" class:demo-invisible=is_empty>
                    "\u{2715}"
                </button>
            </div>
        </div>

        <p>
            "Submitted: "
            <strong>{move || submitted.get().unwrap_or_else(|| "nothing yet".to_owned())}</strong>
        </p>
        <p class="demo-caption">"Press Enter to submit, Escape to clear."</p>
    }
}
