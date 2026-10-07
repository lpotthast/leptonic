use indoc::indoc;
use leptos::prelude::*;

use super::demos::visually_hidden::VisuallyHiddenAtomDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageAtomVisuallyHidden() -> impl IntoView {
    view! {
        <DocPage title="VisuallyHidden">
            <p>
                "The "<Code inline=true>"VisuallyHidden"</Code>" atom renders content that screen readers read but sighted "
                "users don\u{2019}t see: the context of a repeated \u{201c}Download\u{201d} button, the name of an "
                "icon-only control, a skip link that appears when it receives focus. Together with the "
                <Link href=routes::doc::screen_readers::LiveAnnouncer.materialize()>"live_announcer"</Link>
                ", it is one of the building blocks for screen readers."
            </p>

            <ReactAria hook="VisuallyHidden"/>

            <Section title="Hooks Used">
                <p>
                    <Link href=routes::doc::screen_readers::UseVisuallyHidden.materialize()><Code inline=true>"use_visually_hidden"</Code></Link>
                    ". The atom renders one element with the hook\u{2019}s "
                    "hiding styles and focus tracking."
                </p>
            </Section>

            <Section title="Props">
                <ApiTable kind=ApiKind::Props of="VisuallyHidden">
                    <ApiRow name="element" ty="TextElement" default="Div">
                        "The element to render: "<Code inline=true>"TextElement::Div"</Code>" or "
                        <Code inline=true>"TextElement::Span"</Code>". Use "<Code inline=true>"Span"</Code>
                        " inside inline content such as buttons, links and paragraphs."
                    </ApiRow>
                    <ApiRow name="is_focusable" ty="Signal<bool>" default="false">
                        "Show the content while focus is within it, e.g. for a skip link."
                    </ApiRow>
                    <ApiRow name="classes" ty="Classes" default="empty">"Additional classes of the element."</ApiRow>
                    <ApiRow name="children" ty="Children">"The hidden content."</ApiRow>
                </ApiTable>
            </Section>

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::atoms::{field::TextElement, visually_hidden::VisuallyHidden};

                        view! {
                            <a href="/blog/ssr-in-leptos">
                                "Read more"
                                <VisuallyHidden element=TextElement::Span>" about SSR in Leptos"</VisuallyHidden>
                            </a>
                        }
                    "#)}
                </Code>
                <p>
                    "Screen readers announce the link as \u{201c}Read more about SSR in Leptos\u{201d}, which still makes sense "
                    "in a list of all links on the page."
                </p>
            </Section>

            <Section title="Demo">
                <p>
                    "Each button shows \u{201c}Download\u{201d}; its accessible name is \u{201c}Download invoice 1042\u{201d} "
                    "and so on. Tab into the demo: the skip link appears while it has focus."
                </p>

                <Demo description="Download buttons with visually hidden context and a skip link shown on focus" source=include_str!("demos/visually_hidden.rs")>
                    <VisuallyHiddenAtomDemo/>
                </Demo>
            </Section>

            <Section title="Styling">
                <p>
                    "The atom sets the element\u{2019}s "<Code inline=true>"style"</Code>" attribute to hide it, and removes it "
                    "while a focusable element shows. Style the shown state through "<Code inline=true>"classes"</Code>
                    ", e.g. to place a skip link on top of the page:"
                </p>
                <Code language=Language::Rust>
                    {indoc!(r##"
                        view! {
                            <VisuallyHidden is_focusable=true classes="skip-link">
                                <a href="#main">"Skip to content"</a>
                            </VisuallyHidden>
                        }
                    "##)}
                </Code>
                <Code language=Language::Css>
                    {indoc!(r"
                        .skip-link {
                            position: fixed;
                            top: 0.5em;
                            left: 0.5em;
                            z-index: 100;
                        }
                    ")}
                </Code>
                <p>
                    "While hidden, the inline styles win over these rules; once the element has focus, they apply."
                </p>
            </Section>

            <Section title="Composition">
                <p>
                    "Leptonic uses the atom itself: "
                    <Link href=routes::doc::overlay_behavior::DismissButton.materialize()><Code inline=true>"DismissButton"</Code></Link>
                    " renders a visually hidden button that lets screen reader users close an overlay, and the "
                    <Link href=routes::doc::slider::Atom.materialize()>"Slider atoms"</Link>" hide the native range "
                    <Code inline=true>"<input>"</Code>" of each thumb in it, which screen readers and the keyboard operate."
                </p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::screen_readers::UseVisuallyHidden.materialize()>"use_visually_hidden"</Link></li>
                <li><Link href=routes::doc::screen_readers::LiveAnnouncer.materialize()>"live_announcer"</Link></li>
                <li><Link href=routes::doc::overlay_behavior::DismissButton.materialize()>"DismissButton"</Link></li>
                <li><Link href=routes::doc::focus::FocusRing.materialize()>"FocusRing"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
