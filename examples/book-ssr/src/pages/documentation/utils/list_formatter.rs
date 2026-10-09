use indoc::indoc;
use leptos::prelude::*;

use super::demos::list_formatter::ListFormatterDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageListFormatter() -> impl IntoView {
    view! {
        <DocPage title="ListFormatter">
            <p>
                "A "<Code inline=true>"ListFormatter"</Code>" joins items the way a locale does: \u{201c}apples, pears, and "
                "plums\u{201d} in American English, \u{201c}apples, pears y plums\u{201d} in Spanish. Use it for labels and "
                "announcements that list things, such as the selected filters. It uses ICU4X, so the server renders the "
                "same text as the browser. The locale usually comes from the "
                <Link href=routes::doc::utilities::I18nProvider.materialize()>"I18nProvider"</Link>"."
            </p>

            <ReactAria hook="useListFormatter"/>

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::{
                            ListFormatOptions,
                            ListFormatType,
                            ListFormatter,
                            use_locale,
                        };

                        let locale = use_locale();
                        let hint = move || {
                            let formatter = ListFormatter::new(&locale.get(), &ListFormatOptions {
                                kind: ListFormatType::Disjunction,
                                ..Default::default()
                            });
                            format!("Pay with {}.", formatter.format(&["card", "PayPal", "invoice"]))
                        };
                    "#)}
                </Code>
            </Section>

            <Section title="Demo">
                <Demo description="A comma-separated list formatted as conjunction, disjunction and unit list in a selectable locale" source=include_str!("demos/list_formatter.rs")>
                    <ListFormatterDemo/>
                </Demo>
            </Section>

            <Section title="ListFormatter" id="listformatter-type">
                <DocTable headers=&["Method", "Description"]>
                    <TableRow>
                        <TableCell><Code inline=true>"ListFormatter::new(locale, options)"</Code></TableCell>
                        <TableCell>
                            "A formatter for a "<Code inline=true>"&Locale"</Code>" and "
                            <AnchorLink href="#listformatoptions">"&ListFormatOptions"</AnchorLink>
                            ". Locales without list data fall back to English."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"format(&[&str])"</Code></TableCell>
                        <TableCell>"The items joined into one string."</TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="ListFormatOptions">
                <ApiTable kind=ApiKind::Fields of="ListFormatOptions">
                    <ApiRow name="kind" ty="ListFormatType" default="Conjunction">
                        <Code inline=true>"Conjunction"</Code>" (\u{201c}A, B, and C\u{201d}), "<Code inline=true>"Disjunction"</Code>
                        " (\u{201c}A, B, or C\u{201d}) or "<Code inline=true>"Unit"</Code>" (\u{201c}A, B, C\u{201d}, e.g. for "
                        "measurements like \u{201c}5 ft, 7 in\u{201d})."
                    </ApiRow>
                    <ApiRow name="style" ty="ListFormatStyle" default="Long">
                        <Code inline=true>"Long"</Code>", "<Code inline=true>"Short"</Code>" (\u{201c}A, B, & C\u{201d}) or "
                        <Code inline=true>"Narrow"</Code>", the shortest the locale has."
                    </ApiRow>
                </ApiTable>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::utilities::I18nProvider.materialize()>"I18nProvider"</Link>" \u{2014} where the locale comes from"</li>
                <li><Link href=routes::doc::utilities::NumberFormatter.materialize()>"NumberFormatter"</Link>" \u{2014} also has plural categories for counts"</li>
                <li><Link href=routes::doc::utilities::DateTimeFormatter.materialize()>"DateTimeFormatter"</Link></li>
                <li><Link href=routes::doc::utilities::Collator.materialize()>"Collator"</Link>" \u{2014} sorts the items for a locale"</li>
            </SeeAlso>
        </DocPage>
    }
}
