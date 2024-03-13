use indoc::indoc;
use leptonic::{atoms::link::AnchorLink, components::prelude::*, prelude::*};
use leptos::*;

use crate::pages::documentation::{article::Article, toc::Toc};

#[component]
pub fn PageToggle() -> impl IntoView {
    let (state, set_state) = create_signal(false);

    view! {
        <Article>
            <H1 id="toggle" class="anchor">
                "Toggle"
                <AnchorLink href="#toggle" description="Direct link to article header"/>
            </H1>

            <P>"A toggle is a representation of a boolean value."</P>

            <Code>
                {indoc!(r"
                    let (state, set_state) = create_signal(false);

                    view! {
                        <Toggle state=state set_state=set_state/>
                    }
                ")}
            </Code>

            <Toggle state=state set_state=set_state/>

            <H2 id="icons" class="anchor">
                "Icons"
                <AnchorLink href="#icons" description="Direct link to section: Icons"/>
            </H2>

            <P>"A toggle can be configured with a pair of icons. One icon being rendered in the off position, the other being rendered in the on position."</P>

            <Code>
                {indoc!(r"
                    let (state, set_state) = create_signal(false);

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

            <H2 id="variations" class="anchor">
                "Variations"
                <AnchorLink href="#variations" description="Direct link to section: Variations"/>
            </H2>

            <P>"The toggle comes in two variants: Sliding and Stationary. Sliding toggles are the default and the ones we have used so far."</P>
            <P>"Stationary toggles are not animated and only consist of a single circle."</P>

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
