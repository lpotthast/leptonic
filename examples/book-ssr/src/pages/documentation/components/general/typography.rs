use indoc::indoc;
use leptos::prelude::*;

use super::demos::{
    typography::TypographyDemo, typography_code::TypographyCodeDemo,
    typography_list::TypographyListDemo,
};
use crate::{kit::*, routes};

#[component]
pub fn PageTypography() -> impl IntoView {
    view! {
        <DocPage title="Typography Components">
            <p>
                "Typography is the look of your app\u{2019}s text: headings, paragraphs, lists and code. leptonic\u{2019}s "
                "theme styles plain headings and paragraphs, so you write them as regular HTML elements. For code, use the "
                <Code inline=true>"Code"</Code>" component; for lists, plain "<Code inline=true>"<ul>"</Code>" elements or "
                "the "<Code inline=true>"Ul"</Code>" component."
            </p>

            <Demo description="The theme's heading and paragraph styles, shown on plain text" source=include_str!("demos/typography.rs")>
                <TypographyDemo/>
            </Demo>

            <Section title="Headings and Paragraphs">
                <p>
                    "The theme styles "<Code inline=true>"<h1>"</Code>" to "<Code inline=true>"<h6>"</Code>" and "
                    <Code inline=true>"<p>"</Code>" without classes, through the "<Code inline=true>"--typography-*"</Code>
                    " variables listed under "<AnchorLink href="#styling">"Styling"</AnchorLink>". Write the elements in "
                    "the order of your page\u{2019}s outline, one "<Code inline=true>"<h1>"</Code>" per page and no levels "
                    "skipped: screen reader users move through a page by its headings."
                </p>
                <Code language=Language::Rust>
                    {indoc!(r#"
                        view! {
                            <h1>"Settings"</h1>
                            <p>"Changes are saved automatically."</p>
                            <h2>"Notifications"</h2>
                            <p>"Choose what you want to hear about."</p>
                        }
                    "#)}
                </Code>
                <p>
                    "The demo above shows the styles on list items instead of headings, so that it doesn\u{2019}t add six "
                    "headings to the outline of this page."
                </p>
            </Section>

            <Section title="Ul and Li">
                <p>
                    <Code inline=true>"Ul"</Code>" renders a "<Code inline=true>"<ul>"</Code>" with an "
                    <Code inline=true>"<li>"</Code>" for each "<Code inline=true>"Li"</Code>" slot. It has no styles of "
                    "its own; a plain "<Code inline=true>"<ul>"</Code>" looks the same."
                </p>
                <Demo description="A list of three items" source=include_str!("demos/typography_list.rs")>
                    <TypographyListDemo/>
                </Demo>
                <Section title="Props" id="ul-props">
                    <ApiTable kind=ApiKind::Props of="Ul">
                        <ApiRow name="li" ty="Vec<Li>" default="vec![]">
                            "The items, written as "<Code inline=true>"<Li slot>"</Code>" children."
                        </ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">
                            "Classes and styles of the "<Code inline=true>"<ul>"</Code>"."
                        </ApiRow>
                    </ApiTable>
                </Section>
                <Section title="Li">
                    <p>
                        "A slot of "<Code inline=true>"Ul"</Code>" holding the content of one item ("
                        <Code inline=true>"children"</Code>")."
                    </p>
                </Section>
            </Section>

            <Section title="Code">
                <p>
                    <Code inline=true>"Code"</Code>" renders its text as a code block, or inline with "
                    <Code inline=true>"inline=true"</Code>". With leptonic\u{2019}s "<Code inline=true>"syntax-highlight"</Code>
                    " feature, code blocks with a "<Code inline=true>"language"</Code>" are highlighted once they run in the "
                    "browser; the server renders plain text. Code blocks get a copy button, which requires the "
                    <Code inline=true>"clipboard"</Code>" feature."
                </p>
                <Demo description="A code block with a copy button, inline code, and the result of the last copy" source=include_str!("demos/typography_code.rs")>
                    <TypographyCodeDemo/>
                </Demo>

                <Section title="Props" id="code-props">
                    <ApiTable kind=ApiKind::Props of="Code">
                        <ApiRow name="children" ty="impl Into<Oco<'static, str>>">"The code. Required."</ApiRow>
                        <ApiRow name="inline" ty="Option<bool>" default="None">
                            "Renders the code inline instead of as a block."
                        </ApiRow>
                        <ApiRow name="language" ty="Option<Language>" default="None">
                            "The language to highlight. Languages without highlighting support show plain text; use "
                            <Code inline=true>"Language::Other"</Code>" for unlisted languages."
                        </ApiRow>
                        <ApiRow name="show_copy_button" ty="Option<bool>" default="None">
                            "Whether to show the copy button. Without a value, code blocks show it and inline code doesn\u{2019}t."
                        </ApiRow>
                        <ApiRow name="on_copy" ty="Option<Out<Result<(), ()>>>" default="None">
                            "Receives whether copying to the clipboard succeeded."
                        </ApiRow>
                        <ApiRow name="trusted" ty="Option<bool>" default="None">
                            "With "<Code inline=true>"Some(false)"</Code>" and the "<Code inline=true>"sanitize"</Code>
                            " feature, the highlighted HTML is sanitized before rendering. Treated as "
                            <Code inline=true>"true"</Code>" when not set."
                        </ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">
                            "Additional classes and styles."
                        </ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Styling">
                <p>"Override any of these CSS variables to adapt the typography to your design:"</p>
                <CssVariables prefix="--typography-" scss=theme_scss!("typography")/>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Layout.materialize()>"Content & Layout"</Link></li>
                <li><Link href=routes::doc::kbd::Component.materialize()>"Kbd Components"</Link></li>
                <li><Link href=routes::doc::SanitizedHtml.materialize()>"Sanitized HTML Component"</Link></li>
                <li><Link href=routes::doc::Themes.materialize()>"Themes"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
