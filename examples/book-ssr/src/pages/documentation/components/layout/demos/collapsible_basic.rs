use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn CollapsibleBasicDemo() -> impl IntoView {
    let disabled = RwSignal::new(false);
    // App state: whether the body is open.
    let expanded = RwSignal::new(false);

    view! {
        <Collapsible is_expanded=expanded set_expanded=expanded is_disabled=disabled>
            <CollapsibleHeader slot>"Shipping"</CollapsibleHeader>
            <CollapsibleBody slot>"Orders ship within two working days, in recyclable packaging."</CollapsibleBody>
        </Collapsible>

        <div class="demo-controls">
            <Checkbox is_selected=disabled set_selected=disabled>"Disabled"</Checkbox>
        </div>
        <p class="demo-status">{move || if expanded.get() { "Open" } else { "Closed" }}</p>
    }
}
