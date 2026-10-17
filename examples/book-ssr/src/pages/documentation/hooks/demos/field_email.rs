use leptonic::hooks::form::{UseFieldInput, UseFieldReturn, use_field};
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
        label,
        field,
        description,
        error_message,
        description_id,
        error_message_id,
    } = use_field(UseFieldInput {
        has_label: true.into(),
        ..UseFieldInput::default()
    });
    // The ids the input is described by, shown below the demo.
    let described_by = Signal::derive(move || {
        [description_id.get(), error_message_id.get()]
            .into_iter()
            .flatten()
            .map(|id| id.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    });

    view! {
        <div class="demo-field">
            <label class="demo-field-label" {..label.attrs()}>"Email address"</label>
            <input
                type="email"
                class="demo-input demo-text-input"
                prop:value=email
                on:input=move |e| email.set(event_target_value(&e))
                // Validity is up to you: `use_field` only connects the parts.
                aria-invalid=move || is_invalid.get().then_some("true")
                {..field.attrs()}
            />
            <p class="demo-field-description" {..description.attrs()}>
                "We\u{2019}ll never share your email."
            </p>
            // Rendered only while the value is invalid; the input references it only then.
            <Show when=move || is_invalid.get()>
                <p class="demo-field-error" {..error_message.attrs()}>
                    "Enter an address like name@example.com."
                </p>
            </Show>
        </div>

        <p class="demo-status">
            {move || format!("The input is described by: {}", described_by.get())}
        </p>
    }
}
