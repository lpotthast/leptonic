use leptonic::hooks::*;
use leptos::prelude::*;

/// Whether `email` looks like an address: a name, an `@` and a domain with a dot.
fn is_email(email: &str) -> bool {
    email
        .split_once('@')
        .is_some_and(|(name, domain)| !name.is_empty() && domain.contains('.'))
}

#[component]
pub fn FieldEmailDemo() -> impl IntoView {
    let email = RwSignal::new("ferris@".to_owned());
    let is_invalid =
        Signal::derive(move || email.with(|email| !email.is_empty() && !is_email(email)));

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
    // The ids the input is described by, shown below the demo.
    let described_by = field_props.aria_describedby;

    view! {
        <div class="demo-field">
            <label class="demo-field-label" {..label_props.into_attrs()}>"Email address"</label>
            <input
                type="email"
                class="demo-input demo-text-input"
                prop:value=email
                on:input=move |e| email.set(event_target_value(&e))
                // Validity is up to you: `use_field` only connects the parts.
                aria-invalid=move || is_invalid.get().then_some("true")
                {..field_props.into_attrs()}
            />
            <p class="demo-field-description" {..description_props.into_attrs()}>
                "We\u{2019}ll never share your email."
            </p>
            // Rendered only while the value is invalid; the input references it only then.
            <Show when=move || is_invalid.get()>
                <p class="demo-field-error" {..error_message_props.clone().into_attrs()}>
                    "Enter an address like name@example.com."
                </p>
            </Show>
        </div>

        <p class="demo-status">
            {move || format!("The input is described by: {}", described_by.get().unwrap_or_default())}
        </p>
    }
}
