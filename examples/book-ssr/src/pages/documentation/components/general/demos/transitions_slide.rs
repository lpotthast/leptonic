use leptonic::components::prelude::*;
use leptos::prelude::*;

/// The frame around the transition clips the panel while it is outside.
#[component]
pub fn TransitionsSlideDemo() -> impl IntoView {
    let (visible, set_visible) = signal(true);

    view! {
        <div class="demo-transitions-demo">
            <Button on_press=move |_| set_visible.update(|visible| *visible = !*visible)>
                {move || if visible.get() { "Slide out" } else { "Slide in" }}
            </Button>
            <div class="demo-transitions-clip">
                <Slide is_shown=visible>
                    <div class="demo-transitions-panel">"Slide moves the panel in from below while fading it in."</div>
                </Slide>
            </div>
        </div>
    }
}
