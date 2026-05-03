use leptonic::{prelude::*, utils::css::{em, pct}};
use leptos::prelude::*;

use crate::pages::documentation::article::Article;

#[component]
pub fn PageTransition() -> impl IntoView {
    let (transition_collapse_h, set_transition_collapse_h) = signal(false);
    let (transition_collapse_v, set_transition_collapse_v) = signal(false);
    let (transition_fade, set_transition_fade) = signal(false);
    //let (transition_grow, set_transition_grow) = signal(false);
    //let (transition_slide, set_transition_slide) = signal(false);
    //let (transition_zoom, set_transition_zoom) = signal(false);

    view! {
        <Article>
            <h1>"Transitions"</h1>

            <h2>"Transition - Collapse - horizontally"</h2>
            <Toggle state=transition_collapse_h set_state=set_transition_collapse_h/>
            <Collapse show=transition_collapse_h axis=CollapseAxis::X>
                <Skeleton height=em(5.0) width=pct(100.0)>"Collapse"</Skeleton>
            </Collapse>

            <Separator />

            <h2>"Transition - Collapse - vertically"</h2>
            <Toggle state=transition_collapse_v set_state=set_transition_collapse_v/>
            <Collapse show=transition_collapse_v axis=CollapseAxis::Y>
                <Skeleton height=em(5.0)>"Collapse"</Skeleton>
            </Collapse>

            <Separator />

            <h2>"Transition - Fade"</h2>
            <Toggle state=transition_fade set_state=set_transition_fade/>
            <Fade inn=Signal::derive(move || transition_fade.get())>
                <Skeleton>"Fade"</Skeleton>
            </Fade>

            <Separator />

            //<h2>"Transition - Grow"</h2>
            //<Toggle state=transition_grow set_state=set_transition_grow/>
            //<Grow inn=Signal::derive(move || transition_grow.get())>
            //    <Skeleton>"Grow"</Skeleton>
            //</Grow>

            //<Separator />

            //<h2>"Transition - Slide"</h2>
            //<Toggle state=transition_slide set_state=set_transition_slide/>
            //<Slide inn=Signal::derive(move || transition_slide.get())>
            //    <Skeleton>"Slide"</Skeleton>
            //</Slide>

            //<Separator />

            //<h2>"Transition - Zoom"</h2>
            //<Toggle state=transition_zoom set_state=set_transition_zoom/>
            //<Zoom inn=Signal::derive(move || transition_zoom.get())>
            //    <Skeleton>"Zoom"</Skeleton>
            //</Zoom>
        </Article>
    }
}
