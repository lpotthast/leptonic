use leptos::prelude::*;

use crate::{
    Height,
    utils::{classes::Classes, css::css_custom_property, styles::Styles},
};

css_custom_property!(APP_BAR_HEIGHT: Height = "--app-bar-height");

#[component]
pub fn AppBar(
    #[prop(into, optional)] height: Option<Height>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let styles = match height {
        Some(h) => styles.add(APP_BAR_HEIGHT.declare(h)),
        None => styles,
    };
    view! {
        <header class=classes.add("leptonic-app-bar") style=styles>
            {children()}
        </header>
    }
}
