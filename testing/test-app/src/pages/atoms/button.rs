use leptonic::{
    atoms::prelude::{Button, Dialog, DialogTrigger, ModalBackdrop, ModalContent, ProgressBar},
    hooks::{ButtonFormAttributes, ButtonType, FormMethod},
    utils::aria::AriaCurrent,
};
use leptos::prelude::*;

/// Button atoms (react-aria-components' `Button.test.js` setups):
/// - `#test-button-basic`, counting presses in `#test-button-basic-count`; `#test-button-disabled`;
///   `#test-button-labelled` (`aria-label`, `aria-current`); `#test-button-form-props` (form
///   attributes of a button outside its form).
/// - Pending ("isPending"): `#test-button-pending` turns pending when pressed (presses counted in
///   `#test-button-pending-count`, reset by `#test-button-pending-reset`); a submit button in a
///   form with two text inputs, turning pending a moment after a press or by
///   `#test-button-pending-submit-toggle` (submits counted in `#test-button-pending-submits`);
///   `#test-button-pending-labelled`, pending and named by `aria-label`; a pending dialog trigger
///   (`#test-button-pending-trigger`).
#[component]
pub fn PageAtomButton() -> impl IntoView {
    let (basic_count, set_basic_count) = signal(0u32);
    let (disabled_count, set_disabled_count) = signal(0u32);
    let pending = RwSignal::new(false);
    let pending_count = RwSignal::new(0u32);
    let submit_pending = RwSignal::new(false);
    let submits = RwSignal::new(0u32);
    let trigger_focused = RwSignal::new(false);

    view! {
        <div id="test-page-atom-button">
            <h1>"Button Atom Test Page"</h1>

            <section>
                <h2>"Basic Button"</h2>
                <Button
                    on_press=move |_| set_basic_count.update(|c| *c += 1)
                    attr:id="test-button-basic"
                >
                    "Press me"
                </Button>
                <div>"Press count: " <span id="test-button-basic-count">{basic_count}</span></div>
            </section>

            <section>
                <h2>"Disabled Button"</h2>
                <Button
                    on_press=move |_| set_disabled_count.update(|c| *c += 1)
                    is_disabled=Signal::from(true)
                    attr:id="test-button-disabled"
                >
                    "Disabled"
                </Button>
                <div>
                    "Press count: " <span id="test-button-disabled-count">{disabled_count}</span>
                </div>
            </section>

            <section>
                <h2>"Labelled Button"</h2>
                // Icon-only: named by `aria_label`, marked as the current page.
                <Button
                    attr:id="test-button-labelled"
                    aria_label="Page 2"
                    aria_current=Some(AriaCurrent::Page)
                >
                    "2"
                </Button>
            </section>

            <section>
                <h2>"Form props"</h2>
                <form id="test-button-form"></form>
                <Button
                    attr:id="test-button-form-props"
                    button_type=ButtonType::Submit
                    form=ButtonFormAttributes {
                        form: Some("test-button-form".into()),
                        form_method: Some(FormMethod::Post),
                        name: Some("action".into()),
                        value: Some("save".into()),
                        ..ButtonFormAttributes::default()
                    }
                >
                    "Submit elsewhere"
                </Button>
            </section>

            <section>
                <h2>"Pending"</h2>
                <Button
                    attr:id="test-button-pending"
                    is_pending=pending
                    on_press=move |_| {
                        pending.set(true);
                        pending_count.update(|c| *c += 1);
                    }
                >
                    "Save"
                    <ProgressBar aria_label="loading" value={None::<u32>} />
                </Button>
                <div>"Presses: " <span id="test-button-pending-count">{pending_count}</span></div>
                <button id="test-button-pending-reset" on:click=move |_| pending.set(false)>
                    "Reset"
                </button>

                <form on:submit=move |e| {
                    e.prevent_default();
                    submits.update(|s| *s += 1);
                }>
                    <input id="test-button-pending-input-1" type="text" />
                    <input id="test-button-pending-input-2" type="text" />
                    <Button
                        attr:id="test-button-pending-submit"
                        button_type=ButtonType::Submit
                        is_pending=submit_pending
                        on_press=move |_| {
                            // Later: setting it at once would turn the button into a plain one
                            // before the click submits the form.
                            set_timeout(move || submit_pending.set(true), std::time::Duration::ZERO);
                        }
                    >
                        "Submit"
                    </Button>
                </form>
                <div>"Submits: " <span id="test-button-pending-submits">{submits}</span></div>
                <button
                    id="test-button-pending-submit-toggle"
                    on:click=move |_| submit_pending.update(|p| *p = !*p)
                >
                    "Toggle submit pending"
                </button>

                <Button attr:id="test-button-pending-labelled" aria_label="Upload" is_pending=true>
                    "\u{2191}"
                    <ProgressBar aria_label="uploading" value={None::<u32>} />
                </Button>

                <DialogTrigger>
                    <Button
                        attr:id="test-button-pending-trigger"
                        is_pending=true
                        on_focus_change=move |focused| trigger_focused.set(focused)
                    >
                        "Delete…"
                    </Button>
                    <ModalBackdrop is_dismissable=true>
                        <ModalContent>
                            <Dialog aria_label="Alert">"Deleted"</Dialog>
                        </ModalContent>
                    </ModalBackdrop>
                </DialogTrigger>
                <div>
                    "Trigger focused: "
                    <span id="test-button-pending-trigger-focused">
                        {move || trigger_focused.get().to_string()}
                    </span>
                </div>
            </section>

        </div>
    }
}
