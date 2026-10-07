use leptos::prelude::*;

use crate::{
    Height,
    hooks::{IntoAttrs, LandmarkRole, UseLandmarkInput, UseLandmarkReturn, use_landmark},
    utils::{CapturedElement, classes::Classes, css::css_custom_property, styles::Styles},
};

css_custom_property!(APP_BAR_HEIGHT: Height = "--app-bar-height");

/// The bar at the top of an app: a banner landmark (F6 reaches it).
#[component]
pub fn AppBar(
    #[prop(into, optional)] height: Option<Height>,
    /// Names the banner, e.g. when a page has several.
    #[prop(into, optional)]
    aria_label: MaybeProp<String>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let styles = match height {
        Some(h) => styles.add(APP_BAR_HEIGHT.declare(h)),
        None => styles,
    };
    let element = CapturedElement::new();
    let UseLandmarkReturn { props } = use_landmark(
        UseLandmarkInput {
            role: LandmarkRole::Banner,
            aria_label,
            aria_labelledby: None,
            focus: None,
        },
        element,
    );
    view! {
        <header
            {..props.into_attrs()}
            {..element.attr()}
            class=classes.add("leptonic-app-bar")
            style=styles
        >
            {children()}
        </header>
    }
}
