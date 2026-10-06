use leptonic::{
    atoms::focus_scope::FocusScope,
    components::prelude::{Button, ButtonVariant},
};
use leptos::prelude::*;

#[component]
pub fn FocusScopeDemo() -> impl IntoView {
    let (open, set_open) = signal(false);

    view! {
        <Button on_press=move |_| set_open.set(true)>"Open form"</Button>

        <Show when=move || open.get()>
            <FocusScope contain=true restore_focus=true auto_focus=true>
                <div class="demo-focus-scope demo-mt-1">
                    <input type="text" placeholder="First name" class="demo-focus-item"/>
                    <input type="text" placeholder="Last name" class="demo-focus-item"/>
                    <div class="demo-focus-row">
                        <Button on_press=move |_| set_open.set(false)>"Submit"</Button>
                        <Button variant=ButtonVariant::Outlined on_press=move |_| set_open.set(false)>"Cancel"</Button>
                    </div>
                </div>
            </FocusScope>
        </Show>
    }
}
