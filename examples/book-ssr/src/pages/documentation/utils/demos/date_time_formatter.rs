use leptonic::{
    atoms::{
        field::Label,
        radio::{Radio, RadioGroup},
    },
    hooks::{Orientation, collections::Key},
    jiff::civil::date,
    utils::{
        date_time_formatter::{DateTimeFormatOptions, DateTimeFormatter, DateTimeStyle},
        i18n::Locale,
    },
};
use leptos::prelude::*;

#[component]
pub fn DateTimeFormatterDemo() -> impl IntoView {
    let locale = RwSignal::new(Locale::default());
    // A fixed moment (14 March 2026, 15:09:26), so that the server renders the same text as the browser.
    let moment = date(2026, 3, 14).at(15, 9, 26, 0);

    let format = move |date_style: Option<DateTimeStyle>, time_style: Option<DateTimeStyle>| {
        move || {
            let options = DateTimeFormatOptions {
                date_style,
                time_style,
                ..Default::default()
            };
            DateTimeFormatter::new(&locale.get(), options).format(&moment)
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
        <RadioGroup orientation=Orientation::Horizontal default_value=Key::from("en-US") on_change=on_locale_change classes="demo-choice-group">
            <Label classes="demo-choice-group-label">"Locale"</Label>
            <div class="demo-choice-group-items">
                <Radio value="en-US" classes="demo-radio">
                    <span class="demo-radio-circle" aria-hidden="true"></span>
                    "English (US)"
                </Radio>
                <Radio value="en-GB" classes="demo-radio">
                    <span class="demo-radio-circle" aria-hidden="true"></span>
                    "English (UK)"
                </Radio>
                <Radio value="de-DE" classes="demo-radio">
                    <span class="demo-radio-circle" aria-hidden="true"></span>
                    "German"
                </Radio>
                <Radio value="ja-JP" classes="demo-radio">
                    <span class="demo-radio-circle" aria-hidden="true"></span>
                    "Japanese"
                </Radio>
            </div>
        </RadioGroup>
        <dl class="demo-format-list">
            <dt>"Long date"</dt>
            <dd>{format(Some(DateTimeStyle::Long), None)}</dd>
            <dt>"Medium date"</dt>
            <dd>{format(Some(DateTimeStyle::Medium), None)}</dd>
            <dt>"Short date"</dt>
            <dd>{format(Some(DateTimeStyle::Short), None)}</dd>
            <dt>"Short time"</dt>
            <dd>{format(None, Some(DateTimeStyle::Short))}</dd>
            <dt>"Medium date and time"</dt>
            <dd>{format(Some(DateTimeStyle::Medium), Some(DateTimeStyle::Medium))}</dd>
        </dl>
    }
}
