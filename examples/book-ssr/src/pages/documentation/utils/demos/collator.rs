use leptonic::{
    Collator, CollatorOptions, Filter, FilterQuery, Locale, Orientation,
    atoms::{
        field::Label,
        input::Input,
        radio::{RadioButton, RadioField, RadioGroup},
        search_field::SearchField,
    },
    hooks::collections::Key,
};
use leptos::prelude::*;

const WORDS: [&str; 7] = [
    "Zebra",
    "Äpfel",
    "olive",
    "Ångström",
    "apple",
    "Öl",
    "Orange",
];

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
            let query = FilterQuery::new(query);
            words
                .into_iter()
                .filter(|word| filter.contains(word, &query))
                .collect::<Vec<_>>()
        })
    };

    let on_locale_change = move |key: Option<Key>| {
        if let Some(new_locale) = key
            .as_ref()
            .and_then(Key::as_str)
            .and_then(|tag| tag.parse().ok())
        {
            locale.set(new_locale);
        }
    };

    view! {
        <RadioGroup orientation=Orientation::Horizontal default_value=Key::from("en-US") on_change=on_locale_change classes="demo-choice-group">
            <Label classes="demo-choice-group-label">"Locale"</Label>
            <div class="demo-choice-group-items">
                <RadioField value="en-US">
                    <RadioButton classes="demo-radio">
                        <span class="demo-radio-circle" aria-hidden="true"></span>
                        "English (US)"
                    </RadioButton>
                </RadioField>
                <RadioField value="de-DE">
                    <RadioButton classes="demo-radio">
                        <span class="demo-radio-circle" aria-hidden="true"></span>
                        "German"
                    </RadioButton>
                </RadioField>
                <RadioField value="sv-SE">
                    <RadioButton classes="demo-radio">
                        <span class="demo-radio-circle" aria-hidden="true"></span>
                        "Swedish"
                    </RadioButton>
                </RadioField>
            </div>
        </RadioGroup>
        <SearchField value=query set_value=query classes=["demo-field", "demo-mt-1"]>
            <Label classes="demo-field-label">"Filter"</Label>
            <Input classes="demo-atom-input"/>
        </SearchField>
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
