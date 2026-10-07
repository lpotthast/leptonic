use leptonic::atoms::button::Button;
use leptos::prelude::*;
use leptos_icons::Icon;

/// The result of the last action.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Outcome {
    Saved,
    UploadFailed,
}

#[component]
pub fn StatusAlertDemo() -> impl IntoView {
    let outcome = RwSignal::new(None::<Outcome>);

    view! {
        <div class="demo-inline-controls">
            <Button on_press=move |_| outcome.set(Some(Outcome::Saved)) classes="demo-btn">"Save"</Button>
            <Button on_press=move |_| outcome.set(Some(Outcome::UploadFailed)) classes="demo-btn">"Upload"</Button>
        </div>

        // A status region is announced politely, after what the screen reader is reading. It has to be in the page
        // before its message appears, so it is always rendered.
        <div role="status">
            {move || (outcome.get() == Some(Outcome::Saved)).then(|| view! {
                <div class="demo-alert" data-variant="success">
                    <span class="demo-alert-icon" aria-hidden="true"><Icon icon=icondata::BsCheckCircle/></span>
                    <div>
                        <p class="demo-alert-title">"Saved"</p>
                        <p class="demo-alert-text">"Your changes were saved."</p>
                    </div>
                </div>
            })}
        </div>

        // An alert is announced at once, interrupting the screen reader, also when it is added to the page.
        {move || (outcome.get() == Some(Outcome::UploadFailed)).then(|| view! {
            <div role="alert" class="demo-alert" data-variant="danger">
                <span class="demo-alert-icon" aria-hidden="true"><Icon icon=icondata::BsExclamationTriangle/></span>
                <div>
                    <p class="demo-alert-title">"Upload failed"</p>
                    <p class="demo-alert-text">"The file is larger than 10 MB."</p>
                </div>
            </div>
        })}
    }
}
