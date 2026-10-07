use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn TransitionsGrowDemo() -> impl IntoView {
    let (visible, set_visible) = signal(true);

    view! {
        <div class="demo-transitions-demo">
            <Button on_press=move |_| set_visible.update(|visible| *visible = !*visible)>
                {move || if visible.get() { "Shrink" } else { "Grow" }}
            </Button>
            <Grow is_shown=visible>
                <div class="demo-transitions-panel">"Grow scales the panel up from its center while fading it in."</div>
            </Grow>
        </div>
    }
}
