// No upstream: marks along a slider's track (React Spectrum has none; react-aria leaves them to the
// application).
use leptos::prelude::*;

use crate::utils::fraction::Fraction;

/// Which marks a slider shows (named apart from the `SliderMarks` atom that renders them).
#[derive(Default, Debug, Clone)]
pub enum SliderMarkPlacement {
    /// No marks.
    #[default]
    None,
    /// A mark per step (at most about 20: every n-th step on long ranges).
    Automatic {
        /// Whether each mark is named by its formatted value.
        create_names: bool,
    },
    /// The given marks.
    Custom { marks: Vec<CustomSliderMark> },
}

/// A mark on a slider's track, placed by the app (named apart from the `SliderMark` atom that
/// renders a placed mark).
#[derive(Debug, Clone)]
pub struct CustomSliderMark {
    pub value: SliderMarkValue,
    /// A name shown next to the mark (C2: user-visible text, may change with the locale).
    pub name: MaybeProp<String>,
}

/// Where a mark is.
#[derive(Debug, Clone, Copy)]
pub enum SliderMarkValue {
    /// At a value of the slider's range.
    Value(f64),
    /// At a fraction of the track.
    Fraction(Fraction),
}

/// A mark placed on the track.
#[derive(Debug, Clone)]
pub struct ComputedSliderMark {
    /// Position along the track.
    pub percentage: Fraction,
    /// The value of the range at the mark.
    pub value: f64,
    pub name: Option<String>,
    /// The thumbs' values, for [`is_in_range`](Self::is_in_range).
    values: Signal<Vec<f64>>,
}

impl ComputedSliderMark {
    /// Whether the mark lies within the selected range: up to the thumb (one thumb), between the
    /// first and the last thumb (several). Tracked.
    #[must_use]
    pub fn is_in_range(&self) -> bool {
        let value = self.value;
        self.values.with(|values| match values.as_slice() {
            [] => false,
            [thumb] => value <= *thumb,
            [first, .., last] => *first <= value && value <= *last,
        })
    }
}

impl PartialEq for ComputedSliderMark {
    fn eq(&self, other: &Self) -> bool {
        self.percentage.get().to_bits() == other.percentage.get().to_bits()
            && self.value.to_bits() == other.value.to_bits()
            && self.name == other.name
    }
}

/// Input of [`use_slider_marks`].
#[derive(Debug, Clone)]
pub struct UseSliderMarksInput {
    pub min_value: Signal<f64>,
    pub max_value: Signal<f64>,
    pub step: Signal<f64>,
    /// The thumbs' values.
    pub values: Signal<Vec<f64>>,
    pub marks: SliderMarkPlacement,
    /// Formats a value for automatic names.
    pub format: Callback<f64, String>,
}

/// The most automatic marks (roughly): long ranges mark every n-th step.
const MAX_AUTOMATIC_MARKS: f64 = 20.0;

/// Where `value` is in the range `min..=max` (unclamped: a value outside lies outside 0..=1).
fn percent(min: f64, max: f64, value: f64) -> f64 {
    if max == min {
        0.0
    } else {
        (value - min) / (max - min)
    }
}

