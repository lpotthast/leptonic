use indoc::indoc;
use leptonic::{components::prelude::*, prelude::*};
use leptos::prelude::*;

use crate::pages::documentation::{article::Article, toc::Toc};

#[component]
pub fn PageToggle() -> impl IntoView {
    let (state, set_state) = signal(false);

    view! {
        <Article>
            <h1 id="toggle" class="anchor">
                "Toggle"
                <AnchorLink href="#toggle" description="Direct link to article header"/>
            </h1>

            <p>"A toggle is a representation of a boolean value."</p>

            <Code>
                {indoc!(r"
                    let (state, set_state) = signal(false);

                    view! {
                        <Toggle state=state set_state=set_state/>
                    }
                ")}
            </Code>

            <Toggle state=state set_state=set_state/>

            <h2 id="icons" class="anchor">
                "Icons"
                <AnchorLink href="#icons" description="Direct link to section: Icons"/>
            </h2>

            <p>"A toggle can be configured with a pair of icons. One icon being rendered in the off position, the other being rendered in the on position."</p>

            <Code>
                {indoc!(r"
                    let (state, set_state) = signal(false);

                    view! {
                        <Toggle state=state set_state=set_state icons=ToggleIcons {
                            on: icondata::BsFolderFill,
                            off: icondata::BsFolder,
                        }/>
                    }
                ")}
            </Code>

            <Toggle state=state set_state=set_state icons=ToggleIcons {
                on: icondata::BsFolderFill,
                off: icondata::BsFolder,
            }/>

            <h2 id="variations" class="anchor">
                "Variations"
                <AnchorLink href="#variations" description="Direct link to section: Variations"/>
            </h2>

            <p>"The toggle comes in two variants: Sliding and Stationary. Sliding toggles are the default and the ones we have used so far."</p>
            <p>"Stationary toggles are not animated and only consist of a single circle."</p>

            <Toggle state=state set_state=set_state variant=ToggleVariant::Stationary icons=ToggleIcons {
                on: icondata::BsFolderFill,
                off: icondata::BsFolder,
            }/>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Toggle", link: "#toggle" },
                Toc::Leaf { title: "Icons", link: "#icons" },
                Toc::Leaf { title: "Variations", link: "#variations" },
            ]
        }/>
    }
}
