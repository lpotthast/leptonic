use leptonic::{
    components::prelude::*,
    hooks::{Orientation, collections::Key},
    utils::{
        i18n::Locale,
        number_formatter::{NumberFormatOptions, NumberFormatter, NumberStyle, SignDisplay},
        number_parser::NumberParser,
    },
};
use leptos::prelude::*;

#[component]
pub fn NumberFormatterDemo() -> impl IntoView {
    let locale = RwSignal::new(Locale::default());
    let text = RwSignal::new(String::from("1234.5"));

    // The typed text, read the way the selected locale writes numbers.
    let value = Memo::new(move |_| {
        NumberParser::new(&locale.get(), &NumberFormatOptions::default()).parse::<f64>(&text.get())
    });
    let format = move |options: NumberFormatOptions| {
        move || {
            value
                .get()
                .map(|value| NumberFormatter::new(&locale.get(), options.clone()).format(value))
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
            <Radio value="de-DE">"German"</Radio>
            <Radio value="hi-IN">"Hindi"</Radio>
            <Radio value="ar-EG">"Arabic (Egypt)"</Radio>
        </RadioGroup>
        <TextField label="Amount" value=text set_value=text/>
        <p class="demo-status">
            {move || match value.get() {
                Some(value) => format!("Parsed as {value}."),
                None => format!("Not a number in {}.", locale.with(Locale::locale_str)),
            }}
        </p>
        <dl class="demo-format-list">
            <dt>"Decimal"</dt>
            <dd>{format(NumberFormatOptions::default())}</dd>
            <dt>"Two fraction digits"</dt>
            <dd>
                {format(NumberFormatOptions {
                    minimum_fraction_digits: Some(2),
                    maximum_fraction_digits: Some(2),
                    ..Default::default()
                })}
            </dd>
            <dt>"Always signed, no grouping"</dt>
            <dd>
                {format(NumberFormatOptions {
                    sign_display: SignDisplay::Always,
                    use_grouping: false,
                    ..Default::default()
                })}
            </dd>
            <dt>"Euros"</dt>
            <dd>
                {format(NumberFormatOptions {
                    style: NumberStyle::Currency,
                    currency: Some("EUR".to_owned()),
                    ..Default::default()
                })}
            </dd>
        </dl>
    }
}
