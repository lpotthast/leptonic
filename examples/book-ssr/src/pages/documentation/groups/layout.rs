use indoc::indoc;
use leptos::prelude::*;

use super::demos::{layout_app_bar::LayoutAppBarDemo, layout_skeleton::LayoutSkeletonDemo};
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageLayout() -> impl IntoView {
    view! {
        <DocPage title="Content & Layout">
            <p>
                "Content & Layout covers what structures a page and presents its content: toolbars and separators "
                "bundling and dividing controls, key caps in text, and the parts every app has without any behavior of "
                "their own \u{2014} an app bar, stacks and grids, cards, text styles, icons and placeholders for content "
                "that is still loading."
            </p>
            <p>
                "They belong together because they arrange or display content instead of collecting input or navigating. "
                "leptonic provides the ones with ARIA semantics or behavior: the toolbar, the separator and the key caps. "
                "The others are plain markup and CSS; the "<AnchorLink href="#recipes">"recipes"</AnchorLink>
                " below show how to build them accessibly."
            </p>

            <Section title="Pages">
                <SectionMembers overview=routes::doc::Layout.materialize()/>
            </Section>

            <Section title="Relationships">
                <ul>
                    <li>
                        <Link href=routes::doc::Toolbar.materialize()>"Toolbar"</Link>" groups controls into one tab stop "
                        "whose controls the arrow keys move between. A vertical "
                        <Link href=routes::doc::Separator.materialize()>"Separator"</Link>" divides groups of controls "
                        "inside it. Groups of "<Link href=routes::doc::ToggleButton.materialize()>"toggle buttons"</Link>
                        " are toolbars too."
                    </li>
                    <li>
                        <Link href=routes::doc::Kbd.materialize()>"Kbd"</Link>" adds key caps and keyboard shortcuts to "
                        "text, in the form of the user\u{2019}s platform."
                    </li>
                    <li>
                        "An "<AnchorLink href="#app-bar">"app bar"</AnchorLink>" holds the app name, navigation and global "
                        "actions, often as icon buttons. "<AnchorLink href="#stack">"Stacks"</AnchorLink>" and "
                        <AnchorLink href="#grid-layout">"grids"</AnchorLink>" arrange elements, "
                        <AnchorLink href="#card">"cards"</AnchorLink>" put related content on a surface of its own, and a "
                        <AnchorLink href="#skeleton">"skeleton"</AnchorLink>" holds the place of any of them while its "
                        "content loads."
                    </li>
                    <li>
                        "A set of tags that users navigate, select and remove is a "
                        <Link href=routes::doc::TagGroup.materialize()>"Tag Group"</Link>"; a static label is a styled "
                        <Code inline=true>"<span>"</Code>"."
                    </li>
                </ul>
            </Section>

            <Section title="Decision Guide">
                <DocTable headers=&["If you want to\u{2026}", "Use"]>
                    <TableRow>
                        <TableCell>"Group related controls, such as the formatting buttons of an editor"</TableCell>
                        <TableCell><Link href=routes::doc::Toolbar.materialize()>"Toolbar"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Mark a boundary between sections or groups of controls"</TableCell>
                        <TableCell><Link href=routes::doc::Separator.materialize()>"Separator"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Show a key or a keyboard shortcut"</TableCell>
                        <TableCell><Link href=routes::doc::Kbd.materialize()>"Kbd"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Put the app name, navigation and global actions at the top"</TableCell>
                        <TableCell>"An "<AnchorLink href="#app-bar">"app bar"</AnchorLink>": a "<Code inline=true>"<header>"</Code></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Arrange elements in a row or a column, or in columns that adapt to the screen width"</TableCell>
                        <TableCell>
                            "CSS flexbox ("<AnchorLink href="#stack">"stack"</AnchorLink>") or grid ("
                            <AnchorLink href="#grid-layout">"grid layout"</AnchorLink>")"
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Navigate rows and cells of data with the keyboard"</TableCell>
                        <TableCell>
                            <Link href=routes::doc::Grid.materialize()>"Grid"</Link>" or "
                            <Link href=routes::doc::Table.materialize()>"Table"</Link>
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Show related content, such as a summary with its actions, on a surface of its own"</TableCell>
                        <TableCell>"A "<AnchorLink href="#card">"card"</AnchorLink></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Hold the place of content that is still loading"</TableCell>
                        <TableCell>"A "<AnchorLink href="#skeleton">"skeleton"</AnchorLink></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Write headings, paragraphs, lists and code"</TableCell>
                        <TableCell>"HTML elements with your "<AnchorLink href="#typography">"text styles"</AnchorLink></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Show an icon"</TableCell>
                        <TableCell><AnchorLink href="#icons">"leptos_icons"</AnchorLink></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Render HTML you didn\u{2019}t write, e.g. from users or a CMS, or edit rich text"</TableCell>
                        <TableCell><AnchorLink href="#rich-content">"ammonia, leptos-tiptap"</AnchorLink></TableCell>
                    </TableRow>
                </DocTable>

                <p>
                    "A separator is announced to screen readers as a boundary. For spacing without that meaning, use CSS "
                    "margins or the gap of a stack."
                </p>
            </Section>

            <Section title="Recipes">
                <p>
                    "These parts have no behavior, so leptonic has no piece for them: an app builds them from HTML elements "
                    "and its own CSS. The snippets use stand-ins for your design tokens ("<Code inline=true>"var(--surface)"</Code>
                    ", "<Code inline=true>"var(--border)"</Code>", \u{2026})."
                </p>

                <Section title="App Bar">
                    <p>
                        "An app bar is the bar at the top of an app, holding its name, the main navigation and global "
                        "actions. Render it as a "<Code inline=true>"<header>"</Code>" next to your "
                        <Code inline=true>"<main>"</Code>", not inside it, and make it the page\u{2019}s "
                        <Code inline=true>"banner"</Code>" landmark with "
                        <Link href=routes::doc::focus::UseLandmark.materialize()>"use_landmark"</Link>": screen reader users "
                        "jump to it directly, and "<Keys keys="F6"/>" and "<Keys keys="Shift + F6"/>" reach it among the "
                        "page\u{2019}s landmarks. Wrap its links in a "<Code inline=true>"<nav>"</Code>", and name icon-only "
                        "buttons with "<Code inline=true>"aria_label"</Code>". "<Code inline=true>"position: sticky"</Code>
                        " keeps it at the top while the page scrolls."
                    </p>
                    <Demo
                        description="App bar with a title and two icon buttons, sticking to the top of a scrolling frame"
                        source=include_str!("demos/layout_app_bar.rs")
                    >
                        <LayoutAppBarDemo/>
                    </Demo>
                </Section>

                <Section title="Stack">
                    <p>
                        "A stack arranges elements in a column or a row with even spacing: flexbox with a "
                        <Code inline=true>"gap"</Code>". Spacing between elements belongs to the stack, not to the "
                        "elements, so that they can be moved around freely."
                    </p>
                    <Code language=Language::Css>
                        {indoc!(r"
                            .stack { display: flex; flex-direction: column; gap: 1em; }
                            .stack-row { display: flex; flex-wrap: wrap; align-items: center; gap: 0.5em; }
                        ")}
                    </Code>
                </Section>

                <Section title="Grid Layout">
                    <p>
                        "Columns that adapt to the screen width need no media queries: CSS grid fits as many columns of a "
                        "minimum width as there is room for. The visual order must match the order in the markup, which "
                        "screen readers and the "<Keys keys="Tab"/>" key follow."
                    </p>
                    <Code language=Language::Css>
                        {indoc!(r"
                            .grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(12em, 1fr)); gap: 1em; }
                            .grid-wide { grid-column: 1 / -1; }
                        ")}
                    </Code>
                </Section>

                <Section title="Card">
                    <p>
                        "A card puts related content on a surface of its own. Give it a heading, so that it is found by "
                        "heading navigation, and use "<Code inline=true>"<article>"</Code>" for content that stands on its "
                        "own, such as a post in a feed. A card that is a link as a whole puts the link on its heading and "
                        "stretches the link\u{2019}s area over the card, instead of wrapping everything in an "
                        <Code inline=true>"<a>"</Code>":"
                    </p>
                    <Code language=Language::Css>
                        {indoc!(r#"
                            .card { position: relative; padding: 1.5em; border: 1px solid var(--border); border-radius: 8px; background: var(--surface); }
                            .card-title { margin: 0 0 0.5em; font-size: 1.25em; }
                            /* The heading's link covers the card. */
                            .card-link::after { content: ""; position: absolute; inset: 0; }
                            .card:has(.card-link[data-focus-visible]) { outline: 2px solid var(--focus); outline-offset: 2px; }
                        "#)}
                    </Code>
                </Section>

                <Section title="Skeleton">
                    <p>
                        "A skeleton holds the place of content that is still loading, shaped like it, so that the page "
                        "doesn\u{2019}t jump when the content arrives. The placeholders have no role, so screen readers pass "
                        "over them: set "<Code inline=true>"aria-busy=\"true\""</Code>" on the region that is loading, and "
                        "announce the result with the "
                        <Link href=routes::doc::screen_readers::LiveAnnouncer.materialize()>"live announcer"</Link>
                        " where it matters (\u{201c}12 results\u{201d}). Stop the shimmer when the user prefers reduced motion."
                    </p>
                    <Demo description="A profile loading: placeholders for the name and the text" source=include_str!("demos/layout_skeleton.rs")>
                        <LayoutSkeletonDemo/>
                    </Demo>
                </Section>

                <Section title="Typography">
                    <p>
                        "Write text with the HTML elements that carry its meaning ("<Code inline=true>"<h2>"</Code>", "
                        <Code inline=true>"<p>"</Code>", "<Code inline=true>"<ul>"</Code>", "<Code inline=true>"<code>"</Code>
                        ", "<Code inline=true>"<kbd>"</Code>") and give them your text styles in CSS. Keep the heading levels "
                        "in order, as screen reader users navigate by them; size a heading with CSS rather than by picking "
                        "another level. Key caps and shortcuts come from the "
                        <Link href=routes::doc::Kbd.materialize()>"Kbd Atoms"</Link>"."
                    </p>
                    <Code language=Language::Css>
                        {indoc!(r"
                            h1 { font-size: 2em; font-weight: 700; }
                            h2 { font-size: 1.5em; font-weight: 700; }
                            p { line-height: 1.5; }
                            code { padding: 0.1em 0.3em; border-radius: 4px; background: var(--surface); font-family: monospace; }
                        ")}
                    </Code>
                </Section>

                <Section title="Icons">
                    <p>
                        "Render icons with the "<Link href="https://crates.io/crates/leptos_icons" target=LinkTarget::Blank>"leptos_icons"</Link>
                        " crate and an icon set from "<Link href="https://crates.io/crates/icondata" target=LinkTarget::Blank>"icondata"</Link>
                        ". An icon next to text, or inside a button named with "<Code inline=true>"aria_label"</Code>
                        ", is decorative: hide it from assistive technology. An icon that stands alone and means something "
                        "is an image with a name:"
                    </p>
                    <Code language=Language::Rust>
                        {indoc!(r#"
                            use leptos::prelude::*;
                            use leptos_icons::Icon;

                            view! {
                                // Decorative, next to its text.
                                <span aria-hidden="true"><Icon icon=icondata::BsCheckCircle/></span>" Saved"
                                // Meaningful on its own.
                                <span role="img" aria-label="Verified"><Icon icon=icondata::BsPatchCheck/></span>
                            }
                        "#)}
                    </Code>
                </Section>

                <Section title="Rich Content">
                    <p>
                        "HTML from other sources, such as user input, a CMS or a Markdown renderer, may contain scripts. "
                        "Clean it with the "<Link href="https://crates.io/crates/ammonia" target=LinkTarget::Blank>"ammonia"</Link>
                        " crate before you render it with "<Code inline=true>"inner_html"</Code>", on the server or in the "
                        "browser:"
                    </p>
                    <Code language=Language::Rust>
                        {indoc!(r#"
                            use leptos::prelude::*;

                            let html = "<p>Hello <b>world</b><script>alert(1)</script></p>";
                            view! { <div inner_html=ammonia::clean(html)></div> }
                        "#)}
                    </Code>
                    <p>
                        "For a rich text editor, use the "
                        <Link href="https://crates.io/crates/leptos-tiptap" target=LinkTarget::Blank>"leptos-tiptap"</Link>
                        " crate, which brings the Tiptap editor to Leptos. Give the editor a label, as any field."
                    </p>
                </Section>
            </Section>
        </DocPage>
    }
}