/// The marks of a slider, placed on its track. They are recomputed when the range, the step or
/// a name changes; [`ComputedSliderMark::is_in_range`] follows the thumbs.
pub fn use_slider_marks(input: UseSliderMarksInput) -> Signal<Vec<ComputedSliderMark>> {
    let UseSliderMarksInput {
        min_value,
        max_value,
        step,
        values,
        marks,
        format,
    } = input;

    let marks = StoredValue::new(marks);
    // Plain data: allocates no signals per run (a mark's range check reads `values`).
    Memo::new(move |_| {
        let (min, max, step) = (min_value.get(), max_value.get(), step.get());
        marks.with_value(|marks| match marks {
            SliderMarkPlacement::None => Vec::new(),
            SliderMarkPlacement::Automatic { create_names } => {
                if step <= 0.0 || max <= min {
                    return Vec::new();
                }
                let steps = ((max - min) / step).round();
                let every = (steps / MAX_AUTOMATIC_MARKS).round().max(1.0);
                // `steps / every` is small and non-negative (at most about 40).
                #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                let count = (steps / every).floor() as usize;
                (0..=count)
                    .map(|n| {
                        #[allow(clippy::cast_precision_loss)]
                        let value = (n as f64 * every).mul_add(step, min).min(max);
                        ComputedSliderMark {
                            percentage: Fraction::new(percent(min, max, value)),
                            value,
                            name: create_names.then(|| format.run(value)),
                            values,
                        }
                    })
                    .collect()
            }
            SliderMarkPlacement::Custom { marks } => marks
                .iter()
                .filter_map(|mark| {
                    let percentage = match mark.value {
                        SliderMarkValue::Value(value) => {
                            let percentage = percent(min, max, value);
                            if !(0.0..=1.0).contains(&percentage) {
                                crate::utils::dev_warn!(
                                    "A slider mark lies outside the slider's range: {mark:?}"
                                );
                                return None;
                            }
                            Fraction::new(percentage)
                        }
                        SliderMarkValue::Fraction(fraction) => fraction,
                    };
                    let value = percentage.get().mul_add(max - min, min);
                    Some(ComputedSliderMark {
                        percentage,
                        value,
                        name: mark.name.get(),
                        values,
                    })
                })
                .collect(),
        })
    })
    .into()
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;
    use crate::testing::with_owner;

    fn marks(
        marks: SliderMarkPlacement,
        values: RwSignal<Vec<f64>>,
        step: f64,
    ) -> Signal<Vec<ComputedSliderMark>> {
        use_slider_marks(UseSliderMarksInput {
            min_value: Signal::stored(0.0),
            max_value: Signal::stored(10.0),
            step: Signal::stored(step),
            values: values.into(),
            marks,
            format: Callback::new(|value: f64| format!("{value}")),
        })
    }

    #[test]
    fn automatic_marks_per_step_with_names() {
        with_owner(|| {
            let values = RwSignal::new(vec![5.0]);
            let marks = marks(
                SliderMarkPlacement::Automatic { create_names: true },
                values,
                2.5,
            )
            .get_untracked();
            let percentages: Vec<f64> = marks.iter().map(|m| m.percentage.get()).collect();
            assert_that!(percentages).is_equal_to(vec![0.0, 0.25, 0.5, 0.75, 1.0]);
            assert_that!(marks[1].name.as_deref()).is_equal_to(Some("2.5"));
            assert_that!(marks[1].value).is_equal_to(2.5);
        });
    }

    #[test]
    fn long_ranges_mark_every_nth_step() {
        with_owner(|| {
            let values = RwSignal::new(vec![0.0]);
            let marks = marks(
                SliderMarkPlacement::Automatic {
                    create_names: false,
                },
                values,
                0.1,
            )
            .get_untracked();
            // 100 steps: every 5th.
            assert_that!(marks.len()).is_equal_to(21);
        });
    }

    #[test]
    fn in_range_follows_the_thumbs() {
        with_owner(|| {
            let values = RwSignal::new(vec![5.0]);
            let marks = marks(
                SliderMarkPlacement::Automatic {
                    create_names: false,
                },
                values,
                2.5,
            )
            .get_untracked();
            let in_range = || {
                marks
                    .iter()
                    .map(|m| untrack(|| m.is_in_range()))
                    .collect::<Vec<_>>()
            };
            assert_that!(in_range()).is_equal_to(vec![true, true, true, false, false]);
            values.set(vec![2.5, 7.5]);
            assert_that!(in_range()).is_equal_to(vec![false, true, true, true, false]);
        });
    }

    #[test]
    fn custom_marks_outside_the_range_are_dropped() {
        with_owner(|| {
            let values = RwSignal::new(vec![0.0]);
            let custom = SliderMarkPlacement::Custom {
                marks: vec![
                    CustomSliderMark {
                        value: SliderMarkValue::Value(5.0),
                        name: "Half".into(),
                    },
                    CustomSliderMark {
                        value: SliderMarkValue::Fraction(Fraction::new(0.1)),
                        name: MaybeProp::default(),
                    },
                    CustomSliderMark {
                        value: SliderMarkValue::Value(11.0),
                        name: MaybeProp::default(),
                    },
                ],
            };
            let marks = marks(custom, values, 1.0).get_untracked();
            assert_that!(marks.len()).is_equal_to(2);
            assert_that!(marks[0].percentage).is_equal_to(Fraction::new(0.5));
            assert_that!(marks[1].percentage).is_equal_to(Fraction::new(0.1));
            assert_that!(marks[0].name.as_deref()).is_equal_to(Some("Half"));
        });
    }
}
