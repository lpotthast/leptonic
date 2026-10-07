use indoc::indoc;
use leptos::prelude::*;

use super::demos::collator::CollatorDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageCollator() -> impl IntoView {
    view! {
        <DocPage title="Collator">
            <p>
                "Sorting and searching text by comparing bytes gets languages wrong: \u{201c}\u{c4}pfel\u{201d} belongs "
                "next to \u{201c}apple\u{201d} in German but after \u{201c}Zebra\u{201d} in Swedish, and someone typing "
                "\u{201c}zebra\u{201d} expects to find \u{201c}Zebra\u{201d}. A "<Code inline=true>"Collator"</Code>
                " compares strings by the rules of a locale, and "<AnchorLink href="#filter">"Filter"</AnchorLink>
                " matches search text with them. Both use ICU4X, so the server sorts and filters exactly like the browser. "
                "The locale usually comes from the "
                <Link href=routes::doc::utilities::I18nProvider.materialize()>"I18nProvider"</Link>"."
            </p>

            <ReactAria hook="useCollator"/>

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r"
                        use leptonic::utils::{
                            filter::{Collator, CollatorOptions},
                            i18n::use_locale,
                        };

                        let locale = use_locale();
                        let sorted_names = move || {
                            let collator = Collator::new(&locale.get(), &CollatorOptions::default());
                            let mut names = names.get();
                            names.sort_by(|a, b| collator.compare(a, b));
                            names
                        };
                    ")}
                </Code>
            </Section>

            <Section title="Demo">
                <p>
                    "The words are sorted with a "<Code inline=true>"Collator"</Code>" and filtered with a "
                    <Code inline=true>"Filter"</Code>" of the selected locale. Compare German and Swedish, and type "
                    "\u{201c}o\u{201d} to see that case doesn\u{2019}t matter."
                </p>
                <Demo description="Words sorted and filtered in a selectable locale" source=include_str!("demos/collator.rs")>
                    <CollatorDemo/>
                </Demo>
            </Section>

            <Section title="Collator" id="collator-type">
                <DocTable headers=&["Method", "Description"]>
                    <TableRow>
                        <TableCell><Code inline=true>"Collator::new(locale, options)"</Code></TableCell>
                        <TableCell>
                            "A collator for a "<Code inline=true>"&Locale"</Code>" and "
                            <AnchorLink href="#collatoroptions">"&CollatorOptions"</AnchorLink>
                            ". Locales without collation data use the root order."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"compare(a, b)"</Code></TableCell>
                        <TableCell>
                            "An "<Code inline=true>"Ordering"</Code>", for "<Code inline=true>"sort_by"</Code>". Strings "
                            "that differ only in what the sensitivity ignores are "<Code inline=true>"Equal"</Code>"."
                        </TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="CollatorOptions">
                <ApiTable kind=ApiKind::Fields of="CollatorOptions">
                    <ApiRow name="sensitivity" ty="CollatorSensitivity" default="Base">
                        "Which differences count: "<Code inline=true>"Base"</Code>" only different letters (a = \u{e1} = A), "
                        <Code inline=true>"Accent"</Code>" also accents (a \u{2260} \u{e1}), "<Code inline=true>"Case"</Code>
                        " also case (a \u{2260} A), "<Code inline=true>"Variant"</Code>" all of them."
                    </ApiRow>
                    <ApiRow name="ignore_punctuation" ty="bool" default="false">
                        "Whether to ignore punctuation when comparing."
                    </ApiRow>
                </ApiTable>
                <p>
                    <Code inline=true>"ignore_punctuation"</Code>" has no effect yet, and "<Code inline=true>"Case"</Code>
                    " currently tells accents apart as well."
                </p>
            </Section>

            <Section title="Filter">
                <ReactAria hook="useFilter"/>
                <p>
                    "A "<Code inline=true>"Filter"</Code>" ("<Code inline=true>"Filter::new(locale, options)"</Code>
                    ", with the same options) checks whether a text matches what someone typed, comparing with a collator, "
                    "so the default "<Code inline=true>"Base"</Code>" sensitivity ignores case. Both strings are "
                    "normalized first, so a precomposed \u{201c}\u{e9}\u{201d} matches an \u{201c}e\u{201d} followed by a "
                    "combining accent."
                </p>
                <DocTable headers=&["Method", "Matches when"]>
                    <TableRow>
                        <TableCell><Code inline=true>"contains(text, query)"</Code></TableCell>
                        <TableCell><Code inline=true>"query"</Code>" appears anywhere in "<Code inline=true>"text"</Code>"."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"starts_with(text, query)"</Code></TableCell>
                        <TableCell><Code inline=true>"text"</Code>" begins with "<Code inline=true>"query"</Code>"."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"ends_with(text, query)"</Code></TableCell>
                        <TableCell><Code inline=true>"text"</Code>" ends with "<Code inline=true>"query"</Code>"."</TableCell>
                    </TableRow>
                </DocTable>
                <p>
                    "An empty query always matches. A filter compares pieces of the text as long as the query in bytes, so "
                    "a query only matches accented text when it has the same accents: \u{201c}apf\u{201d} doesn\u{2019}t find "
                    "\u{201c}\u{c4}pfel\u{201d} yet."
                </p>
                <p>
                    "Comboboxes filter their options with such a filter: see "<Code inline=true>"use_contains_filter"</Code>
                    " in "<Link href=routes::doc::combobox::Hook.materialize()>"Combobox Hooks"</Link>"."
                </p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::utilities::I18nProvider.materialize()>"I18nProvider"</Link>" \u{2014} where the locale comes from"</li>
                <li><Link href=routes::doc::utilities::ListFormatter.materialize()>"ListFormatter"</Link></li>
                <li><Link href=routes::doc::utilities::NumberFormatter.materialize()>"NumberFormatter"</Link></li>
                <li><Link href=routes::doc::Combobox.materialize()>"Combobox"</Link>" \u{2014} filters its options as you type"</li>
                <li><Link href=routes::doc::Table.materialize()>"Table"</Link>" \u{2014} sorts its rows"</li>
            </SeeAlso>
        </DocPage>
    }
}
