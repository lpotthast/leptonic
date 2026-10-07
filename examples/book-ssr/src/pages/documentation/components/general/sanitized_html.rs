use indoc::indoc;
use leptos::prelude::*;

use super::demos::sanitized_html::SanitizedHtmlDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageSanitizedHtml() -> impl IntoView {
    view! {
        <DocPage title="Sanitized HTML Component">
            <p>
                "Some text arrives as HTML: a comment written in a rich text editor, an article from a CMS, the output "
                "of a Markdown renderer. Rendering such HTML as it is lets whoever wrote it run code in your users\u{2019} "
                "browsers (cross-site scripting). The "<Code inline=true>"SanitizedHtml"</Code>" component cleans the HTML "
                "with "<Link href="https://docs.rs/ammonia" target=LinkTarget::Blank>"ammonia"</Link>" and renders the "
                "result inside a "<Code inline=true>"<div>"</Code>". It keeps text markup (paragraphs, emphasis, lists, "
                "links, tables, images) and removes everything that can run code: "<Code inline=true>"<script>"</Code>
                " and "<Code inline=true>"<style>"</Code>" elements, event handler attributes such as "
                <Code inline=true>"onclick"</Code>", and "<Code inline=true>"javascript:"</Code>" URLs."
            </p>
            <p>
                "It requires leptonic\u{2019}s "<Code inline=true>"sanitize"</Code>" feature (part of "
                <Code inline=true>"full"</Code>", see "<Link href=routes::doc::Installation.materialize()>"Installation"</Link>
                "). Sanitizing runs in Rust, so the server renders the same cleaned HTML as the browser."
            </p>

            <Demo
                description="Text field with HTML containing a script, an event handler and a javascript URL, rendered sanitized"
                source=include_str!("demos/sanitized_html.rs")
            >
                <SanitizedHtmlDemo/>
            </Demo>

            <Section title="Props">
                <ApiTable kind=ApiKind::Props of="SanitizedHtml">
                    <ApiRow name="html" ty="Signal<String>">
                        "The HTML to sanitize and render: a string or any signal of one. Required."
                    </ApiRow>
                    <ApiRow name="configure" ty="Option<Callback<ammonia::Builder<'static>, ammonia::Builder<'static>>>" default="None">
                        "Adapts the sanitizing rules: receives ammonia\u{2019}s default "<Code inline=true>"Builder"</Code>
                        " and returns the one to use. Called whenever "<Code inline=true>"html"</Code>" changes. See "
                        <AnchorLink href="#custom-rules">"Custom Rules"</AnchorLink>"."
                    </ApiRow>
                    <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">
                        "Classes and styles of the "<Code inline=true>"<div>"</Code>"."
                    </ApiRow>
                </ApiTable>
            </Section>

            <Section title="Custom Rules">
                <p>
                    "ammonia\u{2019}s defaults are strict: they drop "<Code inline=true>"class"</Code>" and "
                    <Code inline=true>"style"</Code>" attributes and every element they don\u{2019}t know. To allow more, "
                    "configure the builder. Naming its type requires "<Code inline=true>"ammonia"</Code>" as a dependency "
                    "of your app, in the version leptonic uses (4), as leptonic doesn\u{2019}t re-export it."
                </p>
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::components::prelude::*;

                        view! {
                            <SanitizedHtml
                                html=comment
                                // Also keep <mark> elements, e.g. for highlighted search terms.
                                configure=|mut builder: ammonia::Builder<'static>| {
                                    builder.add_tags(&["mark"]);
                                    builder
                                }
                            />
                        }
                    "#)}
                </Code>
                <p>
                    "Only allow what you need: every element and attribute you add is one more way for the HTML\u{2019}s "
                    "author to change your page."
                </p>
            </Section>

            <Section title="Sanitizing Without Rendering">
                <p>
                    "The "<Code inline=true>"leptonic::components::sanitized_html::sanitize"</Code>
                    " function returns the cleaned HTML as a "<Code inline=true>"String"</Code>", e.g. to clean HTML once "
                    "on the server before storing it. Its second argument configures the builder like the "
                    <Code inline=true>"configure"</Code>" prop; "<Code inline=true>"None"</Code>" uses the defaults."
                </p>
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::components::sanitized_html::sanitize;

                        let clean = sanitize("<b>Hello</b><script>alert('xss')</script>", None);
                        assert_eq!(clean, "<b>Hello</b>");
                    "#)}
                </Code>
            </Section>

            <Section title="Styling">
                <p>
                    "The element has no class of its own; pass "<Code inline=true>"classes"</Code>" and style the "
                    "elements inside it through that class. Headings, paragraphs and lists in it get the theme\u{2019}s "
                    <Link href=routes::doc::Typography.materialize()>"typography"</Link>" like any other."
                </p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Typography.materialize()>"Typography Components"</Link></li>
                <li><Link href=routes::doc::RichTextEditor.materialize()>"Rich Text Editor"</Link></li>
                <li><Link href=routes::doc::Layout.materialize()>"Content & Layout"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
