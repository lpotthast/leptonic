use leptonic::components::prelude::*;
use leptos::prelude::*;

/// The theme ships no styles for `Grow`: the animation comes from the `demo-transitions-grow` class.
#[component]
pub fn TransitionsGrowDemo() -> impl IntoView {
    let (visible, set_visible) = signal(true);

    view! {
        <div class="demo-transitions-demo">
            <Button on_press=move |_| set_visible.update(|visible| *visible = !*visible)>
                {move || if visible.get() { "Shrink" } else { "Grow" }}
            </Button>
            <Grow inn=visible.into() classes="demo-transitions-grow">
                <div class="demo-transitions-panel">"Grow scales the panel up from its top edge while fading it in."</div>
            </Grow>
        </div>
    }
}
