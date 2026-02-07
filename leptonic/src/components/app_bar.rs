use leptos::prelude::*;

use crate::Height;

#[component]
pub fn AppBar(#[prop(into, optional)] height: Option<Height>, children: Children) -> impl IntoView {
    view! {
        <leptonic-app-bar style=height
            .map(|it| ("--app-bar-height", format!("{it}")))>{children()}</leptonic-app-bar>
    }
}
