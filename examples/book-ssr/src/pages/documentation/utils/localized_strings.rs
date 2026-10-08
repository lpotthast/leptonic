use indoc::indoc;
use leptos::prelude::*;

use super::demos::localized_strings::LocalizedStringsDemo;
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageLocalizedStrings() -> impl IntoView {
    view! {
        <DocPage title="use_localized_strings">
            <p>
                "Leptonic\u{2019}s hooks and atoms bring texts of their own: labels such as \u{201c}Clear search\u{201d}, "
                "descriptions such as a table\u{2019}s sort order, and screen reader announcements such as "
                "\u{201c}3 items selected.\u{201d} They come in 34 languages and follow the locale of the surrounding "
                <Link href=routes::doc::utilities::I18nProvider.materialize()>"I18nProvider"</Link>". "
                <Code inline=true>"use_localized_strings"</Code>" gives your own code the same messages."
            </p>

            <ReactAriaSource path="i18n/useLocalizedStringFormatter.ts"/>

            <Section title="The intl-strings Feature">
                <p>
                    "The messages of every language are part of leptonic\u{2019}s default "<Code inline=true>"intl-strings"</Code>
                    " feature. An app that declares leptonic with "<Code inline=true>"default-features = false"</Code>" adds it "
                    "explicitly; without it, every locale gets the English messages, and the bundle is smaller."
                </p>
                <Code language=Language::Shell>
                    {indoc!(r"
                        cargo add leptonic --no-default-features --features atoms,intl-strings
                    ")}
                </Code>
                <p>
                    "A locale without messages of its own uses those of another locale with its language (\u{201c}de-AT\u{201d} "
                    "gets \u{201c}de-DE\u{201d}\u{2019}s), else English. The languages: Arabic, Bulgarian, Chinese "
                    "(simplified and traditional), Croatian, Czech, Danish, Dutch, English, Estonian, Finnish, French, German, "
                    "Greek, Hebrew, Hungarian, Italian, Japanese, Korean, Latvian, Lithuanian, Norwegian, Polish, Portuguese "
                    "(Brazil and Portugal), Romanian, Russian, Serbian, Slovak, Slovenian, Spanish, Swedish, Turkish and "
                    "Ukrainian."
                </p>
            </Section>

            <Section title="Demo">
                <p>
                    "Switch the locale of the provider: a few of the messages leptonic\u{2019}s tables, grids, toasts and "
                    "search fields use follow it."
                </p>
                <Demo description="Messages of four families in a locale chosen with a radio group" source=include_str!("demos/localized_strings.rs")>
                    <LocalizedStringsDemo/>
                </Demo>
            </Section>

            <Section title="Usage">
                <p>
                    "Each family\u{2019}s messages are a struct ("<Code inline=true>"TableStrings"</Code>", "
                    <Code inline=true>"CalendarStrings"</Code>", \u{2026}) with one method per message, taking the "
                    "message\u{2019}s arguments: "<Code inline=true>"ascending_sort(column_name)"</Code>", "
                    <Code inline=true>"selected_count(count)"</Code>" (which picks the plural form of the locale). "
                    <Code inline=true>"use_localized_strings"</Code>" returns a "<Code inline=true>"Memo"</Code>" of a family\u{2019}s "
                    "messages in the current locale, so text read from it follows locale changes:"
                </p>
                <Code language=Language::Rust>
                    {indoc!(r"
                        use leptonic::utils::intl_strings::{TableStrings, use_localized_strings};
                        use leptos::prelude::*;

                        let strings = use_localized_strings::<TableStrings>();
                        let select_all = Signal::derive(move || strings.read().select_all());
                    ")}
                </Code>
                <p>
                    "Outside a component, e.g. on the server for a given request\u{2019}s locale, get the messages with "
                    <Code inline=true>"LocalizedStrings::for_locale(locale)"</Code>". "
                    <Code inline=true>"strings().by_key(key)"</Code>" looks a message up by its key, for code that computes "
                    "which message it needs."
                </p>
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::utils::{
                            i18n::{Locale, locale},
                            intl_strings::{LocalizedStrings, ToastStrings},
                        };

                        let strings = ToastStrings::for_locale(Locale::from(locale!("de-DE")));
                        let region_name = strings.notifications(2);
                    "#)}
                </Code>
            </Section>

            <Section title="Families">
                <DocTable headers=&["Struct", "Messages of"]>
                    <TableRow><TableCell><Code inline=true>"AtomStrings"</Code></TableCell><TableCell>"Texts the atoms add (e.g. a select\u{2019}s placeholder)"</TableCell></TableRow>
                    <TableRow><TableCell><Code inline=true>"BreadcrumbsStrings"</Code></TableCell><TableCell><Link href=routes::doc::Breadcrumbs.materialize()>"Breadcrumbs"</Link></TableCell></TableRow>
                    <TableRow><TableCell><Code inline=true>"CalendarStrings"</Code></TableCell><TableCell><Link href=routes::doc::Calendar.materialize()>"Calendar"</Link>" and range calendar"</TableCell></TableRow>
                    <TableRow><TableCell><Code inline=true>"ColorStrings"</Code>", "<Code inline=true>"ColorNameStrings"</Code></TableCell><TableCell>"The color controls, and color names ("<Link href=format!("{}#color-names", routes::doc::Color.materialize())>"Color"</Link>")"</TableCell></TableRow>
                    <TableRow><TableCell><Code inline=true>"ComboBoxStrings"</Code></TableCell><TableCell><Link href=routes::doc::Combobox.materialize()>"Combobox"</Link></TableCell></TableRow>
                    <TableRow><TableCell><Code inline=true>"DatePickerStrings"</Code>", "<Code inline=true>"DateValidationStrings"</Code></TableCell><TableCell>"Date and time fields and pickers: segment names, descriptions, validation messages"</TableCell></TableRow>
                    <TableRow><TableCell><Code inline=true>"DndStrings"</Code></TableCell><TableCell><Link href=routes::doc::DragAndDrop.materialize()>"Drag & Drop"</Link></TableCell></TableRow>
                    <TableRow><TableCell><Code inline=true>"GridStrings"</Code></TableCell><TableCell>"Selection in grids and grid lists"</TableCell></TableRow>
                    <TableRow><TableCell><Code inline=true>"MenuStrings"</Code></TableCell><TableCell><Link href=routes::doc::Menu.materialize()>"Menu"</Link></TableCell></TableRow>
                    <TableRow><TableCell><Code inline=true>"NumberFieldStrings"</Code>", "<Code inline=true>"SpinButtonStrings"</Code></TableCell><TableCell><Link href=routes::doc::NumberField.materialize()>"Number Field"</Link>" and spin buttons"</TableCell></TableRow>
                    <TableRow><TableCell><Code inline=true>"OverlayStrings"</Code></TableCell><TableCell>"Overlays (the dismiss button)"</TableCell></TableRow>
                    <TableRow><TableCell><Code inline=true>"SearchFieldStrings"</Code></TableCell><TableCell><Link href=routes::doc::SearchField.materialize()>"Search Field"</Link></TableCell></TableRow>
                    <TableRow><TableCell><Code inline=true>"TableStrings"</Code></TableCell><TableCell><Link href=routes::doc::Table.materialize()>"Table"</Link>": selection, sorting, resizing, expanding"</TableCell></TableRow>
                    <TableRow><TableCell><Code inline=true>"TagStrings"</Code></TableCell><TableCell><Link href=routes::doc::TagGroup.materialize()>"Tag Group"</Link></TableCell></TableRow>
                    <TableRow><TableCell><Code inline=true>"ToastStrings"</Code></TableCell><TableCell><Link href=routes::doc::Toast.materialize()>"Toast"</Link></TableCell></TableRow>
                    <TableRow><TableCell><Code inline=true>"TreeStrings"</Code></TableCell><TableCell><Link href=routes::doc::Tree.materialize()>"Tree"</Link></TableCell></TableRow>
                </DocTable>
                <p>
                    "To change a text for your app, pass your own where the hook or atom takes it ("
                    <Code inline=true>"aria_label"</Code>", a placeholder, \u{2026}): yours replaces the message."
                </p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::utilities::I18nProvider.materialize()>"I18nProvider"</Link>" \u{2014} where the locale comes from"</li>
                <li><Link href=routes::doc::utilities::NumberFormatter.materialize()>"NumberFormatter"</Link></li>
                <li><Link href=routes::doc::utilities::ListFormatter.materialize()>"ListFormatter"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
