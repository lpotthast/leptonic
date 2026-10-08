use leptonic::{
    atoms::{
        field::Label,
        radio::{RadioButton, RadioField, RadioGroup},
    },
    hooks::{Orientation, collections::Key},
    utils::{
        i18n::{I18nProvider, Locale, locale, use_i18n},
        intl_strings::{
            GridStrings, SearchFieldStrings, TableStrings, ToastStrings, use_localized_strings,
        },
    },
};
use leptos::prelude::*;

/// The locales to choose from: tag and name.
const LOCALES: [(&str, &str); 4] = [
    ("en-US", "English"),
    ("de-DE", "German"),
    ("ja-JP", "Japanese"),
    ("ar-AE", "Arabic"),
];

#[component]
pub fn LocalizedStringsDemo() -> impl IntoView {
    view! {
        <I18nProvider locale=Locale::from(locale!("en-US"))>
            <LocaleChoice/>
            <Messages/>
        </I18nProvider>
    }
}

/// Changes the locale of the surrounding provider.
#[component]
fn LocaleChoice() -> impl IntoView {
    let set_locale = use_i18n().map(|i18n| i18n.set_locale);
    let on_change = move |key: Option<Key>| {
        let new_locale = key
            .as_ref()
            .and_then(Key::as_str)
            .and_then(|tag| tag.parse::<Locale>().ok());
        if let (Some(new_locale), Some(set_locale)) = (new_locale, set_locale) {
            set_locale.run(new_locale);
        }
    };

    view! {
        <RadioGroup orientation=Orientation::Horizontal default_value=Key::from("en-US") on_change classes="demo-choice-group">
            <Label classes="demo-choice-group-label">"Locale"</Label>
            <div class="demo-choice-group-items">
                {LOCALES
                    .map(|(tag, name)| view! {
                        <RadioField value=tag>
                            <RadioButton classes="demo-radio">
                                <span class="demo-radio-circle" aria-hidden="true"></span>
                                {name}
                            </RadioButton>
                        </RadioField>
                    })
                    .collect_view()}
            </div>
        </RadioGroup>
    }
}

/// Some of the messages leptonic's hooks use, in the provider's locale.
#[component]
fn Messages() -> impl IntoView {
    let table = use_localized_strings::<TableStrings>();
    let grid = use_localized_strings::<GridStrings>();
    let toast = use_localized_strings::<ToastStrings>();
    let search_field = use_localized_strings::<SearchFieldStrings>();

    view! {
        <dl class="demo-format-list">
            <dt>"Table: sort description"</dt>
            <dd>{move || table.read().ascending_sort("Name")}</dd>
            <dt>"Grid: selection announcement"</dt>
            <dd>{move || grid.read().selected_count(3)}</dd>
            <dt>"Toast region: name"</dt>
            <dd>{move || toast.read().notifications(2)}</dd>
            <dt>"Search field: clear button"</dt>
            <dd>{move || search_field.read().clear_search()}</dd>
        </dl>
    }
}
