use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn TransitionsFadeDemo() -> impl IntoView {
    let (visible, set_visible) = signal(true);

    view! {
        <div class="demo-transitions-demo">
            <Button on_press=move |_| set_visible.update(|visible| *visible = !*visible)>
                {move || if visible.get() { "Fade out" } else { "Fade in" }}
            </Button>
            <Fade is_shown=visible>
                <div class="demo-transitions-panel">"Fade animates the opacity. The panel keeps its space while hidden."</div>
            </Fade>
        </div>
    }
}
