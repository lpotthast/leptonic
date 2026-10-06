use leptonic::components::prelude::*;
use leptos::prelude::*;

/// The theme ships no styles for `Zoom`: the animation comes from the `demo-transitions-zoom` class.
#[component]
pub fn TransitionsZoomDemo() -> impl IntoView {
    let (visible, set_visible) = signal(true);

    view! {
        <div class="demo-transitions-demo">
            <Button on_press=move |_| set_visible.update(|visible| *visible = !*visible)>
                {move || if visible.get() { "Zoom out" } else { "Zoom in" }}
            </Button>
            <Zoom inn=visible.into() classes="demo-transitions-zoom">
                <div class="demo-transitions-panel">"Zoom scales the panel up from its center."</div>
            </Zoom>
        </div>
    }
}
