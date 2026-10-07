use indoc::indoc;
use leptos::prelude::*;

use super::demos::scroll::ScrollDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageScroll() -> impl IntoView {
    view! {
        <DocPage title="scroll">
            <p>
                "The functions of "<Code inline=true>"leptonic::utils::scroll"</Code>" find the elements that scroll and "
                "bring an element into view inside them. Collections use them to keep the focused item visible, "
                <Link href=routes::doc::grid::Hook.materialize()>"grid cells"</Link>" to scroll themselves into the "
                "viewport; use them when you move focus or selection in a scrolling container yourself. Unlike the "
                "browser\u{2019}s "<Code inline=true>"scrollIntoView"</Code>", "<Code inline=true>"scroll_into_view"</Code>
                " scrolls only the container you give it, never the page around it."
            </p>

            <ReactAriaSource path="utils/scrollIntoView.ts"/>

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r"
                        use leptonic::utils::scroll::{ScrollAlignment, ScrollIntoViewOpts, scroll_into_view};

                        // `list` and `item` are `web_sys::HtmlElement`s: keep the item visible in the list.
                        scroll_into_view(&list, &item, ScrollIntoViewOpts {
                            block: ScrollAlignment::Nearest,
                            inline: ScrollAlignment::Nearest,
                        });
                    ")}
                </Code>
            </Section>

            <Section title="Demo">
                <p>"The buttons scroll the list to an item; the page itself doesn\u{2019}t move:"</p>
                <Demo description="A scrollable list of 30 items with buttons scrolling the first, middle and last item into the center of the list" source=include_str!("demos/scroll.rs")>
                    <ScrollDemo/>
                </Demo>
            </Section>

            <Section title="scroll_into_view">
                <p>
                    <Code inline=true>"scroll_into_view(scroll_view: &HtmlElement, element: &HtmlElement, opts: ScrollIntoViewOpts)"</Code>
                    " scrolls "<Code inline=true>"scroll_view"</Code>" so that "<Code inline=true>"element"</Code>", one of its "
                    "descendants, is visible. It respects scroll margins and padding, borders, scrollbars and "
                    "right-to-left layouts."
                </p>
                <ApiTable kind=ApiKind::Fields of="ScrollIntoViewOpts">
                    <ApiRow name="block" ty="ScrollAlignment" default="Nearest">"Where to align the element vertically."</ApiRow>
                    <ApiRow name="inline" ty="ScrollAlignment" default="Nearest">"Where to align it horizontally."</ApiRow>
                </ApiTable>
                <p>
                    <Code inline=true>"ScrollAlignment"</Code>" is "<Code inline=true>"Start"</Code>", "
                    <Code inline=true>"Center"</Code>", "<Code inline=true>"End"</Code>" or "<Code inline=true>"Nearest"</Code>
                    " (the least scrolling that makes the element visible; nothing if it already is)."
                </p>
            </Section>

            <Section title="scroll_into_viewport">
                <p>
                    <Code inline=true>"scroll_into_viewport(target: Option<&Element>, opts: &ScrollIntoViewportOpts)"</Code>
                    " brings "<Code inline=true>"target"</Code>" into the browser window. Normally it uses the browser\u{2019}s "
                    "own scrolling; while page scrolling is prevented (an open "<Link href=routes::doc::Modal.materialize()>"modal"</Link>
                    "), it scrolls only the target\u{2019}s scrolling ancestors below the page."
                </p>
                <ApiTable kind=ApiKind::Fields of="ScrollIntoViewportOpts">
                    <ApiRow name="containing_element" ty="Option<Element>" default="None">
                        "An element around the target to center in the window first, e.g. a grid when focusing one of its cells."
                    </ApiRow>
                </ApiTable>
            </Section>

            <Section title="is_scrollable">
                <p>
                    <Code inline=true>"is_scrollable(node: &Element, check_for_overflow: bool) -> bool"</Code>" is true if the "
                    "element\u{2019}s "<Code inline=true>"overflow"</Code>" lets it scroll ("<Code inline=true>"auto"</Code>" or "
                    <Code inline=true>"scroll"</Code>"; the document\u{2019}s root element unless "<Code inline=true>"hidden"</Code>
                    "). With "<Code inline=true>"check_for_overflow"</Code>", its content must also be larger than the element."
                </p>
            </Section>

            <Section title="get_scroll_parent">
                <p>
                    <Code inline=true>"get_scroll_parent(node: &Element, check_for_overflow: bool) -> Element"</Code>
                    " returns the nearest scrollable ancestor of the element, or the document\u{2019}s scrolling element. "
                    <Code inline=true>"get_scroll_parents(..)"</Code>" returns all of them up to the root, the element itself "
                    "included when it scrolls."
                </p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::overlay_behavior::UsePreventScroll.materialize()>"use_prevent_scroll"</Link></li>
                <li><Link href=routes::doc::CollectionState.materialize()>"Collection State"</Link></li>
                <li><Link href=routes::doc::focus::Focusability.materialize()>"focusability"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
