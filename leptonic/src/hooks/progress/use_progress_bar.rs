// Upstream: react-aria/src/progress/useProgressBar.ts @ 99e6102368
// Upstream: react-aria/test/progress/useProgressBar.test.js @ 99e6102368
// Upstream: react-aria-components/test/ProgressBar.test.js @ 99e6102368
use leptos::prelude::*;

use crate::{
    Id, IntoAttrs,
    hooks::form::{LabelElementType, UseLabelFieldAttrs, UseLabelInput, UseLabelProps, use_label},
    utils::{
        aria::AriaRole,
        fraction::Fraction,
        math::percentage_in_range,
        number_formatter::{NumberFormatOptions, NumberStyle, use_number_formatter},
        number_value::NumberValue,
        part::{Part, Remount},
        signal_attr::{FixedAttr, SignalAttr, fixed_attr, signal_attr},
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
// - `role: ProgressRole`: the element's role, a progress bar or a meter (`use_meter` sets
//   `Meter`). React-aria's `useMeter` replaces the role in `useProgressBar`'s returned props.
// - Also returns the `percentage` (a `Fraction`, C13) and the `value_text`, which react-aria-components
//   computes in its `ProgressBar` for render props.
//
// =============================================================================

/// The role of a [`use_progress_bar`] element.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ProgressRole {
    /// `role="progressbar"`: the progress of an operation.
    #[default]
    Progressbar,
    /// `role="meter"`: a value within a known range (see
    /// [`use_meter`](fn@crate::hooks::meter::use_meter), which sets it).
    Meter,
}

impl ProgressRole {
    fn aria_role(self) -> AriaRole {
        match self {
            Self::Progressbar => AriaRole::Progressbar,
            Self::Meter => AriaRole::Meter,
        }
    }
}

/// Input of [`use_progress_bar`] (and, with its own value type, [`use_meter`](fn@crate::hooks::meter::use_meter)).
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
    pub id: Option<Id>,
    /// Whether a visible label is rendered (with `label_props`).
    pub has_label: Signal<bool>,
    /// Names the progress bar when there is no visible label.
    pub aria_label: MaybeProp<String>,
    pub aria_labelledby: Signal<Option<Id>>,
    pub aria_describedby: Signal<Option<Id>>,
    /// The element's role. Default: `Progressbar`.
    pub role: ProgressRole,
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
            aria_labelledby: Signal::default(),
            aria_describedby: Signal::default(),
            role: ProgressRole::Progressbar,
        }
    }
}

/// The return value of [`use_progress_bar`].
#[derive(Debug)]
pub struct UseProgressBarReturn {
    /// The progress bar element.
    pub progress_bar: Part<UseProgressBarProps>,
    /// The label element.
    pub label: Part<UseLabelProps>,
    /// The progress as a share of the range (`.as_percent()`: 0 to 100); `None` while
    /// indeterminate.
    pub percentage: Signal<Option<Fraction>>,
    /// The formatted value (or `value_label`); `None` while indeterminate.
    pub value_text: Signal<Option<String>>,
}

/// Props for the progress bar (or meter) element.
#[derive(Debug)]
pub struct UseProgressBarProps {
    pub field_props: crate::hooks::form::UseLabelFieldProps,
    pub aria_describedby: Signal<Option<Id>>,
    pub role: AriaRole,
    pub aria_valuenow: Signal<Option<String>>,
    pub aria_valuemin: Signal<String>,
    pub aria_valuemax: Signal<String>,
    pub aria_valuetext: Signal<Option<String>>,
}

impl Remount for UseProgressBarProps {
    fn remount(&self) -> Self {
        Self {
            field_props: self.field_props.remount(),
            aria_describedby: self.aria_describedby,
            role: self.role,
            aria_valuenow: self.aria_valuenow,
            aria_valuemin: self.aria_valuemin,
            aria_valuemax: self.aria_valuemax,
            aria_valuetext: self.aria_valuetext,
        }
    }
}

pub type UseProgressBarAttrs = (
    UseLabelFieldAttrs,
    SignalAttr<Option<Id>>,
    FixedAttr,
    SignalAttr<Option<String>>,
    SignalAttr<String>,
    SignalAttr<String>,
    SignalAttr<Option<String>>,
);

impl IntoAttrs for UseProgressBarProps {
    type Attrs = UseProgressBarAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            self.field_props.into_attrs(),
            signal_attr("aria-describedby", self.aria_describedby),
            fixed_attr("role", self.role),
            signal_attr("aria-valuenow", self.aria_valuenow),
            signal_attr("aria-valuemin", self.aria_valuemin),
            signal_attr("aria-valuemax", self.aria_valuemax),
            signal_attr("aria-valuetext", self.aria_valuetext),
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
///     <div {..progress.progress_bar.attrs()}>
///         <span {..progress.label.attrs()}>"Uploading"</span>
///         <span>{progress.value_text}</span>
///     </div>
/// }
/// ```
pub fn use_progress_bar<T: NumberValue>(input: UseProgressBarInput<T>) -> UseProgressBarReturn {
    crate::metrics::subject!();
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
        role,
    } = input;
    let role = role.aria_role();

    let label = use_label(UseLabelInput {
        id,
        has_label,
        label_element_type: LabelElementType::Span,
        aria_label,
        aria_labelledby,
        ..UseLabelInput::default()
    });
    let label_field_props = label.field.into_props();
    let label_label_props = label.label.into_props();

