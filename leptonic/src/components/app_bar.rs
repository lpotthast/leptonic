use leptos::prelude::*;

use crate::{
    Height,
    utils::{classes::Classes, styles::Styles},
};

#[component]
pub fn AppBar(
    #[prop(into, optional)] height: Option<Height>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let styles = match height {
        Some(h) => styles.add("--app-bar-height", h),
        None => styles,
    };
    view! {
        <div class=classes.add("leptonic-app-bar") style=styles>
            {children()}
        </div>
    }
}
