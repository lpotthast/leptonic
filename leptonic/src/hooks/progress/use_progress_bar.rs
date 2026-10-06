// Upstream: react-aria/src/progress/useProgressBar.ts @ 99e6102368
use leptos::{
    attr::{self, Attr},
    prelude::*,
};

use crate::{
    hooks::{
        IntoAttrs, LabelElementType, UseLabelFieldAttrs, UseLabelInput, UseLabelProps, use_label,
    },
    utils::{
        aria::AriaRole,
        math::percentage_in_range,
        number_formatter::{NumberFormatOptions, NumberStyle, use_number_formatter},
        number_value::NumberValue,
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - The value is an `Option`: `None` is indeterminate progress (react-aria: `value` plus a
//   separate `isIndeterminate` flag, which ignores the value). Reason: one state, no impossible
//   combinations.
// - Generic over the value type (`T: NumberValue`, e.g. `u64` bytes) instead of JS's `number`.
// - `value_label` is a string (react-aria: any `ReactNode`, used as `aria-valuetext`).
// - Of the DOM props react-aria passes through (`filterDOMProps` with labelable props), only `id`,
//   `aria-label`, `aria-labelledby` and `aria-describedby` are offered.
// - Also returns the `percentage` (0 to 100) and the `value_text`, which react-aria-components
//   computes in its `ProgressBar` for render props.
//
// =============================================================================

/// Input of [`use_progress_bar`] (and, with its own value type, [`use_meter`](crate::hooks::use_meter)).
#[derive(Debug, Clone)]
pub struct UseProgressBarInput<T: NumberValue> {
    /// The progress, clamped to the range. `None`: indeterminate (the progress isn't known).
    pub value: Signal<Option<T>>,
    /// Default: 0.
    pub min_value: Signal<T>,
    /// Default: 100.
    pub max_value: Signal<T>,
    /// How the value text is formatted. A percent style formats the percentage, other styles the
    /// value. Default: percent.
    pub format_options: Signal<NumberFormatOptions>,
    /// Replaces the formatted value text (e.g. "1 of 4").
    pub value_label: MaybeProp<String>,
    /// The element's id. Generated when `None`.
    pub id: Option<String>,
    /// Whether a visible label is rendered (with `label_props`).
    pub has_label: Signal<bool>,
    /// Names the progress bar when there is no visible label.
    pub aria_label: MaybeProp<String>,
    pub aria_labelledby: Option<String>,
    pub aria_describedby: Option<String>,
}

impl<T: NumberValue> Default for UseProgressBarInput<T> {
    /// No progress yet (0) in the range 0 to 100, formatted as a percentage.
    fn default() -> Self {
        Self {
            value: Signal::stored(Some(T::ZERO)),
            min_value: Signal::stored(T::ZERO),
            max_value: Signal::stored(T::from_f64(100.0).expect("100 fits every number type")),
            format_options: Signal::stored(NumberFormatOptions {
                style: NumberStyle::Percent,
                ..NumberFormatOptions::default()
            }),
            value_label: MaybeProp::default(),
            id: None,
            has_label: Signal::stored(false),
            aria_label: MaybeProp::default(),
            aria_labelledby: None,
            aria_describedby: None,
        }
    }
}

/// Return value of [`use_progress_bar`].
#[derive(Debug)]
pub struct UseProgressBarReturn {
    /// For the progress bar element.
    pub props: UseProgressBarProps,
    /// For the visible label, a `<span>` (a progress bar isn't labelable by a `<label>`).
    pub label_props: UseLabelProps,
    /// The progress in percent (0 to 100); `None` while indeterminate.
    pub percentage: Signal<Option<f64>>,
    /// The formatted value (or `value_label`); `None` while indeterminate.
    pub value_text: Signal<Option<String>>,
}

/// Props for the progress bar (or meter) element.
#[derive(Debug)]
pub struct UseProgressBarProps {
    pub field_props: crate::hooks::UseLabelFieldProps,
    pub aria_describedby: Option<String>,
    pub role: AriaRole,
    pub aria_valuenow: Signal<Option<String>>,
    pub aria_valuemin: Signal<String>,
    pub aria_valuemax: Signal<String>,
    pub aria_valuetext: Signal<Option<String>>,
}

pub type UseProgressBarAttrs = (
    UseLabelFieldAttrs,
    Attr<attr::AriaDescribedby, Option<String>>,
    Attr<attr::Role, AriaRole>,
    Attr<attr::AriaValuenow, Signal<Option<String>>>,
    Attr<attr::AriaValuemin, Signal<String>>,
    Attr<attr::AriaValuemax, Signal<String>>,
    Attr<attr::AriaValuetext, Signal<Option<String>>>,
);

impl IntoAttrs for UseProgressBarProps {
    type Attrs = UseProgressBarAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            self.field_props.into_attrs(),
            Attr(attr::AriaDescribedby, self.aria_describedby),
            Attr(attr::Role, self.role),
            Attr(attr::AriaValuenow, self.aria_valuenow),
            Attr(attr::AriaValuemin, self.aria_valuemin),
            Attr(attr::AriaValuemax, self.aria_valuemax),
            Attr(attr::AriaValuetext, self.aria_valuetext),
        )
    }
}

/// Provides the accessibility of a progress bar: the progress of an operation over time, known
/// (determinate) or not (indeterminate).
///
/// ```ignore
/// let progress = use_progress_bar(UseProgressBarInput {
///     value: Signal::derive(move || Some(uploaded.get())),
///     max_value: Signal::stored(total),
///     has_label: true.into(),
///     ..UseProgressBarInput::default()
/// });
/// view! {
///     <div {..progress.props.into_attrs()}>
///         <span {..progress.label_props.into_attrs()}>"Uploading"</span>
///         <span>{progress.value_text}</span>
///     </div>
/// }
/// ```
pub fn use_progress_bar<T: NumberValue>(input: UseProgressBarInput<T>) -> UseProgressBarReturn {
    progress(input, AriaRole::Progressbar)
}

/// The progress bar with the given role (also the meter's).
pub(crate) fn progress<T: NumberValue>(
    input: UseProgressBarInput<T>,
    role: AriaRole,
) -> UseProgressBarReturn {
    let UseProgressBarInput {
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

    let label = use_label(UseLabelInput {
        id,
        has_label,
        label_element_type: LabelElementType::Span,
        aria_label,
        aria_labelledby,
        ..UseLabelInput::default()
    });

    let clamped = Signal::derive(move || {
        let (min, max) = (min_value.get(), max_value.get());
        value.get().map(|value| {
            if value < min {
                min
            } else if value > max {
                max
            } else {
                value
            }
        })
    });
    // As a fraction of the range; 0 for an empty range.
    let fraction = Signal::derive(move || {
        let (min, max) = (min_value.get().to_f64(), max_value.get().to_f64());
        clamped
            .get()
            .map(|value| percentage_in_range(min, max, value.to_f64()))
    });
    let formatter = use_number_formatter(format_options);
    let value_text = Signal::derive(move || {
        let value = clamped.get()?;
        if let Some(label) = value_label.get() {
            return Some(label);
        }
        Some(formatter.with(|formatter| {
            if formatter.options().style == NumberStyle::Percent {
                formatter.format(fraction.get().unwrap_or_default())
            } else {
                formatter.format(value)
            }
        }))
    });

    UseProgressBarReturn {
        props: UseProgressBarProps {
            field_props: label.field_props,
            aria_describedby,
            role,
            aria_valuenow: Signal::derive(move || clamped.get().map(|value| value.to_string())),
            aria_valuemin: Signal::derive(move || min_value.get().to_string()),
            aria_valuemax: Signal::derive(move || max_value.get().to_string()),
            aria_valuetext: value_text,
        },
        label_props: label.label_props,
        percentage: Signal::derive(move || fraction.get().map(|fraction| fraction * 100.0)),
        value_text,
    }
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;

    fn progress_of(input: UseProgressBarInput<f64>) -> UseProgressBarReturn {
        use_progress_bar(UseProgressBarInput {
            aria_label: "Loading".into(),
            ..input
        })
    }

    // Upstream: useProgressBar.test.js "with default props if no props are provided".
    #[test]
    fn defaults_to_zero_of_a_hundred() {
        Owner::new().with(|| {
            let progress = progress_of(UseProgressBarInput::default());
            assert_that!(progress.props.role).is_equal_to(AriaRole::Progressbar);
            assert_that!(progress.props.aria_valuenow.get_untracked())
                .is_equal_to(Some("0".to_owned()));
            assert_that!(progress.props.aria_valuemin.get_untracked()).is_equal_to("0".to_owned());
            assert_that!(progress.props.aria_valuemax.get_untracked())
                .is_equal_to("100".to_owned());
            assert_that!(progress.value_text.get_untracked()).is_equal_to(Some("0%".to_owned()));
        });
    }

    // Upstream: "with value of 25%" and RAC "supports a custom range".
    #[test]
    fn formats_the_percentage() {
        Owner::new().with(|| {
            let progress = progress_of(UseProgressBarInput {
                value: Signal::stored(Some(3.0)),
                max_value: Signal::stored(6.0),
                ..UseProgressBarInput::default()
            });
            assert_that!(progress.props.aria_valuenow.get_untracked())
                .is_equal_to(Some("3".to_owned()));
            assert_that!(progress.value_text.get_untracked()).is_equal_to(Some("50%".to_owned()));
            assert_that!(progress.percentage.get_untracked()).is_equal_to(Some(50.0));
        });
    }

    #[test]
    fn clamps_the_value_to_the_range() {
        Owner::new().with(|| {
            let progress = progress_of(UseProgressBarInput {
                value: Signal::stored(Some(150.0)),
                ..UseProgressBarInput::default()
            });
            assert_that!(progress.props.aria_valuenow.get_untracked())
                .is_equal_to(Some("100".to_owned()));
            assert_that!(progress.percentage.get_untracked()).is_equal_to(Some(100.0));
        });
    }

    // RAC: "renders 0 percent for an empty range (with a non-zero bound)".
    #[test]
    fn an_empty_range_is_zero_percent() {
        Owner::new().with(|| {
            let progress = progress_of(UseProgressBarInput {
                value: Signal::stored(Some(5.0)),
                min_value: Signal::stored(5.0),
                max_value: Signal::stored(5.0),
                ..UseProgressBarInput::default()
            });
            assert_that!(progress.value_text.get_untracked()).is_equal_to(Some("0%".to_owned()));
            assert_that!(progress.percentage.get_untracked()).is_equal_to(Some(0.0));
        });
    }

    // Upstream: "with indeterminate prop".
    #[test]
    fn indeterminate_has_no_value() {
        Owner::new().with(|| {
            let progress = progress_of(UseProgressBarInput {
                value: Signal::stored(None),
                ..UseProgressBarInput::default()
            });
            assert_that!(progress.props.aria_valuenow.get_untracked()).is_none();
            assert_that!(progress.props.aria_valuetext.get_untracked()).is_none();
            assert_that!(progress.props.aria_valuemin.get_untracked()).is_equal_to("0".to_owned());
            assert_that!(progress.percentage.get_untracked()).is_none();
        });
    }

    // Upstream: "with custom text value".
    #[test]
    fn a_value_label_replaces_the_text() {
        Owner::new().with(|| {
            let progress = progress_of(UseProgressBarInput {
                value: Signal::stored(Some(25.0)),
                value_label: "Loading 1 of 4".into(),
                ..UseProgressBarInput::default()
            });
            assert_that!(progress.props.aria_valuetext.get_untracked())
                .is_equal_to(Some("Loading 1 of 4".to_owned()));
        });
    }

    // Non-percent styles format the value itself.
    #[test]
    fn a_decimal_style_formats_the_value() {
        Owner::new().with(|| {
            let progress = use_progress_bar(UseProgressBarInput::<u32> {
                value: Signal::stored(Some(1500)),
                max_value: Signal::stored(4000),
                format_options: Signal::stored(NumberFormatOptions::default()),
                aria_label: "Downloaded".into(),
                ..UseProgressBarInput::default()
            });
            assert_that!(progress.value_text.get_untracked()).is_equal_to(Some("1,500".to_owned()));
        });
    }

    // Upstream: "supports labeling".
    #[test]
    fn a_visible_label_names_the_bar() {
        Owner::new().with(|| {
            let progress = use_progress_bar(UseProgressBarInput::<f64> {
                has_label: Signal::stored(true),
                ..UseProgressBarInput::default()
            });
            let label_id = progress.label_props.id.clone();
            assert_that!(progress.label_props.html_for).is_none();
            assert_that!(progress.props.field_props.aria_labelledby.get_untracked())
                .is_equal_to(Some(label_id));
        });
    }
}