    // Memos: each is read by several of the attributes and the returned signals.
    let clamped = Memo::new(move |_| {
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
    let fraction = Memo::new(move |_| {
        let (min, max) = (min_value.get().to_f64(), max_value.get().to_f64());
        clamped
            .get()
            .map(|value| percentage_in_range(min, max, value.to_f64()))
    });
    let formatter = use_number_formatter(format_options);
    let value_text = Memo::new(move |_| {
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
        progress_bar: Part::stored(UseProgressBarProps {
            field_props: label_field_props,
            aria_describedby,
            role,
            aria_valuenow: Signal::derive(move || clamped.get().map(|value| value.to_string())),
            aria_valuemin: Signal::derive(move || min_value.get().to_string()),
            aria_valuemax: Signal::derive(move || max_value.get().to_string()),
            aria_valuetext: value_text.into(),
        }),
        label: Part::stored(label_label_props),
        percentage: Signal::derive(move || fraction.get().map(Fraction::new)),
        value_text: value_text.into(),
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
        crate::testing::with_owner(|| {
            let progress = progress_of(UseProgressBarInput::default());
            let props = progress.progress_bar.into_props();
            assert_that!(props.role).is_equal_to(AriaRole::Progressbar);
            assert_that!(props.aria_valuenow.get_untracked()).is_equal_to(Some("0".to_owned()));
            assert_that!(props.aria_valuemin.get_untracked()).is_equal_to("0".to_owned());
            assert_that!(props.aria_valuemax.get_untracked()).is_equal_to("100".to_owned());
            assert_that!(progress.value_text.get_untracked()).is_equal_to(Some("0%".to_owned()));
        });
    }

    // Upstream: "with value of 25%" and "supports a custom range".
    #[test]
    fn formats_the_percentage() {
        crate::testing::with_owner(|| {
            let progress = progress_of(UseProgressBarInput {
                value: Signal::stored(Some(3.0)),
                max_value: Signal::stored(6.0),
                ..UseProgressBarInput::default()
            });
            let props = progress.progress_bar.into_props();
            assert_that!(props.aria_valuenow.get_untracked()).is_equal_to(Some("3".to_owned()));
            assert_that!(progress.value_text.get_untracked()).is_equal_to(Some("50%".to_owned()));
            assert_that!(
                progress
                    .percentage
                    .get_untracked()
                    .map(Fraction::as_percent)
            )
            .is_equal_to(Some(50.0));
        });
    }

    #[test]
    fn clamps_the_value_to_the_range() {
        crate::testing::with_owner(|| {
            let progress = progress_of(UseProgressBarInput {
                value: Signal::stored(Some(150.0)),
                ..UseProgressBarInput::default()
            });
            let props = progress.progress_bar.into_props();
            assert_that!(props.aria_valuenow.get_untracked()).is_equal_to(Some("100".to_owned()));
            assert_that!(
                progress
                    .percentage
                    .get_untracked()
                    .map(Fraction::as_percent)
            )
            .is_equal_to(Some(100.0));
        });
    }

    // Upstream: "renders 0 percent for an empty range with a non-zero bound".
    #[test]
    fn an_empty_range_is_zero_percent() {
        crate::testing::with_owner(|| {
            let progress = progress_of(UseProgressBarInput {
                value: Signal::stored(Some(5.0)),
                min_value: Signal::stored(5.0),
                max_value: Signal::stored(5.0),
                ..UseProgressBarInput::default()
            });
            assert_that!(progress.value_text.get_untracked()).is_equal_to(Some("0%".to_owned()));
            assert_that!(
                progress
                    .percentage
                    .get_untracked()
                    .map(Fraction::as_percent)
            )
            .is_equal_to(Some(0.0));
        });
    }

    // Upstream: "with indeterminate prop".
    #[test]
    fn indeterminate_has_no_value() {
        crate::testing::with_owner(|| {
            let progress = progress_of(UseProgressBarInput {
                value: Signal::stored(None),
                ..UseProgressBarInput::default()
            });
            let props = progress.progress_bar.into_props();
            assert_that!(props.aria_valuenow.get_untracked()).is_none();
            assert_that!(props.aria_valuetext.get_untracked()).is_none();
            assert_that!(props.aria_valuemin.get_untracked()).is_equal_to("0".to_owned());
            assert_that!(progress.percentage.get_untracked()).is_none();
        });
    }

    // Upstream: "with custom text value".
    #[test]
    fn a_value_label_replaces_the_text() {
        crate::testing::with_owner(|| {
            let progress = progress_of(UseProgressBarInput {
                value: Signal::stored(Some(25.0)),
                value_label: "Loading 1 of 4".into(),
                ..UseProgressBarInput::default()
            });
            let props = progress.progress_bar.into_props();
            assert_that!(props.aria_valuetext.get_untracked())
                .is_equal_to(Some("Loading 1 of 4".to_owned()));
        });
    }

    // Non-percent styles format the value itself.
    #[test]
    fn a_decimal_style_formats_the_value() {
        crate::testing::with_owner(|| {
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
        crate::testing::with_owner(|| {
            let progress = use_progress_bar(UseProgressBarInput::<f64> {
                has_label: Signal::stored(true),
                ..UseProgressBarInput::default()
            });
            let progress_label_props = progress.label.into_props();
            let progress_props = progress.progress_bar.into_props();
            let label_id = progress_label_props.id.clone();
            assert_that!(progress_label_props.html_for).is_none();
            assert_that!(progress_props.field_props.aria_labelledby.get_untracked())
                .is_equal_to(Some(label_id));
        });
    }
}
