use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::i18n_provider::I18nProviderDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageI18nProvider() -> impl IntoView {
    view! {
        <DocPage title="I18nProvider">
            <p>
                "Leptonic\u{2019}s hooks format numbers, compare text and lay out arrow keys and overlays for a locale: a "
                "language, a region and the writing direction that follows from them. "<Code inline=true>"I18nProvider"</Code>
                " (in "<Code inline=true>"leptonic::utils::i18n"</Code>") sets that locale for everything rendered inside it, "
                "and "<AnchorLink href="#use-locale">"use_locale"</AnchorLink>", "
                <AnchorLink href="#use-direction">"use_direction"</AnchorLink>" and "
                <AnchorLink href="#use-i18n">"use_i18n"</AnchorLink>" read and change it. Without a provider, the locale is "
                "\u{201c}en-US\u{201d}, written left to right."
            </p>

            <ReactAria hook="I18nProvider"/>

            <Section title="Props">
                <ApiTable kind=ApiKind::Props of="I18nProvider">
                    <ApiRow name="locale" ty="Option<Locale>" default="None (\u{201c}en-US\u{201d})">
                        "The locale the provider starts with. Later changes go through "
                        <Code inline=true>"set_locale"</Code>" of "<AnchorLink href="#use-i18n">"use_i18n"</AnchorLink>"."
                    </ApiRow>
                    <ApiRow name="children" ty="Children">"Required. The part of the app that uses the locale."</ApiRow>
                </ApiTable>
            </Section>

            <Section title="Example">
                <p>"Wrap your app, usually right inside "<Code inline=true>"Root"</Code>":"</p>
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::utils::i18n::{I18nProvider, Locale, locale};

                        view! {
                            <I18nProvider locale=Locale::from(locale!("de-DE"))>
                                <App/>
                            </I18nProvider>
                        }
                    "#)}
                </Code>
                <p>
                    "The provider renders no element of its own. It doesn\u{2019}t set "<Code inline=true>"lang"</Code>" or "
                    <Code inline=true>"dir"</Code>" in the document either: render them on "<Code inline=true>"<html>"</Code>
                    " yourself, so that the browser and screen readers know the language (see "
                    <Link href=routes::doc::Accessibility.materialize()>"Accessibility"</Link>")."
                </p>
            </Section>

            <Section title="Demo">
                <p>
                    "The radio group sits inside the provider and changes its locale. The number is formatted with "
                    <Link href=routes::doc::utilities::NumberFormatter.materialize()>"use_number_formatter"</Link>
                    ", which follows the locale like the number fields and sliders do."
                </p>
                <Demo description="A locale switcher inside an I18nProvider, showing the locale, the writing direction and a formatted number" source=include_str!("demos/i18n_provider.rs")>
                    <I18nProviderDemo/>
                </Demo>
            </Section>

            <Section title="use_locale">
                <p>
                    <Code inline=true>"use_locale() -> Signal<Locale>"</Code>" returns the locale of the nearest "
                    <Code inline=true>"I18nProvider"</Code>", or a signal of \u{201c}en-US\u{201d} outside of one. It is "
                    "reactive: whatever reads it updates when the locale changes. Pass it to the "
                    <Link href=routes::doc::utilities::NumberFormatter.materialize()>"formatters"</Link>
                    " for your own text:"
                </p>
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::utils::{
                            i18n::use_locale,
                            list_formatter::{ListFormatOptions, ListFormatter},
                        };

                        let locale = use_locale();
                        let guests = move || {
                            ListFormatter::new(&locale.get(), &ListFormatOptions::default())
                                .format(&["Ada", "Grace", "Linus"])
                        };
                    "#)}
                </Code>
            </Section>

            <Section title="use_direction">
                <p>
                    <Code inline=true>"use_direction() -> Signal<WritingDirection>"</Code>" returns the writing direction of "
                    "the current locale. Hooks use it to swap "<Keys keys="ArrowLeft"/>" and "<Keys keys="ArrowRight"/>
                    " in right-to-left layouts and to mirror sliders and overlay placements at the start or end. Use it the "
                    "same way in your own components."
                </p>
            </Section>

            <Section title="use_i18n">
                <p>
                    <Code inline=true>"use_i18n() -> Option<I18nContext>"</Code>" returns the provider\u{2019}s context, "
                    <Code inline=true>"None"</Code>" outside of a provider. Use it to change the locale, e.g. from a language "
                    "menu:"
                </p>
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::utils::i18n::{Locale, locale, use_i18n};

                        if let Some(i18n) = use_i18n() {
                            i18n.set_locale.run(Locale::from(locale!("fr-FR")));
                        }
                    "#)}
                </Code>

                <Section title="I18nContext">
                    <ApiTable kind=ApiKind::Fields of="I18nContext">
                        <ApiRow name="locale" ty="Signal<Locale>">"The current locale."</ApiRow>
                        <ApiRow name="set_locale" ty="Callback<Locale>">"Changes the locale for everything inside the provider."</ApiRow>
                    </ApiTable>
                    <p>
                        "Its methods "<Code inline=true>"get_locale()"</Code>", "<Code inline=true>"direction()"</Code>" and "
                        <Code inline=true>"is_rtl()"</Code>" read the current locale, tracked like "
                        <Code inline=true>"locale.get()"</Code>"."
                    </p>
                </Section>
            </Section>

            <Section title="Locale">
                <p>
                    "A "<Code inline=true>"Locale"</Code>" wraps an ICU4X locale and knows its writing direction. Create one "
                    "from a literal checked at compile time with "<AnchorLink href="#locale-macro">"locale!"</AnchorLink>
                    ", or parse a BCP 47 tag at runtime, e.g. one stored in a cookie:"
                </p>
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::utils::i18n::{InvalidLocale, Locale};

                        let locale: Result<Locale, InvalidLocale> = "pt-BR".parse();
                    "#)}
                </Code>
                <ApiTable kind=ApiKind::Fields of="Locale">
                    <ApiRow name="direction" ty="WritingDirection">"The writing direction, derived from the locale\u{2019}s script."</ApiRow>
                </ApiTable>
                <DocTable headers=&["Method", "Returns"]>
                    <TableRow>
                        <TableCell><Code inline=true>"locale_str()"</Code></TableCell>
                        <TableCell>"The BCP 47 tag, e.g. \u{201c}de-DE\u{201d}."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"language()"</Code></TableCell>
                        <TableCell>"The language code, e.g. \u{201c}de\u{201d}."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"region()"</Code></TableCell>
                        <TableCell>"The region code, e.g. "<Code inline=true>"Some(\"DE\")"</Code>", if the tag has one."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"is_rtl()"</Code></TableCell>
                        <TableCell>"Whether the locale is written right to left."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"icu_locale()"</Code></TableCell>
                        <TableCell>"The wrapped "<Code inline=true>"icu_locale::Locale"</Code>", for calling ICU4X directly."</TableCell>
                    </TableRow>
                </DocTable>
                <p>
                    <Code inline=true>"Locale::default()"</Code>" is \u{201c}en-US\u{201d}. A string that isn\u{2019}t a valid "
                    "tag fails to parse with "<Code inline=true>"InvalidLocale"</Code>"."
                </p>

                <Section title="locale!" id="locale-macro">
                    <p>
                        <Code inline=true>"leptonic::utils::i18n::locale"</Code>" re-exports ICU4X\u{2019}s "
                        <Code inline=true>"locale!"</Code>" macro: an invalid tag is a compile error, so "
                        <Code inline=true>"Locale::from(locale!(\"de-DE\"))"</Code>" can\u{2019}t fail at runtime."
                    </p>
                </Section>
            </Section>

            <Section title="WritingDirection">
                <p>
                    <Code inline=true>"WritingDirection::Ltr"</Code>" or "<Code inline=true>"WritingDirection::Rtl"</Code>
                    " (in "<Code inline=true>"leptonic::utils::locale"</Code>"). Leptonic derives it from the script the "
                    "locale is written in: Arabic, Hebrew, Thaana, Syriac, Mandaic, N\u{2019}Ko, Adlam and Samaritan are "
                    "right to left, so \u{201c}ar\u{201d}, \u{201c}fa\u{201d}, \u{201c}he\u{201d} and \u{201c}ur\u{201d} are, "
                    "while \u{201c}zh\u{201d} or \u{201c}hi\u{201d} are left to right. "<Code inline=true>"is_rtl()"</Code>
                    " on a "<Code inline=true>"Locale"</Code>" or an "<Code inline=true>"I18nContext"</Code>" compares "
                    "against "<Code inline=true>"Rtl"</Code>"."
                </p>
            </Section>

            <Section title="Server-Side Rendering">
                <p>
                    "Leptonic never asks the browser for its language: the locale is the provider\u{2019}s, on the server "
                    "and in the browser alike, so formatted text hydrates without mismatches. If you choose the locale per "
                    "request, e.g. from a cookie or the "<Code inline=true>"Accept-Language"</Code>" header, make sure the "
                    "hydrating client starts with the same locale as the server. See "
                    <Link href=routes::doc::Ssr.materialize()>"Server-Side Rendering"</Link>"."
                </p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::utilities::NumberFormatter.materialize()>"NumberFormatter"</Link>" \u{2014} numbers, currencies and plural categories"</li>
                <li><Link href=routes::doc::utilities::DateTimeFormatter.materialize()>"DateTimeFormatter"</Link>" \u{2014} dates and times"</li>
                <li><Link href=routes::doc::utilities::ListFormatter.materialize()>"ListFormatter"</Link>" \u{2014} \u{201c}A, B and C\u{201d}"</li>
                <li><Link href=routes::doc::utilities::Collator.materialize()>"Collator"</Link>" \u{2014} sorting and filtering text"</li>
                <li><Link href=routes::doc::Ssr.materialize()>"Server-Side Rendering"</Link></li>
                <li><Link href=routes::doc::Accessibility.materialize()>"Accessibility"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
