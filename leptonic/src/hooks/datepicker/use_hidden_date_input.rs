// Upstream: react-aria-components/src/HiddenDateInput.tsx @ 99e6102368
// Upstream: react-aria-components/test/HiddenDateInput.test.js @ 99e6102368
// Upstream: react-aria-components/test/DateField.test.js @ 99e6102368
use leptos::{
    ev::{self},
    prelude::*,
};

use super::types::{DateValue, Granularity};
use crate::{
    EventHandler, IntoAttrs, OnEvent, PropsWithStyles,
    utils::{
        aria::AriaHidden,
        dom_ext::EventAccessors,
        focusability::{PreventFocusAttr, prevent_focus_attr},
        part::{Part, Remount},
        signal_attr::{FixedAttr, SignalAttr, fixed_attr, signal_attr},
        visually_hidden::visually_hidden_fixed_styles,
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - A hook of its own (react-aria-components: `useHiddenDateInput` inside the component file)
//   taking the field's value, granularity and a setter (react-aria: either state).
//
// ## DIFFERENT BEHAVIOR
// - A zoned value shows its own wall time (react-aria: converted to the local time zone).
// - An autofilled value is set as a whole, on the field's current value or placeholder (its time
//   zone and, for dates, its time); react-aria sets a date field's segments one by one first.
//
// =============================================================================

/// Input of [`use_hidden_date_input`].
pub struct UseHiddenDateInputInput<V: DateValue> {
    /// The field's value.
    pub value: Signal<Option<V>>,
    /// The value an autofilled date or time is put on (zone, and the time of dates): the field's
    /// value, else its placeholder.
    pub base: Signal<V>,
    /// The field's granularity: a `date` input for days, else `datetime-local`.
    pub granularity: Signal<Granularity>,
    /// Sets the autofilled value.
    pub set_value: Callback<Option<V>>,
    /// What the browser may fill in (`autocomplete`, e.g. `"bday"`).
    pub auto_complete: Option<String>,
    /// The field's name, telling autofill what it is (the input isn't submitted).
    pub name: Option<String>,
    pub is_disabled: Signal<bool>,
}

/// The return value of [`use_hidden_date_input`].
#[derive(Debug)]
pub struct UseHiddenDateInputReturn {
    /// The visually hidden container element (its styles hide it: merge them into the element's
    /// `style`).
    pub container: Part<PropsWithStyles<UseHiddenDateContainerProps>>,
    /// The hidden input element.
    pub input: Part<UseHiddenDateInputProps>,
}

/// Props of the hidden input's container.
#[derive(Debug)]
pub struct UseHiddenDateContainerProps {
    pub aria_hidden: AriaHidden,
    pub prevent_focus: PreventFocusAttr,
}

impl Remount for UseHiddenDateContainerProps {
    fn remount(&self) -> Self {
        Self {
            aria_hidden: self.aria_hidden,
            prevent_focus: self.prevent_focus.clone(),
        }
    }
}

impl IntoAttrs for UseHiddenDateContainerProps {
    type Attrs = (FixedAttr, PreventFocusAttr, FixedAttr);

    fn into_attrs(self) -> Self::Attrs {
        (
            fixed_attr("aria-hidden", self.aria_hidden),
            self.prevent_focus,
            fixed_attr("data-a11y-ignore", "aria-hidden-focus"),
        )
    }
}

/// Props of the hidden `<input type="date">` (or `datetime-local`).
#[derive(Debug)]
pub struct UseHiddenDateInputProps {
    pub input_type: Signal<&'static str>,
    pub tabindex: i32,
    pub autocomplete: Option<String>,
    pub disabled: Signal<bool>,
    /// Empty: the input isn't submitted with a form (the field's own hidden input is).
    pub form: &'static str,
    pub name: Option<String>,
    pub step: Signal<u32>,
    pub value: Signal<String>,
    pub on_input: EventHandler<web_sys::Event>,
}

impl Remount for UseHiddenDateInputProps {
    fn remount(&self) -> Self {
        Self {
            input_type: self.input_type,
            tabindex: self.tabindex,
            autocomplete: self.autocomplete.clone(),
            disabled: self.disabled,
            form: self.form,
            name: self.name.clone(),
            step: self.step,
            value: self.value,
            on_input: self.on_input.clone(),
        }
    }
}

impl IntoAttrs for UseHiddenDateInputProps {
    type Attrs = (
        SignalAttr<&'static str>,
        FixedAttr,
        FixedAttr,
        SignalAttr<bool>,
        FixedAttr,
        FixedAttr,
        SignalAttr<u32>,
        SignalAttr<String>,
        OnEvent<ev::input>,
    );

    fn into_attrs(self) -> Self::Attrs {
        (
            signal_attr("type", self.input_type),
            fixed_attr("tabindex", self.tabindex),
            fixed_attr("autocomplete", self.autocomplete),
            signal_attr("disabled", self.disabled),
            fixed_attr("form", self.form),
            fixed_attr("name", self.name),
            signal_attr("step", self.step),
            signal_attr("value", self.value),
            self.on_input.into_on(ev::input),
        )
    }
}

/// The value of a `date` or `datetime-local` input, to the minute or second.
fn input_value<V: DateValue>(value: &V, granularity: Granularity) -> String {
    let date_time = value.date_time();
    match granularity {
        Granularity::Day => date_time.date().to_string(),
        Granularity::Second => date_time.strftime("%Y-%m-%dT%H:%M:%S").to_string(),
        Granularity::Hour | Granularity::Minute => date_time.strftime("%Y-%m-%dT%H:%M").to_string(),
    }
}

/// Parses a `date` (`2000-05-30`) or `datetime-local` (`2000-05-30T08:15[:30]`) input value.
fn parse_input(text: &str) -> Option<jiff::civil::DateTime> {
    text.parse::<jiff::civil::DateTime>().ok().or_else(|| {
        text.parse::<jiff::civil::Date>()
            .ok()
            .map(|date| date.to_datetime(jiff::civil::Time::midnight()))
    })
}

/// A visually hidden `<input type="date">` (or `datetime-local`) beside a date field, so that
/// browsers can autofill it (react-aria-components' `HiddenDateInput`): it shows the field's value
/// and sets what the browser fills in.
pub fn use_hidden_date_input<V: DateValue>(
    input: UseHiddenDateInputInput<V>,
) -> UseHiddenDateInputReturn {
    crate::metrics::subject!();
    let UseHiddenDateInputInput {
        value,
        base,
        granularity,
        set_value,
        auto_complete,
        name,
        is_disabled,
    } = input;
    UseHiddenDateInputReturn {
        container: Part::stored(PropsWithStyles::new(
            UseHiddenDateContainerProps {
                aria_hidden: AriaHidden::True,
                prevent_focus: prevent_focus_attr(),
            },
            // Fixed at the top left, so that focusing it doesn't scroll the page.
            visually_hidden_fixed_styles(),
        )),
        input: Part::stored(UseHiddenDateInputProps {
            input_type: Signal::derive(move || {
                if granularity.get() == Granularity::Day {
                    "date"
                } else {
                    "datetime-local"
                }
            }),
            tabindex: -1,
            autocomplete: auto_complete,
            disabled: is_disabled,
            form: "",
            name,
            step: Signal::derive(move || match granularity.get() {
                Granularity::Second => 1,
                Granularity::Hour => 3600,
                Granularity::Day | Granularity::Minute => 60,
            }),
            value: Signal::derive(move || {
                value.with(|value| {
                    value
                        .as_ref()
                        .map(|value| input_value(value, granularity.get()))
                        .unwrap_or_default()
                })
            }),
            on_input: EventHandler::new(move |e: web_sys::Event| {
                let Some(input) =
                    wasm_bindgen::JsCast::dyn_ref::<web_sys::HtmlInputElement>(&e.expect_target())
                        .cloned()
                else {
                    return;
                };
                let Some(date_time) = parse_input(&input.value()) else {
                    return;
                };
                let base = base.get_untracked();
                let time = if granularity.get_untracked() == Granularity::Day {
                    base.time()
                } else {
                    date_time.time()
                };
                set_value.run(Some(base.with_fields(date_time.date(), time, None)));
            }),
        }),
    }
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;
    use jiff::civil::date;

    use super::*;

    #[test]
    fn formats_and_parses_input_values() {
        let value = date(2000, 5, 30).at(8, 15, 30, 0);
        assert_that!(input_value(&value, Granularity::Day)).is_equal_to("2000-05-30".to_owned());
        assert_that!(input_value(&value, Granularity::Minute))
            .is_equal_to("2000-05-30T08:15".to_owned());
        assert_that!(input_value(&value, Granularity::Second))
            .is_equal_to("2000-05-30T08:15:30".to_owned());
        assert_that!(parse_input("2000-05-30")).is_equal_to(Some(date(2000, 5, 30).at(0, 0, 0, 0)));
        assert_that!(parse_input("2000-05-30T08:15"))
            .is_equal_to(Some(date(2000, 5, 30).at(8, 15, 0, 0)));
        assert_that!(parse_input("")).is_none();
    }
}
