use indoc::indoc;
use leptos::prelude::*;

use super::demos::number_formatter::NumberFormatterDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageNumberFormatter() -> impl IntoView {
    view! {
        <DocPage title="NumberFormatter">
            <p>
                "A "<Code inline=true>"NumberFormatter"</Code>" writes numbers the way a locale does: \u{201c}1,234.5\u{201d} "
                "in English, \u{201c}1.234,5\u{201d} in German, \u{201c}12,34,567\u{201d} in Hindi and with Arabic-Indic "
                "digits in Arabic. Number fields and sliders format their values with it, and "
                <AnchorLink href="#numberparser">"NumberParser"</AnchorLink>" reads such text back. Both use ICU4X, so "
                "the server renders the same text as the browser. The locale usually comes from the "
                <Link href=routes::doc::utilities::I18nProvider.materialize()>"I18nProvider"</Link>"."
            </p>

            <ReactAria hook="useNumberFormatter"/>

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::utils::{
                            i18n::{Locale, locale},
                            number_formatter::{NumberFormatOptions, NumberFormatter},
                        };

                        let formatter = NumberFormatter::new(
                            &Locale::from(locale!("de-DE")),
                            NumberFormatOptions {
                                maximum_fraction_digits: Some(1),
                                ..Default::default()
                            },
                        );
                        assert_eq!(formatter.format(1234.56), "1.234,6");
                    "#)}
                </Code>
            </Section>

            <Section title="Demo">
                <p>
                    "The amount is parsed with a "<Code inline=true>"NumberParser"</Code>" of the selected locale, then "
                    "formatted with several options. Switch to German and \u{201c}1234.5\u{201d} becomes twelve thousand "
                    "three hundred and forty-five: the period groups digits there."
                </p>
                <Demo description="An amount parsed and formatted in a selectable locale with different options" source=include_str!("demos/number_formatter.rs")>
                    <NumberFormatterDemo/>
                </Demo>
            </Section>

            <Section title="NumberFormatter" id="numberformatter-type">
                <DocTable headers=&["Method", "Description"]>
                    <TableRow>
                        <TableCell><Code inline=true>"NumberFormatter::new(locale, options)"</Code></TableCell>
                        <TableCell>
                            "A formatter for a "<Code inline=true>"&Locale"</Code>" and "
                            <AnchorLink href="#numberformatoptions">"NumberFormatOptions"</AnchorLink>
                            ". Creating one loads the locale\u{2019}s data, so keep it while the locale and options stay the same."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"format(value)"</Code></TableCell>
                        <TableCell>
                            "The formatted number. Takes any primitive integer or float ("<Code inline=true>"NumberValue"</Code>
                            "); integers are formatted exactly, also beyond what an "<Code inline=true>"f64"</Code>" holds. "
                            "Empty for NaN and infinite floats."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"options()"</Code></TableCell>
                        <TableCell>"The options the formatter was created with."</TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="NumberFormatOptions">
                <p>
                    "All fields have defaults ("<Code inline=true>"NumberFormatOptions::default()"</Code>"), so set only "
                    "what you need with struct update syntax."
                </p>
                <ApiTable kind=ApiKind::Fields of="NumberFormatOptions">
                    <ApiRow name="style" ty="NumberStyle" default="Decimal">
                        <Code inline=true>"Decimal"</Code>", "<Code inline=true>"Currency"</Code>", "
                        <Code inline=true>"Percent"</Code>" (the value times 100, with a percent sign) or "
                        <Code inline=true>"Unit"</Code>"."
                    </ApiRow>
                    <ApiRow name="currency" ty="Option<String>" default="None (\u{201c}USD\u{201d})">
                        "The ISO 4217 code of the currency, e.g. \u{201c}EUR\u{201d}, for the "<Code inline=true>"Currency"</Code>" style."
                    </ApiRow>
                    <ApiRow name="currency_display" ty="CurrencyDisplay" default="Symbol">
                        <Code inline=true>"Symbol"</Code>" (\u{201c}\u{20ac}\u{201d}), "<Code inline=true>"NarrowSymbol"</Code>", "
                        <Code inline=true>"Code"</Code>" (\u{201c}EUR\u{201d}) or "<Code inline=true>"Name"</Code>" (\u{201c}euros\u{201d})."
                    </ApiRow>
                    <ApiRow name="use_grouping" ty="bool" default="true">
                        "Whether to group digits, e.g. thousands."
                    </ApiRow>
                    <ApiRow name="minimum_integer_digits" ty="Option<u32>" default="None">
                        "Pads the integer part with zeros, e.g. 2 for \u{201c}07\u{201d}."
                    </ApiRow>
                    <ApiRow name="minimum_fraction_digits, maximum_fraction_digits" ty="Option<u32>" default="None">
                        "The fraction digits to show; the value is rounded half away from zero. Without them: 0 to 3 for "
                        "decimals, 2 for currencies, 0 for percentages."
                    </ApiRow>
                    <ApiRow name="minimum_significant_digits, maximum_significant_digits" ty="Option<u32>" default="None">
                        "Rounds to significant digits instead. With either set, the fraction digit options are ignored."
                    </ApiRow>
                    <ApiRow name="unit" ty="Option<String>" default="None">
                        "The unit for the "<Code inline=true>"Unit"</Code>" style, e.g. \u{201c}km\u{201d}."
                    </ApiRow>
                    <ApiRow name="unit_display" ty="UnitDisplay" default="Short">
                        <Code inline=true>"Short"</Code>" (\u{201c}16 km\u{201d}), "<Code inline=true>"Narrow"</Code>
                        " (\u{201c}16km\u{201d}) or "<Code inline=true>"Long"</Code>"."
                    </ApiRow>
                    <ApiRow name="sign_display" ty="SignDisplay" default="Auto">
                        <Code inline=true>"Auto"</Code>" (negative numbers only), "<Code inline=true>"Always"</Code>", "
                        <Code inline=true>"ExceptZero"</Code>" or "<Code inline=true>"Never"</Code>"."
                    </ApiRow>
                </ApiTable>
                <p>
                    "Digits, separators and grouping follow the locale. Currencies, percentages and units are written as "
                    "in English for now: the symbol, the percent sign and the unit don\u{2019}t move or get spaced for the "
                    "locale (\u{201c}\u{20ac}1.234,50\u{201d} in German), symbols come from a small table of common "
                    "currencies (others show \u{201c}$\u{201d}), and "<Code inline=true>"UnitDisplay::Long"</Code>
                    " appends an \u{201c}s\u{201d}."
                </p>
            </Section>

            <Section title="use_number_formatter">
                <p>
                    <Code inline=true>"use_number_formatter(options: Signal<NumberFormatOptions>) -> Memo<NumberFormatter>"</Code>
                    " returns a formatter for the current locale (see "
                    <Link href=routes::doc::utilities::I18nProvider.materialize()>"use_locale"</Link>
                    ") and the options, rebuilt only when either changes:"
                </p>
                <Code language=Language::Rust>
                    {indoc!(r"
                        use leptonic::utils::number_formatter::{NumberFormatOptions, NumberStyle, use_number_formatter};

                        let percent = use_number_formatter(Signal::stored(NumberFormatOptions {
                            style: NumberStyle::Percent,
                            ..Default::default()
                        }));

                        view! { <span>{move || percent.get().format(progress.get())}</span> }
                    ")}
                </Code>
            </Section>

            <Section title="NumberParser">
                <p>
                    "The inverse of the formatter: it reads text written the way a locale writes numbers. Number fields "
                    "parse what people type with it."
                </p>
                <DocTable headers=&["Method", "Description"]>
                    <TableRow>
                        <TableCell><Code inline=true>"NumberParser::new(locale, options)"</Code></TableCell>
                        <TableCell>
                            "A parser for a "<Code inline=true>"&Locale"</Code>" and "<Code inline=true>"&NumberFormatOptions"</Code>
                            ". It learns the locale\u{2019}s separators, minus sign, digits and the currency or unit text "
                            "from the formatter with the same options."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"parse::<T>(text)"</Code></TableCell>
                        <TableCell>
                            "The number as any primitive integer or float type, exactly. "<Code inline=true>"None"</Code>
                            " for empty or invalid text, and when "<Code inline=true>"T"</Code>" can\u{2019}t hold the value "
                            "(out of range, or fraction digits for an integer type)."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"is_valid_partial_number::<T>(text, min, max)"</Code></TableCell>
                        <TableCell>
                            "Whether "<Code inline=true>"text"</Code>" can still become a valid number while it is typed, "
                            "e.g. \u{201c}-\u{201d} or \u{201c}1,\u{201d} in German. A minus sign is only allowed when "
                            <Code inline=true>"min"</Code>" (or "<Code inline=true>"T"</Code>") admits negative numbers, a "
                            "decimal separator only for float types."
                        </TableCell>
                    </TableRow>
                </DocTable>
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::utils::{
                            i18n::{Locale, locale},
                            number_formatter::NumberFormatOptions,
                            number_parser::NumberParser,
                        };

                        let parser = NumberParser::new(&Locale::from(locale!("de-DE")), &NumberFormatOptions::default());
                        assert_eq!(parser.parse::<f64>("1.234,56"), Some(1234.56));
                        assert_eq!(parser.parse::<u8>("1.234"), None); // too large for a u8
                    "#)}
                </Code>
            </Section>

            <Section title="plural_category">
                <p>
                    <Code inline=true>"plural_category(locale, number: u64) -> PluralCategory"</Code>" (in "
                    <Code inline=true>"leptonic::utils::plurals"</Code>") tells which plural form a count takes in a "
                    "locale: English has \u{201c}one\u{201d} and \u{201c}other\u{201d} (1 item, 2 items), Polish adds "
                    "\u{201c}few\u{201d} and \u{201c}many\u{201d}, Arabic has six forms. Use it to pick the right text for "
                    "labels and announcements. Unsupported locales return "<Code inline=true>"Other"</Code>"."
                </p>
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use icu_plurals::PluralCategory;
                        use leptonic::utils::plurals::plural_category;

                        let label = match plural_category(&locale.get(), count) {
                            PluralCategory::One => format!("{count} file"),
                            _ => format!("{count} files"),
                        };
                    "#)}
                </Code>
                <p>
                    <Code inline=true>"PluralCategory"</Code>" is ICU4X\u{2019}s type, which leptonic doesn\u{2019}t re-export "
                    "yet: add the "<Code inline=true>"icu_plurals"</Code>" crate to match on it."
                </p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::utilities::I18nProvider.materialize()>"I18nProvider"</Link>" \u{2014} where the locale comes from"</li>
                <li><Link href=routes::doc::utilities::DateTimeFormatter.materialize()>"DateTimeFormatter"</Link></li>
                <li><Link href=routes::doc::utilities::ListFormatter.materialize()>"ListFormatter"</Link></li>
                <li><Link href=routes::doc::utilities::Collator.materialize()>"Collator"</Link></li>
                <li><Link href=routes::doc::NumberField.materialize()>"Number Field"</Link>" \u{2014} formats and parses its value with these types"</li>
            </SeeAlso>
        </DocPage>
    }
}
