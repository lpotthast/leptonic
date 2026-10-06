use leptonic::{
    components::prelude::*,
    hooks::{Orientation, collections::Key},
    utils::{
        filter::{Collator, CollatorOptions, Filter},
        i18n::Locale,
    },
};
use leptos::prelude::*;

const WORDS: [&str; 7] = ["Zebra", "Äpfel", "olive", "Ångström", "apple", "Öl", "Orange"];

#[component]
pub fn CollatorDemo() -> impl IntoView {
    let locale = RwSignal::new(Locale::default());
    let query = RwSignal::new(String::new());

    // Sorted for the locale, then filtered: case-insensitive ("base" sensitivity, the default).
    let words = move || {
        let locale = locale.get();
        let options = CollatorOptions::default();
        let collator = Collator::new(&locale, &options);
        let filter = Filter::new(&locale, &options);
        let mut words = WORDS.to_vec();
        words.sort_by(|a, b| collator.compare(a, b));
        query.with(|query| {
            words
                .into_iter()
                .filter(|word| filter.contains(word, query))
                .collect::<Vec<_>>()
        })
    };

    let on_locale_change = move |key: Option<Key>| {
        if let Some(new_locale) = key.as_ref().and_then(Key::as_str).and_then(|tag| tag.parse().ok()) {
            locale.set(new_locale);
        }
    };

    view! {
        <RadioGroup label="Locale" orientation=Orientation::Horizontal default_value="en-US" on_change=on_locale_change>
            <Radio value="en-US">"English (US)"</Radio>
            <Radio value="de-DE">"German"</Radio>
            <Radio value="sv-SE">"Swedish"</Radio>
        </RadioGroup>
        <SearchField label="Filter" value=query set_value=query/>
        <ol class="demo-collator-list">
            {move || words().into_iter().map(|word| view! { <li>{word}</li> }).collect_view()}
        </ol>
        <p class="demo-status">
            {move || match words().len() {
                1 => "1 word matches.".to_owned(),
                count => format!("{count} words match."),
            }}
        </p>
    }
}
