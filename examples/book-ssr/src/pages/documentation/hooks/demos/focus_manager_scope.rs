use leptonic::{atoms::prelude::FocusScope, utils::classes::Classes};
use leptos::prelude::*;

#[component]
pub fn FocusManagerScopeDemo() -> impl IntoView {
    view! {
        <FocusScope contain=true>
            <div class=Classes::from("demo-focus-scope")>
                <p class=Classes::from("demo-container-title")>"Focus trap (Tab cycles within)"</p>
                <div class=Classes::from("demo-focus-row")>
                    <button class=Classes::from("demo-focus-item")>"Trapped 1"</button>
                    <button class=Classes::from("demo-focus-item")>"Trapped 2"</button>
                    <input type="text" placeholder="Trapped input" class=Classes::from("demo-focus-item")/>
                    <button class=Classes::from("demo-focus-item")>"Trapped 3"</button>
                </div>
            </div>
        </FocusScope>
    }
}
