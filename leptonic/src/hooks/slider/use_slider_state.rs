// Upstream: react-stately/src/slider/useSliderState.ts @ 99e6102368
// Upstream: react-stately/test/slider/useSliderState.test.js @ 99e6102368
// Upstream: @adobe/react-spectrum/test/slider/Slider.test.tsx @ 99e6102368
use leptos::prelude::*;

use crate::{
    ValueBinding,
    utils::{
        fraction::Fraction,
        i18n::{Locale, use_locale},
        list_formatter::{ListFormatOptions, ListFormatType, ListFormatter},
        number_formatter::{NumberFormatOptions, NumberFormatter, use_number_formatter},
        number_value::NumberValue,
        orientation::Orientation,
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Generic over the value type (`NumberValue`: integers or floats), as the number field (C15).
// - Hook-owned values (C4): `default_values` + `on_change`, or `value` bound to app state.
// - `min_value`, `max_value` and `step` are signals; the number format is `NumberFormatOptions`
//   with the locale from the i18n context.
// - A `Copy` struct with methods (C3); thumbs are addressed by `usize` index; positions in the
//   range are `Fraction`s (C13; react-aria: percentages as plain numbers).
//
// ## DIFFERENT BEHAVIOR
// - Two values are formatted "a – b" (react-aria: `Intl.NumberFormat.formatRange`, which ICU4X
//   lacks); more as a list of units, as react-aria.
// - A thumb index without a value reads `min_value` and ignores changes (react-aria: `undefined`
//   and `NaN`), with a development warning from `use_slider_thumb`.
//
// =============================================================================

/// Input of [`use_slider_state`].
#[derive(Debug, Clone)]
pub struct UseSliderStateInput<T: NumberValue> {
    /// The initial values, one per thumb, in ascending order. Default: one thumb at
    /// `min_value`.
    pub default_values: Option<Vec<T>>,
    /// The values as app state, replacing `default_values`.
    pub value: Option<ValueBinding<Vec<T>>>,
    pub min_value: Signal<T>,
    pub max_value: Signal<T>,
    /// The step between values. Default: 1.
    pub step: Signal<T>,
    pub is_disabled: Signal<bool>,
    pub orientation: Signal<Orientation>,
    /// How values are formatted for assistive technology and outputs.
    pub format_options: Signal<NumberFormatOptions>,
    /// Formats a thumb's value for assistive technology instead of `format_options` (e.g. a
    /// color channel's value).
    pub value_label: Option<Callback<T, String>>,
    /// The step of PageUp/PageDown. Default: a tenth of the range (a multiple of the step).
    pub page_size: Option<Signal<T>>,
    /// Called with the values whenever they change, also while dragging.
    pub on_change: Option<Callback<Vec<T>>>,
    /// Called with the values when the user stops dragging (or after a keyboard change).
    pub on_change_end: Option<Callback<Vec<T>>>,
}

/// The state of a slider with one or more thumbs.
#[derive(Debug, Clone, Copy)]
pub struct SliderState<T: NumberValue> {
    /// The thumbs' values, in ascending order.
    pub values: Signal<Vec<T>>,
    /// The smallest value of the range.
    pub min_value: Signal<T>,
    /// The largest value of the range.
    pub max_value: Signal<T>,
    /// The step between values.
    pub step: Signal<T>,
    /// Whether the slider is disabled.
    pub is_disabled: Signal<bool>,
    /// The axis of the track.
    pub orientation: Signal<Orientation>,
    /// The thumb with focus.
    pub focused_thumb: Signal<Option<usize>>,
    binding: ValueBinding<Vec<T>>,
    default_values: StoredValue<Vec<T>>,
    /// The values as last set: dragging sets several times before the binding has updated.
    latest: StoredValue<Vec<T>>,
    dragging: RwSignal<Vec<bool>>,
    set_focused_thumb: WriteSignal<Option<usize>>,
    editable: StoredValue<Vec<bool>>,
    formatting: Memo<NumberFormatter>,
    locale: Signal<Locale>,
    value_label: Option<Callback<T, String>>,
    page_size_override: Option<Signal<T>>,
    on_change_end: Option<Callback<Vec<T>>>,
}

#[derive(Debug, Clone, Copy)]
enum StepDirection {
    Up,
    Down,
}

impl<T: NumberValue> SliderState<T> {
    /// The number of thumbs.
    pub fn thumb_count(&self) -> usize {
        self.values.with(Vec::len)
    }

    /// The value of thumb `index` (tracked). A thumb without a value (an index beyond the
    /// values) is at `min_value`.
    pub fn thumb_value(&self, index: usize) -> T {
        self.values
            .with(|values| values.get(index).copied())
            .unwrap_or_else(|| self.min_value.get())
    }

    /// The values the slider started with (for form resets).
    pub fn default_values(&self) -> Vec<T> {
        self.default_values.get_value()
    }

    /// The smallest value thumb `index` can take: the previous thumb's value or `min_value`.
    pub fn thumb_min_value(&self, index: usize) -> T {
        self.values
            .with(|values| {
                // A missing thumb uses the slider minimum as its fallback value. Applying a
                // preceding thumb's bound would make that fallback invalid for the native input.
                values.get(index)?;
                index
                    .checked_sub(1)
                    .and_then(|previous| values.get(previous).copied())
            })
            .unwrap_or_else(|| self.min_value.get())
    }

    /// The largest value thumb `index` can take: the next thumb's value or `max_value`.
    pub fn thumb_max_value(&self, index: usize) -> T {
        self.values
            .with(|values| values.get(index + 1).copied())
            .unwrap_or_else(|| self.max_value.get())
    }

    /// Sets thumb `index` to `value`, snapped to the step and kept between its neighbors.
    /// Ignored while the slider is disabled, the thumb isn't editable, or has no value.
    pub fn set_thumb_value(&self, index: usize, value: T) {
        if self.is_disabled.get_untracked()
            || !self.is_thumb_editable(index)
            || index >= self.values.with_untracked(Vec::len)
        {
            return;
        }
        let (min, max) = untrack(|| (self.thumb_min_value(index), self.thumb_max_value(index)));
        let value = value.snap_to_step(Some(min), Some(max), self.step.get_untracked());
        // While dragging, the binding may not have caught up with the last set values yet;
        // otherwise it holds the current values (also after changes by the app).
        let mut values = if self
            .dragging
            .with_untracked(|d| d.iter().any(|dragging| *dragging))
        {
            self.latest.get_value()
        } else {
            self.values.get_untracked()
        };
        match values.get_mut(index) {
            Some(current) if *current != value => *current = value,
            _ => return,
        }
        self.latest.set_value(values.clone());
        self.binding.set(values);
    }

    /// Sets thumb `index` to the value at `percent` of the range.
    pub fn set_thumb_percent(&self, index: usize, percent: Fraction) {
        if let Some(value) = untrack(|| self.percent_value(percent)) {
            self.set_thumb_value(index, value);
        }
    }

    /// Where thumb `index` is in the range (tracked).
    pub fn thumb_percent(&self, index: usize) -> Fraction {
        self.value_percent(self.thumb_value(index))
    }

    /// Where `value` is in the range (tracked; values outside it at its ends).
    pub fn value_percent(&self, value: T) -> Fraction {
        let min = self.min_value.get().to_f64();
        let max = self.max_value.get().to_f64();
        if max == min {
            return Fraction::ZERO;
        }
        Fraction::new((value.to_f64() - min) / (max - min))
    }

    /// The value at `percent` of the range, rounded to the step then clamped to the bounds (tracked).
    pub fn percent_value(&self, percent: Fraction) -> Option<T> {
        let min = self.min_value.get();
        let max = self.max_value.get();
        let step = self.step.get();
        let (min_f, max_f, step_f) = (min.to_f64(), max.to_f64(), step.to_f64());
        let value = percent.get().mul_add(max_f - min_f, min_f);
        let rounded = if step_f > 0.0 {
            ((value - min_f) / step_f).round().mul_add(step_f, min_f)
        } else {
            value
        };
        T::from_f64(rounded.clamp(min_f.min(max_f), max_f.max(min_f)))
    }

    /// Whether thumb `index` is being dragged (tracked).
    pub fn is_thumb_dragging(&self, index: usize) -> bool {
        self.dragging
            .with(|dragging| dragging.get(index).copied().unwrap_or(false))
    }

    /// Starts or ends dragging thumb `index`. When the last thumb stops being dragged,
    /// `on_change_end` is called.
    pub fn set_thumb_dragging(&self, index: usize, dragging: bool) {
        if self.is_disabled.get_untracked() || !self.is_thumb_editable(index) {
            return;
        }
        if dragging {
            self.latest.set_value(self.values.get_untracked());
        }
        let was_dragging = self
            .dragging
            .with_untracked(|d| d.get(index).copied().unwrap_or(false));
        self.dragging.update(|d| {
            if d.len() <= index {
                d.resize(index + 1, false);
            }
            d[index] = dragging;
        });
        if was_dragging
            && !self
                .dragging
                .with_untracked(|d| d.iter().any(|dragging| *dragging))
            && let Some(on_change_end) = self.on_change_end
        {
            on_change_end.run(self.latest.get_value());
        }
    }

    pub fn set_focused_thumb(&self, index: Option<usize>) {
        self.set_focused_thumb.set(index);
    }

    /// Whether thumb `index` can be changed.
    pub fn is_thumb_editable(&self, index: usize) -> bool {
        self.editable
            .with_value(|editable| editable.get(index).copied().unwrap_or(true))
    }

    /// Makes thumb `index` (not) changeable (a disabled thumb).
    pub fn set_thumb_editable(&self, index: usize, editable: bool) {
        self.editable.update_value(|e| {
            if e.len() <= index {
                e.resize(index + 1, true);
            }
            e[index] = editable;
        });
    }

    /// Increases thumb `index` by `step_size` (at least the step).
    pub fn increment_thumb(&self, index: usize, step_size: Option<T>) {
        self.step_thumb(index, step_size, StepDirection::Up);
    }

    /// Decreases thumb `index` by `step_size` (at least the step).
    pub fn decrement_thumb(&self, index: usize, step_size: Option<T>) {
        self.step_thumb(index, step_size, StepDirection::Down);
    }

    fn step_thumb(&self, index: usize, step_size: Option<T>, direction: StepDirection) {
        let (step, min, max, value) = untrack(|| {
            (
                self.step.get(),
                self.min_value.get(),
                self.max_value.get(),
                self.thumb_value(index),
            )
        });
        let size = step_size.map_or(step, |size| {
            if size.to_f64() > step.to_f64() {
                size
            } else {
                step
            }
        });
        // Beyond the type's bounds: the range's end.
        let target = match direction {
            StepDirection::Up => value.checked_add(size).unwrap_or(max),
            StepDirection::Down => value.checked_sub(size).unwrap_or(min),
        };
        self.set_thumb_value(index, target.snap_to_step(Some(min), Some(max), step));
    }

    /// The step of PageUp/PageDown: a tenth of the range, a multiple of the step, at least the
    /// step (tracked).
    pub fn page_size(&self) -> T {
        if let Some(page_size) = self.page_size_override {
            return page_size.get();
        }
        let step = self.step.get();
        let range = self.max_value.get().to_f64() - self.min_value.get().to_f64();
        let step_f = step.to_f64();
        let page = range / 10.0;
        let page = if step_f > 0.0 {
            (page / step_f).round() * step_f
        } else {
            page
        };
        T::from_f64(page.max(step_f)).map_or(step, |page| page.snap_to_step(Some(step), None, step))
    }

    /// The formatted value of thumb `index`, for `aria-valuetext` (tracked).
    pub fn thumb_value_label(&self, index: usize) -> String {
        let value = self.thumb_value(index);
        match self.value_label {
            Some(value_label) => value_label.run(value),
            None => self.format_value(value),
        }
    }

    /// `value` formatted with the slider's format options (tracked).
    pub fn format_value(&self, value: T) -> String {
        self.formatting.with(|f| f.format(value))
    }

    /// All values formatted, for an output: one value, "a – b", or a list (tracked).
    pub fn formatted_values(&self) -> String {
        self.values.with(|values| {
            self.formatting.with(|number| match values.as_slice() {
                [] => String::new(),
                [value] => number.format(*value),
                [start, end] => {
                    format!("{} \u{2013} {}", number.format(*start), number.format(*end))
                }
                values => {
                    let formatted: Vec<String> = values.iter().map(|v| number.format(*v)).collect();
                    let parts: Vec<&str> = formatted.iter().map(String::as_str).collect();
                    // Rare (three or more thumbs): created when needed, it isn't `Send`.
                    let list = ListFormatter::new(
                        &self.locale.get(),
                        &ListFormatOptions {
                            kind: ListFormatType::Unit,
                            ..ListFormatOptions::default()
                        },
                    );
                    list.format(&parts)
                }
            })
        })
    }
}

/// Each value snapped to the step and kept between its neighbors (react-stately's
/// `restrictValues`: the neighbors' values as given).
fn restrict_values<T: NumberValue>(values: &[T], min: T, max: T, step: T) -> Vec<T> {
    values
        .iter()
        .enumerate()
        .map(|(index, value)| {
            let lower = index
                .checked_sub(1)
                .and_then(|previous| values.get(previous).copied())
                .unwrap_or(min);
            let upper = values.get(index + 1).copied().unwrap_or(max);
            value.snap_to_step(Some(lower), Some(upper), step)
        })
        .collect()
}

/// Manages the values of a slider with one or more thumbs.
pub fn use_slider_state<T: NumberValue>(input: UseSliderStateInput<T>) -> SliderState<T> {
    let UseSliderStateInput {
        default_values,
        value,
        min_value,
        max_value,
        step,
        is_disabled,
        orientation,
        format_options,
        value_label,
        page_size,
        on_change,
        on_change_end,
    } = input;

    // The values a form reset restores: `default_values`, else the initial bound values (as
    // upstream's `initialValues`), else the minimum.
    let defaults = untrack(|| {
        restrict_values(
            &match (default_values, &value) {
                (Some(default_values), _) => default_values,
                (None, Some(value)) => value.value.get(),
                (None, None) => vec![min_value.get()],
            },
            min_value.get(),
            max_value.get(),
            step.get(),
        )
    });
    let is_bound = value.is_some();
    let binding = value.unwrap_or_else(|| ValueBinding::from(RwSignal::new(defaults.clone())));
    // Bound values are restricted on every change (react-stately restricts the controlled
    // `value`): values out of the range or out of order render within it. Own values are
    // restricted once (the defaults) and stay restricted.
    let bound = binding.value;
    let values = Memo::new(move |_| {
        if is_bound {
            bound.with(|values| {
                restrict_values(values, min_value.get(), max_value.get(), step.get())
            })
        } else {
            bound.get()
        }
    });
    let initial = values.get_untracked();
    let thumbs = initial.len();
    let latest = StoredValue::new(initial);
    let binding = ValueBinding::new(
        values.into(),
        Callback::new(move |values: Vec<T>| {
            binding.set(values.clone());
            if let Some(on_change) = on_change {
                on_change.run(values);
            }
        }),
    );

    let locale = use_locale();
    let formatting = use_number_formatter(format_options);

    let (focused_thumb, set_focused_thumb) = signal(None);
    SliderState {
        values: binding.value,
        min_value,
        max_value,
        step,
        is_disabled,
        orientation,
        focused_thumb: focused_thumb.into(),
        binding,
        default_values: StoredValue::new(defaults),
        latest,
        dragging: RwSignal::new(vec![false; thumbs]),
        set_focused_thumb,
        editable: StoredValue::new(vec![true; thumbs]),
        formatting,
        locale,
        value_label,
        page_size_override: page_size,
        on_change_end,
    }
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;
    use crate::{testing::with_owner, utils::number_formatter::NumberStyle};

    fn state<T: NumberValue>(input: UseSliderStateInput<T>) -> SliderState<T> {
        use_slider_state(input)
    }

    /// A horizontal slider from `min` to `max` in steps of `step`, one thumb at `min`.
    fn input<T: NumberValue>(min: T, max: T, step: T) -> UseSliderStateInput<T> {
        UseSliderStateInput {
            default_values: None,
            value: None,
            min_value: Signal::stored(min),
            max_value: Signal::stored(max),
            step: Signal::stored(step),
            is_disabled: Signal::default(),
            orientation: Signal::stored(Orientation::Horizontal),
            format_options: Signal::default(),
            value_label: None,
            page_size: None,
            on_change: None,
            on_change_end: None,
        }
    }

    /// A callback recording the values it is called with.
    fn recorder<T: NumberValue>() -> (RwSignal<Vec<Vec<T>>>, Callback<Vec<T>>) {
        let calls = RwSignal::new(Vec::new());
        (
            calls,
            Callback::new(move |values| calls.update(|c| c.push(values))),
        )
    }

    /// Upstream: "should allow setting and reading values, percentages, and labels".
    #[test]
    fn values_percentages_and_labels() {
        with_owner(|| {
            let slider = state(UseSliderStateInput {
                default_values: Some(vec![50]),
                format_options: Signal::stored(NumberFormatOptions {
                    style: NumberStyle::Currency,
                    currency: Some("USD".to_owned()),
                    ..NumberFormatOptions::default()
                }),
                ..input(10, 200, 10)
            });
            let percent = |n: f64| Fraction::new(n / 190.0);
            assert_that!(slider.thumb_value(0)).is_equal_to(50);
            assert_that!(slider.thumb_percent(0)).is_equal_to(percent(40.0));
            assert_that!(slider.value_percent(50)).is_equal_to(percent(40.0));
            assert_that!(slider.thumb_value_label(0)).is_equal_to("$50.00".to_owned());
            slider.set_thumb_value(0, 100);
            assert_that!(slider.thumb_percent(0)).is_equal_to(percent(90.0));
            assert_that!(slider.thumb_value_label(0)).is_equal_to("$100.00".to_owned());
            slider.set_thumb_value(0, 500);
            assert_that!(slider.thumb_value(0)).is_equal_to(200);
            assert_that!(slider.thumb_percent(0)).is_equal_to(Fraction::ONE);
            assert_that!(slider.thumb_value_label(0)).is_equal_to("$200.00".to_owned());
            slider.set_thumb_value(0, 0);
            assert_that!(slider.thumb_value(0)).is_equal_to(10);
            assert_that!(slider.thumb_percent(0)).is_equal_to(Fraction::ZERO);
            assert_that!(slider.thumb_value_label(0)).is_equal_to("$10.00".to_owned());
            slider.set_thumb_value(0, 7);
            assert_that!(slider.thumb_value(0)).is_equal_to(10);
            slider.set_thumb_percent(0, Fraction::new(0.13));
            assert_that!(slider.thumb_value(0)).is_equal_to(30);
            assert_that!(slider.thumb_percent(0)).is_equal_to(percent(20.0));
        });
    }

    /// Upstream: "should enforce maxValue and minValue for multiple thumbs".
    #[test]
    fn thumbs_are_bounded_by_their_neighbors() {
        with_owner(|| {
            let slider = state(UseSliderStateInput {
                default_values: Some(vec![50, 70, 90]),
                ..input(10, 200, 1)
            });
            let bounds = |index| (slider.thumb_min_value(index), slider.thumb_max_value(index));
            assert_that!(bounds(0)).is_equal_to((10, 70));
            assert_that!(bounds(1)).is_equal_to((50, 90));
            assert_that!(bounds(2)).is_equal_to((70, 200));
            slider.set_thumb_value(1, 80);
            assert_that!(slider.values.get_untracked()).is_equal_to(vec![50, 80, 90]);
            assert_that!(slider.thumb_min_value(2)).is_equal_to(80);
            slider.set_thumb_value(1, 100);
            assert_that!(slider.values.get_untracked()).is_equal_to(vec![50, 90, 90]);
            assert_that!(slider.thumb_min_value(2)).is_equal_to(90);
        });
    }

    /// Upstream: "should round values to nearest step with two thumbs".
    #[test]
    fn two_thumbs_round_to_the_step_from_their_neighbors() {
        with_owner(|| {
            let slider = state(UseSliderStateInput {
                default_values: Some(vec![1.0, 13.0]),
                ..input(1.0, 15.0, 2.5)
            });
            assert_that!(slider.values.get_untracked()).is_equal_to(vec![1.0, 13.5]);
            assert_that!(slider.thumb_min_value(1)).is_equal_to(1.0);
            slider.set_thumb_value(0, 3.0);
            assert_that!(slider.values.get_untracked()).is_equal_to(vec![3.5, 13.5]);
            assert_that!(slider.thumb_min_value(1)).is_equal_to(3.5);
            assert_that!(slider.thumb_max_value(0)).is_equal_to(13.5);
            slider.set_thumb_value(1, 5.0);
            assert_that!(slider.values.get_untracked()).is_equal_to(vec![3.5, 6.0]);
            assert_that!(slider.thumb_max_value(0)).is_equal_to(6.0);
        });
    }

    /// Upstream: "should round values to nearest step with three thumbs".
    #[test]
    fn three_thumbs_round_to_the_step_from_their_neighbors() {
        with_owner(|| {
            let slider = state(UseSliderStateInput {
                default_values: Some(vec![1.0, 6.0, 13.0]),
                ..input(1.0, 15.0, 2.5)
            });
            assert_that!(slider.values.get_untracked()).is_equal_to(vec![1.0, 6.0, 13.5]);
            assert_that!(slider.thumb_min_value(1)).is_equal_to(1.0);
            assert_that!(slider.thumb_min_value(2)).is_equal_to(6.0);
            slider.set_thumb_value(0, 3.0);
            assert_that!(slider.values.get_untracked()).is_equal_to(vec![3.5, 6.0, 13.5]);
            assert_that!(slider.thumb_min_value(2)).is_equal_to(6.0);
            assert_that!(slider.thumb_min_value(1)).is_equal_to(3.5);
            assert_that!(slider.thumb_max_value(0)).is_equal_to(6.0);
            assert_that!(slider.thumb_max_value(1)).is_equal_to(13.5);
            slider.set_thumb_value(2, 5.0);
            assert_that!(slider.values.get_untracked()).is_equal_to(vec![3.5, 6.0, 6.0]);
            assert_that!(slider.thumb_max_value(0)).is_equal_to(6.0);
            assert_that!(slider.thumb_max_value(1)).is_equal_to(6.0);
        });
    }

    /// Upstream: "should call onChange and onChangeEnd appropriately": every change of a drag is
    /// reported, its end once, with the last values.
    #[test]
    fn on_change_per_change_and_on_change_end_per_drag() {
        with_owner(|| {
            let (changes, on_change) = recorder();
            let (ends, on_change_end) = recorder();
            let slider = state(UseSliderStateInput {
                on_change: Some(on_change),
                on_change_end: Some(on_change_end),
                ..input(0, 100, 1)
            });
            assert_that!(slider.values.get_untracked()).is_equal_to(vec![0]);
            slider.set_thumb_dragging(0, true);
            slider.set_thumb_value(0, 50);
            slider.set_thumb_dragging(0, false);
            assert_that!(changes.get_untracked()).is_equal_to(vec![vec![50]]);
            assert_that!(ends.get_untracked()).is_equal_to(vec![vec![50]]);

            slider.set_thumb_dragging(0, true);
            assert_that!(slider.is_thumb_dragging(0)).is_true();
            slider.set_thumb_value(0, 55);
            slider.set_thumb_value(0, 60);
            assert_that!(ends.get_untracked()).has_length(1);
            slider.set_thumb_value(0, 65);
            slider.set_thumb_dragging(0, false);
            assert_that!(changes.get_untracked().last()).is_equal_to(Some(&vec![65]));
            assert_that!(changes.get_untracked()).has_length(4);
            assert_that!(ends.get_untracked()).is_equal_to(vec![vec![50], vec![65]]);
        });
    }

    /// Upstream: "should not call onChange and onChangeEnd if not being moved".
    #[test]
    fn setting_the_same_value_reports_nothing() {
        with_owner(|| {
            let (changes, on_change) = recorder();
            let (ends, on_change_end) = recorder();
            let slider = state(UseSliderStateInput {
                on_change: Some(on_change),
                on_change_end: Some(on_change_end),
                ..input(0, 100, 1)
            });
            slider.set_thumb_value(0, 0);
            assert_that!(changes.get_untracked()).is_empty();
            assert_that!(ends.get_untracked()).is_empty();
        });
    }

    /// react-spectrum's `Slider.test.tsx` "sets page size to a multiple of step": a tenth of the
    /// range, rounded to the step, at least the step.
    #[test]
    fn the_page_size_is_a_multiple_of_the_step() {
        with_owner(|| {
            let page_size =
                |min: i32, max: i32, step: i32| state(input(min, max, step)).page_size();
            assert_that!(page_size(0, 100, 20)).is_equal_to(20);
            assert_that!(page_size(0, 230, 10)).is_equal_to(20);
            assert_that!(page_size(50, 75, 2)).is_equal_to(2);
            assert_that!(page_size(-50, -15, 2)).is_equal_to(4);
            let slider = state(UseSliderStateInput {
                default_values: Some(vec![60]),
                ..input(50, 75, 2)
            });
            slider.increment_thumb(0, Some(slider.page_size()));
            assert_that!(slider.thumb_value(0)).is_equal_to(62);
        });
    }

    /// react-spectrum's `Slider.test.tsx` "clamps value & defaultValue to the allowed range":
    /// own and bound values are kept in the range, on the step.
    #[test]
    fn values_are_clamped_to_the_range_on_the_step() {
        with_owner(|| {
            let clamped = |min: i32, max: i32, step: i32| {
                let own = state(UseSliderStateInput {
                    default_values: Some(vec![20]),
                    ..input(min, max, step)
                });
                let bound = state(UseSliderStateInput {
                    value: Some(ValueBinding::from(RwSignal::new(vec![20]))),
                    ..input(min, max, step)
                });
                (own.values.get_untracked(), bound.values.get_untracked())
            };
            assert_that!(clamped(50, 100, 1)).is_equal_to((vec![50], vec![50]));
            assert_that!(clamped(0, 10, 1)).is_equal_to((vec![10], vec![10]));
            assert_that!(clamped(50, 100, 3)).is_equal_to((vec![50], vec![50]));
            assert_that!(clamped(0, 10, 3)).is_equal_to((vec![9], vec![9]));
        });
    }

    #[test]
    fn snaps_and_clamps_values() {
        with_owner(|| {
            let slider = state(UseSliderStateInput {
                step: Signal::stored(5),
                default_values: Some(vec![12, 90]),
                value: None,
                min_value: Signal::stored(0),
                max_value: Signal::stored(100),
                is_disabled: Signal::default(),
                orientation: Signal::stored(Orientation::Horizontal),
                format_options: Signal::default(),
                value_label: None,
                page_size: None,
                on_change: None,
                on_change_end: None,
            });
            // Each value snaps relative to its neighbor's (react-stately's `restrictValues`): 90
            // above 12 in steps of 5 is 92.
            assert_that!(slider.values.get_untracked()).is_equal_to(vec![10, 92]);
            // A thumb stays between its neighbors.
            slider.set_thumb_value(0, 95);
            assert_that!(slider.values.get_untracked()).is_equal_to(vec![90, 92]);
            slider.set_thumb_value(1, 200);
            assert_that!(slider.values.get_untracked()).is_equal_to(vec![90, 100]);
        });
    }

    #[test]
    fn a_bound_slider_resets_to_its_initial_values_and_follows_the_app() {
        with_owner(|| {
            let app = RwSignal::new(vec![20, 80]);
            let slider = state(UseSliderStateInput {
                value: Some(ValueBinding::from(app)),
                default_values: None,
                min_value: Signal::stored(0),
                max_value: Signal::stored(100),
                step: Signal::stored(1),
                is_disabled: Signal::default(),
                orientation: Signal::stored(Orientation::Horizontal),
                format_options: Signal::default(),
                value_label: None,
                page_size: None,
                on_change: None,
                on_change_end: None,
            });
            assert_that!(slider.default_values()).is_equal_to(vec![20, 80]);
            // A step on one thumb keeps the other thumb's value as the app set it.
            app.set(vec![30, 60]);
            slider.set_thumb_value(0, 40);
            assert_that!(app.get_untracked()).is_equal_to(vec![40, 60]);
        });
    }

    #[test]
    fn steps_by_at_least_the_step_and_pages_by_a_tenth() {
        with_owner(|| {
            let slider = state(UseSliderStateInput {
                step: Signal::stored(2.0),
                default_values: None,
                value: None,
                min_value: Signal::stored(0.0),
                max_value: Signal::stored(50.0),
                is_disabled: Signal::default(),
                orientation: Signal::stored(Orientation::Horizontal),
                format_options: Signal::default(),
                value_label: None,
                page_size: None,
                on_change: None,
                on_change_end: None,
            });
            assert_that!(slider.page_size()).is_equal_to(6.0);
            slider.increment_thumb(0, None);
            assert_that!(slider.thumb_value(0)).is_equal_to(2.0);
            slider.increment_thumb(0, Some(1.0));
            assert_that!(slider.thumb_value(0)).is_equal_to(4.0);
            slider.decrement_thumb(0, Some(slider.page_size()));
            assert_that!(slider.thumb_value(0)).is_equal_to(0.0);
        });
    }

    #[test]
    fn percent_conversions_round_to_the_step() {
        with_owner(|| {
            let slider = state(UseSliderStateInput {
                step: Signal::stored(10u8),
                default_values: None,
                value: None,
                min_value: Signal::stored(0),
                max_value: Signal::stored(200),
                is_disabled: Signal::default(),
                orientation: Signal::stored(Orientation::Horizontal),
                format_options: Signal::default(),
                value_label: None,
                page_size: None,
                on_change: None,
                on_change_end: None,
            });
            assert_that!(slider.percent_value(Fraction::new(0.26))).is_equal_to(Some(50));
            slider.set_thumb_percent(0, Fraction::new(0.5));
            assert_that!(slider.thumb_percent(0)).is_equal_to(Fraction::new(0.5));
        });
    }

    #[test]
    fn percent_conversions_clamp_integer_values_after_rounding() {
        with_owner(|| {
            let slider = state(input(0, 230, 20));
            assert_that!(slider.percent_value(Fraction::ONE)).is_equal_to(Some(230));

            // Updating a thumb still snaps to a valid step, as upstream's updateValue does.
            slider.set_thumb_percent(0, Fraction::ONE);
            assert_that!(slider.thumb_value(0)).is_equal_to(220);

            let slider = state(input(0, 229, 20));
            assert_that!(slider.percent_value(Fraction::ONE)).is_equal_to(Some(220));
        });
    }

    #[test]
    fn percent_conversions_clamp_fractional_values_after_rounding() {
        with_owner(|| {
            let slider = state(input(0.0, 5.75, 0.5));
            assert_that!(slider.percent_value(Fraction::ONE)).is_equal_to(Some(5.75));

            slider.set_thumb_percent(0, Fraction::ONE);
            assert_that!(slider.thumb_value(0)).is_equal_to(5.5);

            let slider = state(input(0.0, 5.625, 0.5));
            assert_that!(slider.percent_value(Fraction::ONE)).is_equal_to(Some(5.5));
        });
    }

    #[test]
    fn on_change_end_after_the_last_drag() {
        with_owner(|| {
            let ended = RwSignal::new(None::<Vec<i32>>);
            let slider = state(UseSliderStateInput {
                default_values: Some(vec![10, 20]),
                on_change_end: Some(Callback::new(move |values| ended.set(Some(values)))),
                value: None,
                min_value: Signal::stored(0),
                max_value: Signal::stored(100),
                step: Signal::stored(1),
                is_disabled: Signal::default(),
                orientation: Signal::stored(Orientation::Horizontal),
                format_options: Signal::default(),
                value_label: None,
                page_size: None,
                on_change: None,
            });
            slider.set_thumb_dragging(0, true);
            slider.set_thumb_value(0, 15);
            assert_that!(ended.get_untracked()).is_none();
            slider.set_thumb_dragging(0, false);
            assert_that!(ended.get_untracked()).is_equal_to(Some(vec![15, 20]));
        });
    }

    #[test]
    fn disabled_and_non_editable_thumbs_ignore_changes() {
        with_owner(|| {
            let disabled = RwSignal::new(false);
            let slider = state(UseSliderStateInput {
                is_disabled: disabled.into(),
                default_values: None,
                value: None,
                min_value: Signal::stored(0),
                max_value: Signal::stored(10),
                step: Signal::stored(1),
                orientation: Signal::stored(Orientation::Horizontal),
                format_options: Signal::default(),
                value_label: None,
                page_size: None,
                on_change: None,
                on_change_end: None,
            });
            slider.set_thumb_editable(0, false);
            slider.set_thumb_value(0, 5);
            assert_that!(slider.thumb_value(0)).is_equal_to(0);
            slider.set_thumb_editable(0, true);
            disabled.set(true);
            slider.set_thumb_value(0, 5);
            assert_that!(slider.thumb_value(0)).is_equal_to(0);
        });
    }

    #[test]
    fn bound_values_are_restricted_to_the_range_and_order() {
        with_owner(|| {
            let app = RwSignal::new(vec![-20, 150]);
            let slider = state(UseSliderStateInput {
                value: Some(ValueBinding::from(app)),
                default_values: None,
                min_value: Signal::stored(0),
                max_value: Signal::stored(100),
                step: Signal::stored(1),
                is_disabled: Signal::default(),
                orientation: Signal::stored(Orientation::Horizontal),
                format_options: Signal::default(),
                value_label: None,
                page_size: None,
                on_change: None,
                on_change_end: None,
            });
            assert_that!(slider.values.get_untracked()).is_equal_to(vec![0, 100]);
            // Out of order: each value stays between its neighbors.
            app.set(vec![80, 20]);
            assert_that!(slider.values.get_untracked()).is_equal_to(vec![20, 80]);
            assert_that!(slider.default_values()).is_equal_to(vec![0, 100]);
        });
    }

    #[test]
    fn a_thumb_without_a_value_does_not_panic() {
        with_owner(|| {
            let slider = state(UseSliderStateInput {
                default_values: Some(vec![30, 60]),
                value: None,
                min_value: Signal::stored(0),
                max_value: Signal::stored(100),
                step: Signal::stored(1),
                is_disabled: Signal::default(),
                orientation: Signal::stored(Orientation::Horizontal),
                format_options: Signal::default(),
                value_label: None,
                page_size: None,
                on_change: None,
                on_change_end: None,
            });
            // The first missing thumb still has a preceding thumb, whose value must not
            // become its minimum: its native range input falls back to the slider minimum.
            assert_that!(slider.thumb_value(2)).is_equal_to(0);
            assert_that!(slider.thumb_min_value(2)).is_equal_to(0);
            assert_that!(slider.thumb_max_value(2)).is_equal_to(100);
            slider.set_thumb_value(2, 80);
            assert_that!(slider.thumb_value(5)).is_equal_to(0);
            assert_that!(slider.thumb_min_value(5)).is_equal_to(0);
            assert_that!(slider.thumb_max_value(5)).is_equal_to(100);
            slider.set_thumb_value(5, 50);
            slider.increment_thumb(5, None);
            assert_that!(slider.values.get_untracked()).is_equal_to(vec![30, 60]);
            assert_that!(slider.thumb_value_label(5)).is_equal_to("0".to_owned());
        });
    }
}
