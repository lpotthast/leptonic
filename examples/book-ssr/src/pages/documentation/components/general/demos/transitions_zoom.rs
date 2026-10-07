use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn TransitionsZoomDemo() -> impl IntoView {
    let (visible, set_visible) = signal(true);

    view! {
        <div class="demo-transitions-demo">
            <Button on_press=move |_| set_visible.update(|visible| *visible = !*visible)>
                {move || if visible.get() { "Zoom out" } else { "Zoom in" }}
            </Button>
            <Zoom is_shown=visible>
                <div class="demo-transitions-panel">"Zoom scales the panel up from nothing."</div>
            </Zoom>
        </div>
    }
}
