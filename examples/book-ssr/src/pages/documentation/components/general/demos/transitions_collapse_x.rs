use leptonic::{components::prelude::*, prelude::*};
use leptos::prelude::*;

#[component]
pub fn TransitionsCollapseXDemo() -> impl IntoView {
    let (expanded, set_expanded) = signal(true);

    view! {
        <div class="demo-transitions-demo">
            <Button
                on_press=move |_| set_expanded.update(|expanded| *expanded = !*expanded)
                aria_expanded=Signal::derive(move || Some(AriaExpanded::from(expanded.get())))
            >
                {move || if expanded.get() { "Hide sidebar" } else { "Show sidebar" }}
            </Button>
            <div class="demo-transitions-row">
                <Collapse show=expanded axis=CollapseAxis::X>
                    <nav class="demo-transitions-sidebar">"Sidebar"</nav>
                </Collapse>
                <div class="demo-transitions-panel demo-transitions-main">"Main content takes the remaining width."</div>
            </div>
        </div>
    }
}
