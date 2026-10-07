// Upstream: react-aria/src/meter/useMeter.ts @ 99e6102368
use leptos::prelude::*;

use crate::{
    hooks::{UseProgressBarInput, UseProgressBarReturn, progress::use_progress_bar::progress},
    utils::{aria::AriaRole, number_formatter::NumberFormatOptions, number_value::NumberValue},
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - As `use_progress_bar` (generic value type, string `value_label`, the offered DOM props, the
//   returned `percentage` and `value_text`), with a value that is always known.
// - Returns `use_progress_bar`'s types (`props` with `role="meter"`).
//
// =============================================================================

/// Input of [`use_meter`].
#[derive(Debug, Clone)]
pub struct UseMeterInput<T: NumberValue> {
    /// The value, clamped to the range.
    pub value: Signal<T>,
    /// Default: 0.
    pub min_value: Signal<T>,
    /// Default: 100.
    pub max_value: Signal<T>,
    /// How the value text is formatted. A percent style formats the percentage, other styles the
    /// value. Default: percent.
    pub format_options: Signal<NumberFormatOptions>,
    /// Replaces the formatted value text (e.g. "3 of 4 GB").
    pub value_label: MaybeProp<String>,
    /// The element's id. Generated when `None`.
    pub id: Option<String>,
    /// Whether a visible label is rendered (with `label_props`).
    pub has_label: Signal<bool>,
    /// Names the meter when there is no visible label.
    pub aria_label: MaybeProp<String>,
    pub aria_labelledby: Option<String>,
    pub aria_describedby: Option<String>,
}

impl<T: NumberValue> Default for UseMeterInput<T> {
    /// 0 in the range 0 to 100, formatted as a percentage.
    fn default() -> Self {
        let UseProgressBarInput {
            min_value,
            max_value,
            format_options,
            value_label,
            id,
            has_label,
            aria_label,
            aria_labelledby,
            aria_describedby,
            ..
        } = UseProgressBarInput::default();
        Self {
            value: Signal::stored(T::ZERO),
            min_value,
            max_value,
            format_options,
            value_label,
            id,
            has_label,
            aria_label,
            aria_labelledby,
            aria_describedby,
        }
    }
}

/// Provides the accessibility of a meter: a value within a known range (disk usage, battery
/// level), unlike a progress bar not the progress of an operation.
///
/// ```ignore
/// let meter = use_meter(UseMeterInput {
///     value: Signal::stored(75),
///     has_label: true.into(),
///     ..UseMeterInput::default()
/// });
/// view! {
///     <div {..meter.props.into_attrs()}>
///         <span {..meter.label_props.into_attrs()}>"Storage"</span>
///         <span>{meter.value_text}</span>
///     </div>
/// }
/// ```
pub fn use_meter<T: NumberValue>(input: UseMeterInput<T>) -> UseProgressBarReturn {
    let UseMeterInput {
        value,
        min_value,
        max_value,
        format_options,
        value_label,
        id,
        has_label,
        aria_label,
        aria_labelledby,
        aria_describedby,
    } = input;
    progress(
        UseProgressBarInput {
            value: Signal::derive(move || Some(value.get())),
            min_value,
            max_value,
            format_options,
            value_label,
            id,
            has_label,
            aria_label,
            aria_labelledby,
            aria_describedby,
        },
        AriaRole::Meter,
    )
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;

    // Upstream: RAC Meter.test.js "renders" and "supports a custom range".
    #[test]
    fn a_meter_has_the_meter_role_and_a_value() {
        Owner::new().with(|| {
            let meter = use_meter(UseMeterInput {
                value: Signal::stored(3),
                max_value: Signal::stored(6),
                aria_label: "Storage".into(),
                ..UseMeterInput::default()
            });
            assert_that!(meter.props.role).is_equal_to(AriaRole::Meter);
            assert_that!(meter.props.aria_valuenow.get_untracked())
                .is_equal_to(Some("3".to_owned()));
            assert_that!(meter.props.aria_valuetext.get_untracked())
                .is_equal_to(Some("50%".to_owned()));
            assert_that!(
                meter
                    .percentage
                    .get_untracked()
                    .map(crate::utils::fraction::Fraction::as_percent)
            )
            .is_equal_to(Some(50.0));
        });
    }
}
