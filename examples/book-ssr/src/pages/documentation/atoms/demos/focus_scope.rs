use leptonic::{atoms::prelude::FocusScope, components::prelude::*};
use leptos::prelude::*;

#[component]
pub fn FocusScopeDemo() -> impl IntoView {
    let open = RwSignal::new(false);
    let (status, set_status) = signal("The form is closed.");

    let close = move |message| {
        set_status.set(message);
        open.set(false);
    };

    view! {
        <Button on_press=move |_| {
            set_status.set("The form is open.");
            open.set(true);
        }>"Open form"</Button>

        <Show when=move || open.get()>
            // Tab stays inside the form; Submit and Cancel close it and focus returns to "Open form".
            <FocusScope contain=true restore_focus=true auto_focus=true>
                <div class="demo-focus-scope demo-mt-1">
                    <TextField label="First name"/>
                    <TextField label="Last name"/>
                    <div class="demo-focus-row">
                        <Button on_press=move |_| close("Submitted.")>"Submit"</Button>
                        <Button variant=ButtonVariant::Outlined on_press=move |_| close("Cancelled.")>"Cancel"</Button>
                    </div>
                </div>
            </FocusScope>
        </Show>

        <p class="demo-status">{move || status.get()}</p>
    }
}
