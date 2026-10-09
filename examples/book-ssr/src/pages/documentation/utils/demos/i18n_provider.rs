use leptonic::{
    I18nProvider, Locale, NumberFormatOptions, Orientation, WritingDirection,
    atoms::{
        field::Label,
        radio::{RadioButton, RadioField, RadioGroup},
    },
    hooks::collections::Key,
    locale, use_direction, use_i18n, use_locale, use_number_formatter,
};
use leptos::prelude::*;

#[component]
pub fn I18nProviderDemo() -> impl IntoView {
    view! {
        <I18nProvider locale=Locale::from(locale!("en-US"))>
            <LocaleSwitcher/>
        </I18nProvider>
    }
}

/// Rendered inside the provider, so it reads and changes the provider's locale.
#[component]
fn LocaleSwitcher() -> impl IntoView {
    let locale = use_locale();
    let direction = use_direction();
    let i18n = use_i18n();
    // Follows the provider's locale, like the formatting of number fields and sliders.
    let formatter = use_number_formatter(Signal::stored(NumberFormatOptions::default()));

    let on_change = move |key: Option<Key>| {
        let new_locale = key
            .as_ref()
            .and_then(Key::as_str)
            .and_then(|tag| tag.parse::<Locale>().ok());
        if let (Some(new_locale), Some(i18n)) = (new_locale, i18n) {
            i18n.set_locale(new_locale);
        }
    };

    view! {
        <RadioGroup orientation=Orientation::Horizontal default_value=Key::from("en-US") on_change classes="demo-choice-group">
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
                <RadioField value="hi-IN">
                    <RadioButton classes="demo-radio">
                        <span class="demo-radio-circle" aria-hidden="true"></span>
                        "Hindi"
                    </RadioButton>
                </RadioField>
                <RadioField value="ar-EG">
                    <RadioButton classes="demo-radio">
                        <span class="demo-radio-circle" aria-hidden="true"></span>
                        "Arabic (Egypt)"
                    </RadioButton>
                </RadioField>
            </div>
        </RadioGroup>
        <dl class="demo-format-list">
            <dt>"use_locale"</dt>
            <dd>{move || locale.with(Locale::locale_str)}</dd>
            <dt>"use_direction"</dt>
            <dd>
                {move || match direction.get() {
                    WritingDirection::Ltr => "left to right",
                    WritingDirection::Rtl => "right to left",
                }}
            </dd>
            <dt>"1234567.891, formatted"</dt>
            <dd>{move || formatter.get().format(1_234_567.891)}</dd>
        </dl>
    }
}
