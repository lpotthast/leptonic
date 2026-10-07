use leptonic::{
    ScrollBehavior,
    atoms::prelude::{
        AnchorLink, Button, CurrentMatch, Link, Menu, MenuItems, MenuTrigger, Popover,
    },
    hooks::{LinkTarget, UseLinkInput, collections::use_collection, use_link},
};
use leptos::prelude::*;

/// Link atoms (react-aria-components' `Link.test.js` setups), on `/atoms/link`:
/// - `#test-link-self`: this page (`aria-current="page"`); `#test-link-prefix` and
///   `#test-link-exact`: `/atoms` with prefix and exact matching.
/// - `#test-link-toolbar`: client-side navigation to `/atoms/toolbar`; `#test-link-replace`: to
///   `?replaced` without a history entry.
/// - `#test-link-external`: a new tab; `.test-link-disableable`: disabled by
///   `#test-link-toggle-disabled`, counting presses in `#test-link-presses`.
/// - `#test-link-menu`: a link as a menu trigger.
/// - `#test-link-anchor`: an `AnchorLink` to `#test-link-anchor-target` far below, scrolling
///   instantly, counting presses in `#test-link-anchor-presses`.
/// - `#test-link-hook-disabled`: a disabled `use_link` anchor.
#[component]
pub fn PageAtomLink() -> impl IntoView {
    let disabled = RwSignal::new(false);
    let presses = RwSignal::new(0u32);
    let anchor_presses = RwSignal::new(0u32);
    let actions = use_collection(|b| {
        b.item("one", "One");
    });
    let (hook_disabled_attrs, hook_disabled_styles) = use_link(UseLinkInput {
        href: Signal::stored(Some("/atoms/toolbar".to_owned())),
        is_disabled: Signal::stored(true),
        ..UseLinkInput::default()
    })
    .props
    .into_parts();

    view! {
        <div id="test-page-atom-link">
            <Link href="/atoms/link" attr:id="test-link-self">"Self"</Link>
            <Link href="/atoms" attr:id="test-link-prefix">"Prefix"</Link>
            <Link href="/atoms" current_match=CurrentMatch::Exact attr:id="test-link-exact">"Exact"</Link>
            <Link href="/atoms/toolbar" attr:id="test-link-toolbar">"Toolbar"</Link>
            <Link href="/atoms/link?replaced" replace=true attr:id="test-link-replace">"Replace"</Link>
            <Link href="https://example.com" target=LinkTarget::Blank attr:id="test-link-external">"External"</Link>
            <Link
                href="#disableable"
                is_disabled=disabled
                on_press=move |_| presses.update(|n| *n += 1)
                classes="test-link-disableable"
            >
                "Disableable"
            </Link>
            <button id="test-link-toggle-disabled" on:click=move |_| disabled.update(|d| *d = !*d)>
                "Toggle disabled"
            </button>
            <div>"Presses: " <span id="test-link-presses">{presses}</span></div>
            <MenuTrigger>
                <Link href="#menu" attr:id="test-link-menu">"Menu"</Link>
                <Popover>
                    <Menu collection=actions aria_label="Actions">
                        <MenuItems let:node>{node.text_value.to_string()}</MenuItems>
                    </Menu>
                </Popover>
            </MenuTrigger>
            <Button>"Unused"</Button>
            <AnchorLink
                href="#test-link-anchor-target"
                scroll_behavior=ScrollBehavior::Instant
                on_press=move |_| anchor_presses.update(|n| *n += 1)
                attr:id="test-link-anchor"
            >
                "To the target"
            </AnchorLink>
            <div>"Anchor presses: " <span id="test-link-anchor-presses">{anchor_presses}</span></div>
            <a {..hook_disabled_attrs} style=hook_disabled_styles id="test-link-hook-disabled">
                "Disabled hook link"
            </a>
            <div style="height: 3000px"></div>
            <div id="test-link-anchor-target">"Target"</div>
        </div>
    }
}
