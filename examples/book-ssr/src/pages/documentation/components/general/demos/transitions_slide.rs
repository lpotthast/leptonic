use leptonic::components::prelude::*;
use leptos::prelude::*;

/// The theme ships no styles for `Slide`: the animation comes from the `demo-transitions-slide` class.
#[component]
pub fn TransitionsSlideDemo() -> impl IntoView {
    let (visible, set_visible) = signal(true);

    view! {
        <div class="demo-transitions-demo">
            <Button on_press=move |_| set_visible.update(|visible| *visible = !*visible)>
                {move || if visible.get() { "Slide out" } else { "Slide in" }}
            </Button>
            <div class="demo-transitions-clip">
                <Slide inn=visible.into() classes="demo-transitions-slide">
                    <div class="demo-transitions-panel">"Slide moves the panel in from the left. The frame clips it while it is outside."</div>
                </Slide>
            </div>
        </div>
    }
}
