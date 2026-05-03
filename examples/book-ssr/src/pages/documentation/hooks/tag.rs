use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use crate::pages::documentation::{article::Article, demo_shell::DemoShell, toc::Toc};

use super::demos::tag::TagDemo;

#[component]
pub fn PageUseTag() -> impl IntoView {
    view! {
        <Article>
            <h1 id="use_tag" class="anchor">
                "use_tag"
                <AnchorLink href="#use_tag" description="Direct link to article header"/>
            </h1>

            <p>
                "Hooks for creating accessible tag groups with selection and removal capabilities. "
                "See the "<Link href=crate::routes::doc::Chip.materialize()>"Chip overview"</Link>" for concept guidance."
            </p>

            <p>
                "Based on react-aria\u{2019}s "
                <LinkExt href="https://react-spectrum.adobe.com/react-aria/useTagGroup.html" target=LinkTarget::_Blank>
                    "useTagGroup"
                </LinkExt>
                "."
            </p>

            <h2 id="demo" class="anchor">
                "Interactive Demo"
                <AnchorLink href="#demo" description="Direct link to demo"/>
            </h2>

            <p>"Click the x button or press Delete/Backspace to remove a tag:"</p>

            <DemoShell source=include_str!("demos/tag.rs")>
                <TagDemo />
            </DemoShell>

            <Code language=Language::Rust>
                {indoc!(r#"
                    let UseTagGroupReturn { group_props, label_props, .. } = use_tag_group(UseTagGroupInput {
                        label: Some("Categories".to_string()),
                        selection_mode: TagGroupSelectionMode::Multiple,
                        allow_removal: true,
                        on_remove: Some(Callback::new(|key| remove_tag(key))),
                        ..Default::default()
                    });

                    let tag = use_tag(UseTagInput {
                        tag_key: "rust".to_string(),
                        is_selected: Signal::derive(|| false),
                        is_focused: Signal::derive(|| true),
                        is_disabled: Signal::derive(|| false),
                        allow_removal: true,
                        on_remove: Some(Callback::new(|_| remove_this_tag())),
                        ..Default::default()
                    });

                    view! {
                        <div {..group_props}>
                            <div {..tag.row_props.into_attrs()}>
                                <span {..tag.cell_props.into_attrs()}>"Rust"</span>
                                <button {..tag.remove_button_props.into_attrs()}>"×"</button>
                            </div>
                        </div>
                    }
                "#)}
            </Code>

            <h2 id="selection-modes" class="anchor">
                "Selection Modes"
                <AnchorLink href="#selection-modes" description="Direct link to selection modes"/>
            </h2>

            <ul>
                <li><code>"TagGroupSelectionMode::None"</code> " - No selection (default)"</li>
                <li><code>"TagGroupSelectionMode::Single"</code> " - Single tag selection"</li>
                <li><code>"TagGroupSelectionMode::Multiple"</code> " - Multiple tag selection"</li>
            </ul>

            <h2 id="aria-attributes" class="anchor">
                "ARIA Attributes"
                <AnchorLink href="#aria-attributes" description="Direct link to ARIA attributes"/>
            </h2>

            <p>"The hooks automatically set:"</p>
            <ul>
                <li><code>"role=\"grid\""</code> " on the group"</li>
                <li><code>"role=\"row\""</code> " on each tag"</li>
                <li><code>"role=\"gridcell\""</code> " on the tag content"</li>
                <li><code>"aria-selected"</code> " for selection state"</li>
                <li><code>"aria-disabled"</code> " for disabled tags"</li>
                <li><code>"aria-label=\"Remove\""</code> " on remove button"</li>
            </ul>

            <h2 id="keyboard-navigation" class="anchor">
                "Keyboard Navigation"
                <AnchorLink href="#keyboard-navigation" description="Direct link to keyboard"/>
            </h2>

            <ul>
                <li><code>"Arrow Left/Up"</code> " - Focus previous tag"</li>
                <li><code>"Arrow Right/Down"</code> " - Focus next tag"</li>
                <li><code>"Enter/Space"</code> " - Select/toggle tag"</li>
                <li><code>"Delete/Backspace"</code> " - Remove tag (if removable)"</li>
            </ul>

            <h2 id="features" class="anchor">
                "Features"
                <AnchorLink href="#features" description="Direct link to features"/>
            </h2>

            <ul>
                <li>"Multiple selection modes"</li>
                <li>"Removable tags"</li>
                <li>"Keyboard navigation"</li>
                <li>"Focus management"</li>
                <li>"Full ARIA support"</li>
            </ul>
            <h2 id="see-also" class="anchor">
                "See Also"
                <AnchorLink href="#see-also" description="Direct link to section: See Also"/>
            </h2>

            <ul>
                <li><Link href=crate::routes::doc::Chip.materialize()>"Chip overview"</Link></li>
                <li><Link href=crate::routes::doc::chip::Component.materialize()>"Chip component"</Link></li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "use_tag", link: "#use_tag" },
                Toc::Leaf { title: "Demo", link: "#demo" },
                Toc::Leaf { title: "Selection Modes", link: "#selection-modes" },
                Toc::Leaf { title: "ARIA Attributes", link: "#aria-attributes" },
                Toc::Leaf { title: "Keyboard Navigation", link: "#keyboard-navigation" },
                Toc::Leaf { title: "Features", link: "#features" },
                Toc::Leaf { title: "See Also", link: "#see-also" },
            ]
        }/>
    }
}
