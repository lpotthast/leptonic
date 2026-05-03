use leptonic::atoms::focus_scope::FocusScope;
use leptos::prelude::*;

#[component]
pub fn FocusScopeDemo() -> impl IntoView {
    view! {
        <FocusScope contain=true>
            <div style="display: flex; flex-direction: column; gap: 0.5em; padding: 1em; border: 2px solid var(--brand-color); border-radius: 0.5em;">
                <input type="text" placeholder="First name" />
                <input type="text" placeholder="Last name" />
                <button>"Submit"</button>
            </div>
        </FocusScope>
    }
}
