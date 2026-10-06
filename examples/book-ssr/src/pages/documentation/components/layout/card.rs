use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::card::CardDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageCardAndTile() -> impl IntoView {
    view! {
        <DocPage title="Card & Tile Components">
            <p>
                "A card groups content that belongs together, such as a summary with its actions, on a surface that stands "
                "out from the page. The "<Code inline=true>"Card"</Code>" component renders that surface: a "
                <Code inline=true>"<div>"</Code>" with padding, rounded corners and a shadow from the theme. "
                <Code inline=true>"Tile"</Code>" renders the same kind of container without any styling of its own."
            </p>

            <Demo description="Card with a heading, text and a subscribe button showing the subscription state" source=include_str!("demos/card.rs")>
                <CardDemo/>
            </Demo>

            <Section title="Card">
                <Section title="Props" id="card-props">
                    <ApiTable kind=ApiKind::Props of="Card">
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">
                            "Additional classes and styles of the card element."
                        </ApiRow>
                        <ApiRow name="children" ty="Children">"The card\u{2019}s content."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Accessibility" id="card-accessibility">
                    <p>
                        "A card is a plain "<Code inline=true>"<div>"</Code>" without a role: screen readers don\u{2019}t "
                        "announce it as a group. Give each card a heading, so that its content can be found with heading "
                        "navigation, and render a list of cards as a list ("<Code inline=true>"<ul>"</Code>" with a card in each "
                        <Code inline=true>"<li>"</Code>"), so that screen readers announce how many there are."
                    </p>
                </Section>
            </Section>

            <Section title="Tile">
                <p>
                    <Code inline=true>"Tile"</Code>" renders a "<Code inline=true>"<div class=\"leptonic-tile\">"</Code>
                    " around its children. The theme has no rules for that class, so a tile looks like whatever you style "
                    "it as. Use it where a card\u{2019}s padding and shadow don\u{2019}t fit, e.g. for the cells of a "
                    "dashboard grid:"
                </p>

                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::components::prelude::*;

                        view! {
                            <Tile classes="stat-tile">
                                <span class="stat-label">"Open issues"</span>
                                <span class="stat-value">"42"</span>
                            </Tile>
                        }
                    "#)}
                </Code>

                <Code language=Language::Css>
                    {indoc!(r"
                        .stat-tile {
                            display: flex;
                            flex-direction: column;
                            padding: 1em;
                            border: 1px solid var(--border);
                            border-radius: 0.25em;
                        }
                    ")}
                </Code>

                <Section title="Props" id="tile-props">
                    <ApiTable kind=ApiKind::Props of="Tile">
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">
                            "Additional classes and styles of the tile element."
                        </ApiRow>
                        <ApiRow name="children" ty="Children">"The tile\u{2019}s content."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Styling">
                <p>
                    "Cards have the class "<Code inline=true>".leptonic-card"</Code>". Their background and shadow come from "
                    "these theme variables, which the light and dark theme define:"
                </p>
                <CssVariables prefix="--card-" scss=theme_scss!("card")/>
                <p>
                    "The card also has a vertical margin of 1.5em. Override "<Code inline=true>"margin"</Code>" through "
                    <Code inline=true>"classes"</Code>" or "<Code inline=true>"styles"</Code>" when cards sit in a grid or a "
                    "stack that spaces them already."
                </p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Layout.materialize()>"Content & Layout"</Link></li>
                <li><Link href=routes::doc::Stack.materialize()>"Stack Component"</Link></li>
                <li><Link href=routes::doc::GridLayout.materialize()>"Grid Layout Components"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
