//! Transitions showing and hiding their children with CSS (the theme's `transition.scss`):
//! `data-shown` while shown; hidden content is `inert` (not focusable, hidden from assistive
//! technology) and invisible once the transition ended. Reduced motion skips the animation.

use leptos::prelude::*;

use crate::utils::{classes::Classes, data_attributes::flag, styles::Styles};

pub mod collapse;
pub mod fade;
pub mod grow;
pub mod slide;
pub mod zoom;

/// The element of a transition: `class` names the kind (the theme animates it).
fn transition(
    class: &'static str,
    is_shown: Signal<bool>,
    classes: Classes,
    styles: Styles,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            class=classes.add(class)
            style=styles
            data-shown=flag(is_shown)
            inert=move || !is_shown.get()
        >
            {children()}
        </div>
    }
}
