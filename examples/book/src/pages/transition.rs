use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn PageTransition(cx: Scope) -> impl IntoView {
    let (transition_collapse, set_transition_collapse) = create_signal(cx, false);
    let (transition_fade, set_transition_fade) = create_signal(cx, false);
    let (transition_grow, set_transition_grow) = create_signal(cx, false);
    let (transition_slide, set_transition_slide) = create_signal(cx, false);
    let (transition_zoom, set_transition_zoom) = create_signal(cx, false);

    view! { cx,
        <H2>"Transitions"</H2>

        <h2>"Transition - Collapse"</h2>
        <Toggle state=transition_collapse on_toggle=move |s| set_transition_collapse.set(s) />
        <Collapse show=transition_collapse axis=CollapseAxis::X>
            <Skeleton height=Size::Em(5.0)>"Collapse"</Skeleton>
        </Collapse>

        <Toggle state=transition_collapse on_toggle=move |s| set_transition_collapse.set(s) />
        <Collapse show=transition_collapse axis=CollapseAxis::Y>
            <Skeleton height=Size::Em(5.0)>"Collapse"</Skeleton>
        </Collapse>

        <Separator />

        <h2>"Transition - Fade"</h2>
        <Toggle state=transition_fade on_toggle=move |s| set_transition_fade.set(s) />
        <Fade inn=Signal::derive(cx, move || transition_fade.get())>
            <Skeleton>"Fade"</Skeleton>
        </Fade>

        <Separator />

        <h2>"Transition - Grow"</h2>
        <Toggle state=transition_grow on_toggle=move |s| set_transition_grow.set(s) />
        <Grow inn=Signal::derive(cx, move || transition_grow.get())>
            <Skeleton>"Grow"</Skeleton>
        </Grow>

        <Separator />

        <h2>"Transition - Slide"</h2>
        <Toggle state=transition_slide on_toggle=move |s| set_transition_slide.set(s) />
        <Slide inn=Signal::derive(cx, move || transition_slide.get())>
            <Skeleton>"Slide"</Skeleton>
        </Slide>

        <Separator />

        <h2>"Transition - Zoom"</h2>
        <Toggle state=transition_zoom on_toggle=move |s| set_transition_zoom.set(s) />
        <Zoom inn=Signal::derive(cx, move || transition_zoom.get())>
            <Skeleton>"Zoom"</Skeleton>
        </Zoom>
    }
}
