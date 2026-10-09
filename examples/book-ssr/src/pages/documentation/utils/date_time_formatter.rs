use indoc::indoc;
use leptos::prelude::*;

use super::demos::date_time_formatter::DateTimeFormatterDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageDateTimeFormatter() -> impl IntoView {
    view! {
        <DocPage title="DateTimeFormatter">
            <p>
                "A "<Code inline=true>"DateTimeFormatter"</Code>" writes a "<Code inline=true>"jiff"</Code>" date or date and "
                "time the way a locale does: \u{201c}March 14, 2026\u{201d} in American English, \u{201c}14 March 2026\u{201d} "
                "in British English, \u{201c}14. M\u{e4}rz 2026\u{201d} in German. It uses ICU4X, so the server renders the "
                "same text as the browser. The locale usually comes from the "
                <Link href=routes::doc::utilities::I18nProvider.materialize()>"I18nProvider"</Link>"."
            </p>

            <ReactAria hook="useDateFormatter"/>

            <Section title="Example">
                <p>
                    "There is no hook for it: create the formatter where you format, with the locale from "
                    <Code inline=true>"use_locale"</Code>", so that the text follows locale changes."
                </p>
                <Code language=Language::Rust>
                    {indoc!(r"
                        use leptonic::{
                            DateTimeFormatOptions,
                            DateTimeFormatter,
                            DateTimeStyle,
                            use_locale,
                        };

                        let locale = use_locale();
                        // `post.published_at` is a `jiff::civil::DateTime`.
                        let published = move || {
                            DateTimeFormatter::new(&locale.get(), DateTimeFormatOptions {
                                date_style: Some(DateTimeStyle::Long),
                                ..Default::default()
                            })
                            .format(&post.published_at)
                        };
                    ")}
                </Code>
            </Section>

            <Section title="Demo">
                <Demo description="A fixed moment formatted with different date and time styles in a selectable locale" source=include_str!("demos/date_time_formatter.rs")>
                    <DateTimeFormatterDemo/>
                </Demo>
            </Section>

            <Section title="DateTimeFormatter" id="datetimeformatter-type">
                <DocTable headers=&["Method", "Description"]>
                    <TableRow>
                        <TableCell><Code inline=true>"DateTimeFormatter::new(locale, options)"</Code></TableCell>
                        <TableCell>
                            "A formatter for a "<Code inline=true>"&Locale"</Code>" and "
                            <AnchorLink href="#datetimeformatoptions">"DateTimeFormatOptions"</AnchorLink>"."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"format_date(date)"</Code></TableCell>
                        <TableCell>
                            "A "<Code inline=true>"jiff::civil::Date"</Code>" with the options\u{2019} weekday, era, year, month and "
                            "day, in the locale\u{2019}s own pattern, e.g. \u{201c}March 2026\u{201d} (year and long month) or "
                            "\u{201c}Saturday, March 14, 2026\u{201d}. Without any of these options, the ISO form."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"format(&date_time)"</Code></TableCell>
                        <TableCell>
                            "A "<Code inline=true>"jiff::civil::DateTime"</Code>", a wall-clock date and time, with the date and "
                            "time options or the styles, e.g. \u{201c}Mar 14, 2026, 3:09:26 PM\u{201d}. Without a time option, "
                            "the date only."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"format_zoned(&zoned)"</Code></TableCell>
                        <TableCell>
                            "A "<Code inline=true>"jiff::Zoned"</Code>" moment like "<Code inline=true>"format"</Code>", with its "
                            "time zone\u{2019}s name if "<Code inline=true>"time_zone_name"</Code>" asks for it, e.g. "
                            "\u{201c}3:09 PM EDT\u{201d}. To show it in another time zone, convert it with "
                            <Code inline=true>"Zoned::in_tz"</Code>" first."
                        </TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="DateTimeFormatOptions">
                <p>
                    "Choose a "<Code inline=true>"date_style"</Code>" and/or a "<Code inline=true>"time_style"</Code>
                    " for the locale\u{2019}s own patterns; that is what the demo shows. The other fields request single "
                    "parts of the date instead."
                </p>
                <ApiTable kind=ApiKind::Fields of="DateTimeFormatOptions">
                    <ApiRow name="date_style" ty="Option<DateTimeStyle>" default="None">
                        <Code inline=true>"Full"</Code>", "<Code inline=true>"Long"</Code>" (\u{201c}March 14, 2026\u{201d}), "
                        <Code inline=true>"Medium"</Code>" (\u{201c}Mar 14, 2026\u{201d}) or "<Code inline=true>"Short"</Code>
                        " (\u{201c}3/14/26\u{201d}). Together with a time style, the date style also decides the time\u{2019}s "
                        "precision: seconds for "<Code inline=true>"Full"</Code>" and "<Code inline=true>"Long"</Code>
                        ", minutes otherwise; "<Code inline=true>"Full"</Code>" then adds the weekday."
                    </ApiRow>
                    <ApiRow name="time_style" ty="Option<DateTimeStyle>" default="None">
                        "The time alone: "<Code inline=true>"Short"</Code>" with minutes, the others with seconds."
                    </ApiRow>
                    <ApiRow name="weekday, era" ty="Option<DateTimeFormat>" default="None">
                        <Code inline=true>"Long"</Code>" (\u{201c}Saturday\u{201d}), "<Code inline=true>"Short"</Code>" or "
                        <Code inline=true>"Narrow"</Code>"."
                    </ApiRow>
                    <ApiRow name="year, day, hour, minute, second" ty="Option<NumericFormat>" default="None">
                        <Code inline=true>"Numeric"</Code>" (\u{201c}7\u{201d}) or "<Code inline=true>"TwoDigit"</Code>" (\u{201c}07\u{201d})."
                    </ApiRow>
                    <ApiRow name="month" ty="Option<MonthFormat>" default="None">
                        <Code inline=true>"Numeric"</Code>", "<Code inline=true>"TwoDigit"</Code>", "<Code inline=true>"Long"</Code>
                        " (\u{201c}March\u{201d}), "<Code inline=true>"Short"</Code>" or "<Code inline=true>"Narrow"</Code>"."
                    </ApiRow>
                    <ApiRow name="hour_cycle" ty="Option<HourCycle>" default="None">
                        "12 ("<Code inline=true>"H12"</Code>") or 24 ("<Code inline=true>"H24"</Code>") hours. "
                        <Code inline=true>"None"</Code>": the locale\u{2019}s."
                    </ApiRow>
                    <ApiRow name="time_zone_name" ty="Option<TimeZoneFormat>" default="None">
                        <Code inline=true>"Long"</Code>" or "<Code inline=true>"Short"</Code>" time zone name ("
                        <Code inline=true>"format_zoned"</Code>" only)."
                    </ApiRow>
                </ApiTable>
                <p>
                    "The options are localized as a whole: the date parts from the weekday, year, month and day options, "
                    "the time precision from the finest of hour, minute and second. The length follows the month\u{2019}s or "
                    "the weekday\u{2019}s format, so a narrow weekday or month next to other parts is short, and a two-digit "
                    "month, day or hour pads all numbers. A single date part is written on its own (\u{201c}M\u{201d} for a "
                    "narrow March). The era shows only when asked for, and the year is never shortened by the locale. "
                    "Only the Gregorian calendar is supported."
                </p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::utilities::I18nProvider.materialize()>"I18nProvider"</Link>" \u{2014} where the locale comes from"</li>
                <li><Link href=routes::doc::utilities::NumberFormatter.materialize()>"NumberFormatter"</Link></li>
                <li><Link href=routes::doc::utilities::ListFormatter.materialize()>"ListFormatter"</Link></li>
                <li><Link href=routes::doc::DateTime.materialize()>"Date & Time"</Link>" \u{2014} calendars, date fields and pickers"</li>
            </SeeAlso>
        </DocPage>
    }
}
