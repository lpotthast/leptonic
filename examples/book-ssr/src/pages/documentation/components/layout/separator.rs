use indoc::indoc;
use leptonic::{atoms::link::AnchorLink, components::prelude::*};
use leptos::prelude::*;

use crate::pages::documentation::{article::Article, toc::Toc};

#[component]
pub fn PageSeparator() -> impl IntoView {
    view! {
        <Article>
            <h1 id="separator" class="anchor">
                "Separator"
                <AnchorLink href="#separator" description="Direct link to article header"/>
            </h1>

            <Code>
                {indoc!(r"
                    <Separator />
                ")}
            </Code>

            <Separator />
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Separator", link: "#separator" },
            ]
        }/>
    }
}
