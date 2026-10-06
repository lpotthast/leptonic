use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::typography::TypographyDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageTypography() -> impl IntoView {
    view! {
        <DocPage title="Typography">
            <p>
                "leptonic\u{2019}s theme styles plain headings and paragraphs, so you write them as regular HTML "
                "elements. For code, use the "<Code inline=true>"Code"</Code>" component."
            </p>

            <Demo description="Headings, a paragraph, a code block and inline code" source=include_str!("demos/typography.rs")>
                <TypographyDemo/>
            </Demo>

            <Section title="Code">
                <p>
                    <Code inline=true>"Code"</Code>" renders its text as a code block, or inline with "
                    <Code inline=true>"inline=true"</Code>". With leptonic\u{2019}s "<Code inline=true>"syntax-highlight"</Code>
                    " feature, code blocks with a "<Code inline=true>"language"</Code>" are highlighted in the browser. "
                    "Code blocks get a copy button, which requires the "<Code inline=true>"clipboard"</Code>" feature."
                </p>

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

            <Section title="Styling">
                <p>"Override any of these CSS variables to adapt the typography to your design:"</p>
                <CssVariables prefix="--typography-" scss=theme_scss!("typography")/>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Themes.materialize()>"Themes"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
