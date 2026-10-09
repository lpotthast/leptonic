use indoc::indoc;
use leptos::prelude::*;

use super::demos::{
    anchor_link::AnchorLinkAtomDemo, link::LinkAtomDemo, link_button::LinkStyledAsButtonDemo,
    link_external::LinkExternalAtomDemo,
};
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageAtomLink() -> impl IntoView {
    view! {
        <DocPage title="Link Atoms">
            <p>
                "The unstyled "<AnchorLink href="#link">"Link"</AnchorLink>" takes users to another page of your app or "
                "another site, also "<AnchorLink href="#links-that-look-like-buttons">"styled as a button"</AnchorLink>
                ", and "<AnchorLink href="#anchorlink">"AnchorLink"</AnchorLink>" scrolls to an element on the current "
                "page. See the "<Link href=routes::doc::Link.materialize()>"Link overview"</Link>" for concept guidance."
            </p>

            <Section title="Hooks Used">
                <DocTable headers=&["Atom", "Hook"]>
                    <TableRow>
                        <TableCell><Code inline=true>"Link"</Code></TableCell>
                        <TableCell>
                            <Link href=format!("{}#use-link", routes::doc::link::Hook.materialize())>"use_link"</Link>
                            ", which composes "<Link href=routes::doc::interactions::UsePress.materialize()>"use_press"</Link>", "
                            <Link href=routes::doc::interactions::UseHover.materialize()>"use_hover"</Link>", "
                            <Link href=routes::doc::focus::UseFocusable.materialize()>"use_focusable"</Link>" and "
                            <Link href=routes::doc::focus::UseFocusRing.materialize()>"use_focus_ring"</Link>
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"AnchorLink"</Code></TableCell>
                        <TableCell>
                            <Link href=format!("{}#use-anchor-link", routes::doc::link::Hook.materialize())>"use_anchor_link"</Link>
                        </TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::{atoms::link::Link, hooks::link::LinkTarget};

                        view! {
                            <Link href="/settings" classes="my-link">"Settings"</Link>
                            <Link href="https://leptos.dev" target=LinkTarget::Blank classes="my-link">"Leptos"</Link>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Demo">
                <p>
                    "The pages of this concept as a navigation. The link to this page has "
                    <Code inline=true>"aria-current=\"page\""</Code>"; the overview link uses "
                    <Code inline=true>"CurrentMatch::Exact"</Code>", as every other page lies below its route. The links are "
                    "styled through the "<AnchorLink href="#data-attributes">"data attributes"</AnchorLink>" only."
                </p>
                <Demo description="A navigation of four links, the current one marked, with a Disabled checkbox" source=include_str!("demos/link.rs")>
                    <LinkAtomDemo/>
                </Demo>
            </Section>

            <Section title="Link">
                <p>
                    "Renders leptos_router\u{2019}s "<Code inline=true>"<A>"</Code>", so it needs a surrounding "
                    <Code inline=true>"<Router>"</Code>": links within your app navigate on the client, a relative "
                    <Code inline=true>"href"</Code>" is resolved against the current route, and the link has "
                    <Code inline=true>"aria-current=\"page\""</Code>" while its route is the current one. Links to other "
                    "sites are left to the browser. A disabled link renders as a "<Code inline=true>"<span role=\"link\">"</Code>
                    " without "<Code inline=true>"href"</Code>": it can\u{2019}t be followed in any way."
                </p>

                <Section title="Props" id="link-props">
                    <ApiTable kind=ApiKind::Props of="atoms::link::Link">
                        <ApiRow name="href" ty="impl ToHref">"A route of your app or a URL. Required."</ApiRow>
                        <ApiRow name="target" ty="Signal<LinkTarget>" default="Same">
                            "Where to open the link, e.g. "<Code inline=true>"LinkTarget::Blank"</Code>" for a new tab."
                        </ApiRow>
                        <ApiRow name="rel" ty="Vec<LinkRel>" default="empty">
                            "Values of the "<Code inline=true>"rel"</Code>" attribute. "<Code inline=true>"LinkRel::NoOpener"</Code>
                            " is added for "<Code inline=true>"LinkTarget::Blank"</Code>"."
                        </ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>" default="false">"Whether the link can\u{2019}t be followed."</ApiRow>
                        <ApiRow name="current_match" ty="CurrentMatch" default="Prefix">
                            "When the link is the current page: "<Code inline=true>"Prefix"</Code>" while the location is its route or "
                            "below it, "<Code inline=true>"Exact"</Code>" only on its route."
                        </ApiRow>
                        <ApiRow name="replace" ty="bool" default="false">
                            "Replace the current history entry instead of adding one (client-side navigation only)."
                        </ApiRow>
                        <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">"Names the link when its content doesn\u{2019}t."</ApiRow>
                        <ApiRow name="on_press" ty="Option<Callback<PressEvent>>" default="None">"Called when the link is pressed."</ApiRow>
                        <ApiRow name="on_hover_start, on_hover_end" ty="Option<Callback<HoverStartEvent>>, Option<Callback<HoverEndEvent>>" default="None">
                            "Called when a pointer starts or stops hovering the link."
                        </ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the link element."</ApiRow>
                        <ApiRow name="children" ty="ChildrenFn">
                            "The link content, rendered again when the link is disabled or enabled. Required."
                        </ApiRow>
                        <ApiRow name="auto_focus" ty="bool" default="false">"Focus this element when it mounts."</ApiRow>
                        <ApiRow name="on_blur" ty="Option<Callback<FocusEvent>>" default="None">"Called when focus leaves the element."</ApiRow>
                        <ApiRow name="on_focus" ty="Option<Callback<FocusEvent>>" default="None">"Called when focus enters the element."</ApiRow>
                        <ApiRow name="on_focus_change" ty="Option<Callback<bool>>" default="None">"Called when the focused state changes."</ApiRow>
                        <ApiRow name="on_hover_change" ty="Option<Callback<bool>>" default="None">"Called when the hovered state changes."</ApiRow>
                        <ApiRow name="on_key_down" ty="Option<Callback<KeyboardEventWrapper>>" default="None">"Called when a key is pressed."</ApiRow>
                        <ApiRow name="on_key_up" ty="Option<Callback<KeyboardEventWrapper>>" default="None">"Called when a key is released."</ApiRow>
                        <ApiRow name="on_press_change" ty="Option<Callback<bool>>" default="None">"Called when the pressed state changes."</ApiRow>
                        <ApiRow name="on_press_end" ty="Option<Callback<PressEvent>>" default="None">"Called when a press ends."</ApiRow>
                        <ApiRow name="on_press_start" ty="Option<Callback<PressEvent>>" default="None">"Called when a press starts."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Links to Other Sites">
                    <p>
                        "Give a URL as "<Code inline=true>"href"</Code>", usually with "<Code inline=true>"LinkTarget::Blank"</Code>
                        ". "<Code inline=true>"rel"</Code>" describes the relationship to the linked page, e.g. "
                        <Code inline=true>"LinkRel::NoFollow"</Code>" for links search engines should not follow."
                    </p>
                    <Demo description="A link to another site, opening in a new tab" source=include_str!("demos/link_external.rs")>
                        <LinkExternalAtomDemo/>
                    </Demo>
                </Section>
            </Section>

            <Section title="Links That Look Like Buttons">
                <p>
                    "A link that navigates but looks like a button, e.g. \u{201c}Get started\u{201d}, is a "
                    <Code inline=true>"Link"</Code>" with the styles of your buttons: it stays a link for assistive "
                    "technology and keeps "<Code inline=true>"aria-current"</Code>" and the link\u{2019}s data attributes. "
                    "HTML doesn\u{2019}t allow a button inside a link. The demo reuses the book\u{2019}s button styles."
                </p>
                <Demo description="A link styled as a button, with a Disabled checkbox" source=include_str!("demos/link_button.rs")>
                    <LinkStyledAsButtonDemo/>
                </Demo>
            </Section>

            <Section title="AnchorLink">
                <p>
                    "Links to an element on the current page by its id. Pressing it scrolls the element into view and puts "
                    "the id into the URL without adding a history entry. Anchor links are most common next to headings, "
                    "where a bare "<Code inline=true>"#"</Code>" needs an "<Code inline=true>"aria_label"</Code>
                    " saying where it leads."
                </p>

                <Demo description="A text link and a # link next to a heading, both pointing to the heading" source=include_str!("demos/anchor_link.rs")>
                    <AnchorLinkAtomDemo/>
                </Demo>

                <Section title="Props" id="anchor-link-props">
                    <ApiTable kind=ApiKind::Props of="atoms::link::AnchorLink">
                        <ApiRow name="href" ty="Href">
                            "The id of the element to link to: "<Code inline=true>"\"#returns\""</Code>" or "
                            <Code inline=true>"\"returns\""</Code>". Converts from "<Code inline=true>"&'static str"</Code>", "
                            <Code inline=true>"String"</Code>" and "<Code inline=true>"Oco"</Code>". Required."
                        </ApiRow>
                        <ApiRow name="scroll_behavior" ty="Option<ScrollBehavior>" default="Some(Smooth)">
                            "How to scroll to the element; smooth scrolling is instant while reduced motion is preferred. "
                            <Code inline=true>"None"</Code>" doesn\u{2019}t scroll; only the URL fragment changes."
                        </ApiRow>
                        <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">
                            "Names the link when its content doesn\u{2019}t, such as a bare "<Code inline=true>"#"</Code>"."
                        </ApiRow>
                        <ApiRow name="on_press" ty="Option<Callback<PressEvent>>" default="None">"Called after scrolling."</ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the "<Code inline=true>"<a>"</Code>"."</ApiRow>
                        <ApiRow name="children" ty="Children">"The link content. Required."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Data Attributes">
                <ApiTable kind=ApiKind::DataAttributes>
                    <ApiRow name="data-pressed" ty="true">"While the link is pressed."</ApiRow>
                    <ApiRow name="data-hovered" ty="true">"While a pointer hovers the link."</ApiRow>
                    <ApiRow name="data-focused" ty="true">"While the link has focus."</ApiRow>
                    <ApiRow name="data-focus-visible" ty="true">"While the link has keyboard focus."</ApiRow>
                    <ApiRow name="data-disabled" ty="true">
                        "While the link is disabled ("<Code inline=true>"Link"</Code>")."
                    </ApiRow>
                </ApiTable>
                <p>
                    "The current page\u{2019}s "<Code inline=true>"Link"</Code>" has "
                    <Code inline=true>"aria-current=\"page\""</Code>", set by the router."
                </p>
            </Section>

            <Section title="Styling">
                <p>
                    "The atoms bring no styles. "<Code inline=true>"Link"</Code>" renders the class "
                    <Code inline=true>"leptonic-Link"</Code>" and "<Code inline=true>"AnchorLink"</Code>" the class "
                    <Code inline=true>"leptonic-AnchorLink"</Code>", each followed by the "<Code inline=true>"classes"</Code>
                    " you pass. Target the state with the data attributes above, and the current page with "
                    <Code inline=true>"aria-current"</Code>". The book\u{2019}s demos use these rules:"
                </p>
                <Code language=Language::Css>
                    {indoc!(r#"
                        .my-link { color: var(--accent); text-decoration: underline; text-underline-offset: 0.2em; cursor: pointer; }
                        .my-link[data-hovered] { text-decoration-thickness: 2px; }
                        .my-link[aria-current="page"] { font-weight: 600; text-decoration: none; cursor: default; }
                        .my-link[data-disabled] { color: var(--muted); text-decoration: none; cursor: not-allowed; }
                        .my-link[data-focus-visible] { outline: 2px solid var(--focus); outline-offset: 2px; border-radius: 4px; }
                    "#)}
                </Code>
                <p>
                    "Leptonic also ships an optional atom theme that styles the default classes, for apps that don\u{2019}t "
                    "want to start from scratch: "<Code inline=true>"@use \"leptonic/leptonic-atoms\";"</Code>"."
                </p>
            </Section>

            <Section title="Composition">
                <p>
                    "A "<Link href=routes::doc::breadcrumbs::Atom.materialize()>"Breadcrumb"</Link>" hands the "
                    <Code inline=true>"Link"</Code>" inside it its disabled state, its "<Code inline=true>"aria-current"</Code>
                    " and the breadcrumbs\u{2019} "<Code inline=true>"on_action"</Code>", which add to the link\u{2019}s own props."
                </p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Link.materialize()>"Link overview"</Link></li>
                <li><Link href=routes::doc::link::Hook.materialize()>"Link Hooks"</Link></li>
                <li><Link href=routes::doc::breadcrumbs::Atom.materialize()>"Breadcrumbs Atoms"</Link></li>
                <li><Link href=routes::doc::button::Atom.materialize()>"Button Atom"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
