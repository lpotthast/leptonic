use leptonic::{
    atoms::prelude::FocusScope,
    components::prelude::*,
    utils::{classes::Classes, css::em},
};
use leptos::prelude::*;

#[component]
pub fn FocusManagerScopeDemo() -> impl IntoView {
    view! {
        <style>
            ".focus-trap-demo button:focus, .focus-trap-demo input:focus {
                outline: 3px solid #4a9eff;
                outline-offset: 2px;
            }"
        </style>

        <FocusScope contain=true>
            <div
                class="focus-trap-demo"
                style="
                    border: 3px solid #4a9eff;
                    padding: 1.5em;
                    border-radius: 8px;
                    background: rgba(74, 158, 255, 0.1);
                "
            >
                <p class=Classes::from("demo-container-title")>"Focus Trap Container (Tab cycles within)"</p>
                <Stack orientation=StackOrientation::Horizontal spacing=em(0.5)>
                    <button class=Classes::from("demo-btn-solid")>
                        "Trapped 1"
                    </button>
                    <button class=Classes::from("demo-btn-solid")>
                        "Trapped 2"
                    </button>
                    <input
                        type="text"
                        placeholder="Trapped input"
                        class=Classes::from("demo-input-solid")
                    />
                    <button class=Classes::from("demo-btn-solid")>
                        "Trapped 3"
                    </button>
                </Stack>
            </div>
        </FocusScope>
    }
}
