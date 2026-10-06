use leptonic::components::prelude::*;
use leptos::prelude::*;

use crate::routes;

#[component]
pub fn PageWelcome() -> impl IntoView {
    view! {
        <div id="welcome-page">
            <div id="intro">
                <h1 id="slogan">"LEPTONIC"</h1>
                <p id="sub-slogan">"Accessible components for Leptos, from low-level hooks to themed UI."</p>
                <LinkButton href=routes::doc::Overview.materialize() size=ButtonSize::Big classes="cta">
                    "Read the docs"
                </LinkButton>
            </div>

            <ul id="features">
                <Feature title="Accessible">
                    "Keyboard, pointer, touch and screen reader interaction, ported from react-aria\u{2019}s battle-tested hooks."
                </Feature>
                <Feature title="Three layers">
                    "Use the hooks for full control, unstyled atoms for your own design system, or themed components."
                </Feature>
                <Feature title="Rust all the way">
                    "Typed APIs, server-side rendering and hydration, and internationalization without a JS runtime."
                </Feature>
            </ul>
        </div>
    }
}

#[component]
fn Feature(title: &'static str, children: Children) -> impl IntoView {
    view! {
        <li class="feature">
            <h2>{title}</h2>
            <p>{children()}</p>
        </li>
    }
}
