use leptonic::hooks::{
    IntoAttrs, UseNumberFieldInput, UseNumberFieldStateInput, use_button, use_number_field,
    use_number_field_state,
};
use leptos::prelude::*;

#[component]
pub fn PageHookNumberField() -> impl IntoView {
    let state = use_number_field_state(UseNumberFieldStateInput {
        default_value: Some(1.0_f64),
        min_value: Signal::stored(Some(0.0)),
        max_value: Signal::stored(Some(5.0)),
        ..UseNumberFieldStateInput::default()
    });
    let field = use_number_field(UseNumberFieldInput {
        has_label: true,
        ..UseNumberFieldInput::new(state)
    });
    let (dec_attrs, dec_styles) = use_button(field.decrement_button).props.into_parts();
    let (inc_attrs, inc_styles) = use_button(field.increment_button).props.into_parts();

    let submits = RwSignal::new(0u32);

    view! {
        <div id="test-page-hook-number-field">
            <h1>"use_number_field"</h1>
            <button id="test-nf-before">"Before"</button>
            <form on:submit=move |e| {
                e.prevent_default();
                submits.update(|s| *s += 1);
            }>
            <div {..field.group_props.into_attrs()}>
                <label {..field.label_props.into_attrs()}>"Quantity"</label>
                <button {..dec_attrs} style=dec_styles>"-"</button>
                <input {..field.input_props.into_attrs()} data-testid="input" />
                <button {..inc_attrs} style=inc_styles>"+"</button>
            </div>
            </form>
            <button id="test-nf-after">"After"</button>
            <div>"Submits: " <span id="test-nf-submits">{submits}</span></div>
            <div>
                "Value: "
                <span id="test-nf-value">
                    {move || state.number_value.get().map_or("empty".to_owned(), |v| v.to_string())}
                </span>
            </div>
        </div>
    }
}
