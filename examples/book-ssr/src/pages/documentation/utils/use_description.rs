use indoc::indoc;
use leptos::prelude::*;

use super::demos::use_description::UseDescriptionDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageUseDescription() -> impl IntoView {
    view! {
        <DocPage title="use_description">
            <p>
                "Some elements need a description that screen readers read after their name, but that nobody needs to "
                "see: how to operate a long press, what a drag handle does. The "<Code inline=true>"use_description"</Code>
                " hook (in "<Code inline=true>"leptonic::use_description"</Code>") puts such a text into a hidden "
                "element and returns its id, for the "<Code inline=true>"aria-describedby"</Code>" attribute of your element. "
                "It is one of the building blocks for screen readers, next to "
                <Link href=routes::doc::screen_readers::VisuallyHidden.materialize()>"VisuallyHidden"</Link>" and "
                <Link href=routes::doc::screen_readers::LiveAnnouncer.materialize()>"live_announcer"</Link>"."
            </p>

            <ReactAriaSource path="utils/useDescription.ts"/>

            <Section title="Input">
                <p>
                    "The hook takes the description as a "<Code inline=true>"Signal<Option<String>>"</Code>". "
                    <Code inline=true>"None"</Code>" and an empty text create no element."
                </p>
            </Section>

            <Section title="Return">
                <p>
                    "A "<Code inline=true>"Signal<Option<String>>"</Code>": the id of the hidden element, or "
                    <Code inline=true>"None"</Code>" without a description. Pass it to "<Code inline=true>"aria-describedby"</Code>
                    "; an attribute with the value "<Code inline=true>"None"</Code>" isn\u{2019}t rendered."
                </p>
            </Section>

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::use_description;

                        let description = use_description(Signal::stored(Some(String::from("Opens the archive."))));

                        view! {
                            <button aria-describedby=move || description.get()>"Archive"</button>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Demo">
                <p>
                    "With a screen reader, focus the button: it reads \u{201c}Delete draft\u{201d}, then the description. "
                    "Clear the checkbox to remove it; the status shows the attribute the button gets."
                </p>
                <Demo description="A button described through use_description, with the description toggled by a checkbox" source=include_str!("demos/use_description.rs")>
                    <UseDescriptionDemo/>
                </Demo>
            </Section>

            <Section title="Shared Elements">
                <p>
                    "The hidden elements are "<Code inline=true>"<div>"</Code>"s with "<Code inline=true>"display: none"</Code>
                    " at the end of "<Code inline=true>"<body>"</Code>"; "<Code inline=true>"aria-describedby"</Code>
                    " reads hidden elements too. Identical texts share one element: a list of a hundred draggable items "
                    "with the same instructions adds one element, not a hundred. The element is removed when its last user "
                    "is disposed or changes its text."
                </p>
                <p>
                    "Leptonic\u{2019}s hooks use it for their own descriptions: "
                    <Link href=routes::doc::interactions::UsePress.materialize()>"use_press"</Link>" for "
                    <Code inline=true>"LongPress::accessibility_description"</Code>", and the "
                    <Link href=routes::doc::DragAndDrop.materialize()>"Drag & Drop"</Link>" hooks for their drag instructions."
                </p>
            </Section>

            <Section title="Server-Side Rendering">
                <p>
                    "The server renders no description: the id is "<Code inline=true>"None"</Code>" until the hook runs in "
                    "the browser, so hydration matches and the attribute appears once the page is interactive."
                </p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::screen_readers::UseVisuallyHidden.materialize()>"use_visually_hidden"</Link></li>
                <li><Link href=routes::doc::screen_readers::VisuallyHidden.materialize()>"VisuallyHidden"</Link></li>
                <li><Link href=routes::doc::screen_readers::LiveAnnouncer.materialize()>"live_announcer"</Link></li>
                <li><Link href=routes::doc::interactions::UsePress.materialize()>"use_press"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
