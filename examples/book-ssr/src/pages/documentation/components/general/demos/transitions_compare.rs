use leptonic::components::prelude::*;
use leptos::prelude::*;

/// All five transitions side by side, driven by one signal. Grow, Slide and Zoom get their animation from the
/// demo's classes (see "View styles"); the theme only styles Collapse and Fade.
#[component]
pub fn TransitionsCompareDemo() -> impl IntoView {
    let (shown, set_shown) = signal(true);

    view! {
        <div class="demo-transitions-demo">
            <Button on_press=move |_| set_shown.update(|shown| *shown = !*shown)>
                {move || if shown.get() { "Hide all" } else { "Show all" }}
            </Button>
            <div class="demo-transitions-grid">
                <div class="demo-transitions-cell">
                    <span class="demo-transitions-label">"Collapse"</span>
                    <Collapse show=shown>
                        <div class="demo-transitions-tile">"Collapse"</div>
                    </Collapse>
                </div>
                <div class="demo-transitions-cell">
                    <span class="demo-transitions-label">"Fade"</span>
                    <Fade inn=shown>
                        <div class="demo-transitions-tile">"Fade"</div>
                    </Fade>
                </div>
                <div class="demo-transitions-cell">
                    <span class="demo-transitions-label">"Grow"</span>
                    <Grow inn=shown.into() classes="demo-transitions-grow">
                        <div class="demo-transitions-tile">"Grow"</div>
                    </Grow>
                </div>
                <div class="demo-transitions-cell demo-transitions-clip">
                    <span class="demo-transitions-label">"Slide"</span>
                    <Slide inn=shown.into() classes="demo-transitions-slide">
                        <div class="demo-transitions-tile">"Slide"</div>
                    </Slide>
                </div>
                <div class="demo-transitions-cell">
                    <span class="demo-transitions-label">"Zoom"</span>
                    <Zoom inn=shown.into() classes="demo-transitions-zoom">
                        <div class="demo-transitions-tile">"Zoom"</div>
                    </Zoom>
                </div>
            </div>
        </div>
    }
}
