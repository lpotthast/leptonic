use std::{collections::HashMap, sync::Arc};

use leptonic::{
    components::button::{Button, ButtonVariant},
    hooks::{
        ButtonType, FormValidationContext, IntoAttrs, UseFieldInput, UseFieldReturn,
        UseFormResetInput, UseFormValidationInput, UseFormValidationStateInput, ValidationBehavior,
        use_field, use_form_reset, use_form_validation, use_form_validation_state,
    },
    utils::CapturedElement,
};
use leptos::{ev::SubmitEvent, prelude::*};

#[component]
pub fn FormCouponDemo() -> impl IntoView {
    let code = RwSignal::new(String::new());
    // Errors from the server, by field `name`. Provided before the field's hooks run, which read it.
    let server_errors = RwSignal::new(HashMap::<String, Vec<String>>::new());
    provide_context(FormValidationContext {
        errors: server_errors.into(),
    });
    let (applied, set_applied) = signal(None::<String>);

    view! {
        // A plain `<form>`: the browser's constraint validation blocks the submission while the
        // field is invalid.
        <form
            class="demo-form"
            on:submit=move |e: SubmitEvent| {
                e.prevent_default();
                set_applied.set(Some(code.get_untracked()));
            }
            on:reset=move |_| {
                server_errors.set(HashMap::new());
                set_applied.set(None);
            }
        >
            <CouponField code/>
            <div class="demo-flex-center-row">
                <Button button_type=ButtonType::Submit>"Apply"</Button>
                <Button button_type=ButtonType::Reset variant=ButtonVariant::Outlined>"Reset"</Button>
                <Button
                    variant=ButtonVariant::Flat
                    on_press=move |_| server_errors.set(HashMap::from([(
                        "coupon".to_owned(),
                        vec!["This code has expired.".to_owned()],
                    )]))
                >
                    "Simulate server error"
                </Button>
            </div>
        </form>

        <p class="demo-status">
            {move || applied.get().map_or_else(|| "No coupon applied.".to_owned(), |code| format!("Applied coupon: {code}"))}
        </p>
    }
}

/// A coupon code field built from the form hooks and `use_field`.
#[component]
fn CouponField(code: RwSignal<String>) -> impl IntoView {
    let element = CapturedElement::new();

    let validation = use_form_validation_state(UseFormValidationStateInput {
        builtin_validation: Signal::default(),
        is_invalid: false.into(),
        value: code.into(),
        // An empty field is left to the native `required` constraint.
        validate: Some(Arc::new(|code: &String| {
            if code.is_empty()
                || (code.len() == 8
                    && code
                        .chars()
                        .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit()))
            {
                Ok(())
            } else {
                Err(vec![
                    "A coupon code has eight capital letters or digits.".to_owned(),
                ])
            }
        })),
        validation_behavior: ValidationBehavior::Native,
        // Matches the server errors for this field.
        name: Some("coupon".to_owned()),
    });
    use_form_validation(UseFormValidationInput {
        element,
        state: validation,
        validation_behavior: ValidationBehavior::Native,
        focus: None,
    });
    use_form_reset(UseFormResetInput {
        element,
        initial_value: code.get_untracked(),
        on_reset: Callback::new(move |initial| code.set(initial)),
    });

    let UseFieldReturn {
        label_props,
        field_props,
        description_props,
        error_message_props,
        ..
    } = use_field(UseFieldInput {
        has_label: true.into(),
        ..UseFieldInput::default()
    });

    view! {
        <div class="demo-field">
            <label class="demo-field-label" {..label_props.into_attrs()}>"Coupon code"</label>
            <input
                type="text"
                name="coupon"
                required
                class="demo-input demo-text-input"
                prop:value=code
                on:input=move |e| code.set(event_target_value(&e))
                aria-invalid=move || validation.is_invalid.get().then_some("true")
                {..element.attr()}
                {..field_props.into_attrs()}
            />
            <p class="demo-field-description" {..description_props.into_attrs()}>"For example SPRING26."</p>
            <Show when=move || validation.is_invalid.get()>
                <p class="demo-field-error" {..error_message_props.clone().into_attrs()}>
                    {move || validation.validation_errors.get().join(" ")}
                </p>
            </Show>
        </div>
    }
}
