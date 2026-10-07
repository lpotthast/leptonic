use leptonic::{
    components::prelude::*,
    hooks::{Orientation, collections::Key},
    utils::{
        i18n::Locale,
        list_formatter::{ListFormatOptions, ListFormatStyle, ListFormatType, ListFormatter},
    },
};
use leptos::prelude::*;

#[component]
pub fn ListFormatterDemo() -> impl IntoView {
    let locale = RwSignal::new(Locale::default());
    let text = RwSignal::new(String::from("apples, pears, plums"));

    let format = move |r#type: ListFormatType, style: ListFormatStyle| {
        move || {
            let items = text.with(|text| {
                text.split(',')
                    .map(str::trim)
                    .filter(|item| !item.is_empty())
                    .map(ToOwned::to_owned)
                    .collect::<Vec<_>>()
            });
            let items: Vec<&str> = items.iter().map(String::as_str).collect();
            ListFormatter::new(&locale.get(), &ListFormatOptions { r#type, style }).format(&items)
        }
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
        <RadioGroup label="Locale" orientation=Orientation::Horizontal default_value="en-US" on_change=on_locale_change>
            <Radio value="en-US">"English (US)"</Radio>
            <Radio value="es-ES">"Spanish"</Radio>
            <Radio value="de-DE">"German"</Radio>
            <Radio value="ja-JP">"Japanese"</Radio>
        </RadioGroup>
        <TextField label="Items, separated by commas" value=text set_value=text/>
        <dl class="demo-format-list">
            <dt>"Conjunction"</dt>
            <dd>{format(ListFormatType::Conjunction, ListFormatStyle::Long)}</dd>
            <dt>"Disjunction"</dt>
            <dd>{format(ListFormatType::Disjunction, ListFormatStyle::Long)}</dd>
            <dt>"Unit, narrow"</dt>
            <dd>{format(ListFormatType::Unit, ListFormatStyle::Narrow)}</dd>
        </dl>
    }
}
