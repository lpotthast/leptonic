use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn PageTransition(cx: Scope) -> impl IntoView {
    let (transition_collapse_h, set_transition_collapse_h) = create_signal(cx, false);
    let (transition_collapse_v, set_transition_collapse_v) = create_signal(cx, false);
    let (transition_fade, set_transition_fade) = create_signal(cx, false);
    //let (transition_grow, set_transition_grow) = create_signal(cx, false);
    //let (transition_slide, set_transition_slide) = create_signal(cx, false);
    //let (transition_zoom, set_transition_zoom) = create_signal(cx, false);

    view! { cx,
        <H1>"Transitions"</H1>

        <H2>"Transition - Collapse - horizontally"</H2>
        <Toggle state=transition_collapse_h set_state=set_transition_collapse_h/>
        <Collapse show=transition_collapse_h axis=CollapseAxis::X>
            <Skeleton height=Size::Em(5.0) width=Size::Percent(100.0)>"Collapse"</Skeleton>
        </Collapse>

        <Separator />

        <H2>"Transition - Collapse - vertically"</H2>
        <Toggle state=transition_collapse_v set_state=set_transition_collapse_v/>
        <Collapse show=transition_collapse_v axis=CollapseAxis::Y>
            <Skeleton height=Size::Em(5.0)>"Collapse"</Skeleton>
        </Collapse>

        <Separator />

        <H2>"Transition - Fade"</H2>
        <Toggle state=transition_fade set_state=set_transition_fade/>
        <Fade inn=Signal::derive(cx, move || transition_fade.get())>
            <Skeleton>"Fade"</Skeleton>
        </Fade>

        <Separator />

        //<H2>"Transition - Grow"</H2>
        //<Toggle state=transition_grow set_state=set_transition_grow/>
        //<Grow inn=Signal::derive(cx, move || transition_grow.get())>
        //    <Skeleton>"Grow"</Skeleton>
        //</Grow>

        //<Separator />

        //<H2>"Transition - Slide"</H2>
        //<Toggle state=transition_slide set_state=set_transition_slide/>
        //<Slide inn=Signal::derive(cx, move || transition_slide.get())>
        //    <Skeleton>"Slide"</Skeleton>
        //</Slide>

        //<Separator />

        //<H2>"Transition - Zoom"</H2>
        //<Toggle state=transition_zoom set_state=set_transition_zoom/>
        //<Zoom inn=Signal::derive(cx, move || transition_zoom.get())>
        //    <Skeleton>"Zoom"</Skeleton>
        //</Zoom>
    }
}
