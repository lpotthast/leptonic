use leptonic::{components::prelude::*, prelude::*};
use leptos::prelude::*;

#[component]
pub fn TransitionsCollapseDemo() -> impl IntoView {
    let (expanded, set_expanded) = signal(false);

    view! {
        <div class="demo-transitions-demo">
            <Button
                on_press=move |_| set_expanded.update(|expanded| *expanded = !*expanded)
                aria_expanded=Signal::derive(move || Some(AriaExpanded::from(expanded.get())))
            >
                {move || if expanded.get() { "Hide details" } else { "Show details" }}
            </Button>
            <Collapse show=expanded>
                <div class="demo-transitions-panel">
                    <p>"Collapse animates the height of this panel from zero to its content height and back."</p>
                    <p>"Content below the panel moves along with it."</p>
                </div>
            </Collapse>
            <p class="demo-caption">"Text after the collapse."</p>
        </div>
    }
}
