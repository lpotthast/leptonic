// No upstream: marks along a slider's track (React Spectrum has none; react-aria leaves them to the
// application).
use std::borrow::Cow;

use leptos::prelude::*;

/// Which marks a slider shows.
#[derive(Default, Debug, Clone)]
pub enum SliderMarks {
    /// No marks.
    #[default]
    None,
    /// A mark per step (at most about 20: every n-th step on long ranges).
    Automatic {
        /// Whether each mark is named by its formatted value.
        create_names: bool,
    },
    /// The given marks.
    Custom { marks: Vec<SliderMark> },
}

/// A mark on a slider's track.
#[derive(Debug, Clone)]
pub struct SliderMark {
    pub value: SliderMarkValue,
    /// A name shown next to the mark.
    pub name: Option<Cow<'static, str>>,
}

/// Where a mark is.
#[derive(Debug, Clone, Copy)]
pub enum SliderMarkValue {
    /// At a value of the slider's range.
    Value(f64),
    /// At a fraction (0.0 to 1.0) of the track.
    Percentage(f64),
}

/// A mark placed on the track.
#[derive(Debug, Clone)]
pub struct ComputedSliderMark {
    /// Position along the track, 0.0 to 1.0.
    pub percentage: f64,
    /// Whether the mark lies within the selected range: up to the thumb (one thumb), between the
    /// first and the last thumb (several).
    pub in_range: Signal<bool>,
    pub name: Option<Cow<'static, str>>,
}

/// Input of [`use_slider_marks`].
#[derive(Debug, Clone)]
pub struct UseSliderMarksInput {
    pub min_value: Signal<f64>,
    pub max_value: Signal<f64>,
    pub step: Signal<f64>,
    /// The thumbs' values.
    pub values: Signal<Vec<f64>>,
    pub marks: SliderMarks,
    /// Formats a value for automatic names.
    pub format: Callback<f64, String>,
}

/// The most automatic marks (roughly): long ranges mark every n-th step.
const MAX_AUTOMATIC_MARKS: f64 = 20.0;

fn percent(min: f64, max: f64, value: f64) -> f64 {
    if max == min {
        0.0
    } else {
        (value - min) / (max - min)
    }
}

/// The marks of a slider, placed on its track.
pub fn use_slider_marks(input: UseSliderMarksInput) -> Signal<Vec<ComputedSliderMark>> {
    let UseSliderMarksInput {
        min_value,
        max_value,
        step,
        values,
        marks,
        format,
    } = input;

    let in_range = move |value: f64| {
        Signal::derive(move || {
            values.with(|values| match values.as_slice() {
                [] => false,
                [thumb] => value <= *thumb,
                [first, .., last] => *first <= value && value <= *last,
            })
        })
    };

    let marks = StoredValue::new(marks);
    Signal::derive(move || {
        let (min, max, step) = (min_value.get(), max_value.get(), step.get());
        marks.with_value(|marks| match marks {
            SliderMarks::None => Vec::new(),
            SliderMarks::Automatic { create_names } => {
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
                            percentage: percent(min, max, value),
                            in_range: in_range(value),
                            name: create_names.then(|| Cow::Owned(format.run(value))),
                        }
                    })
                    .collect()
            }
            SliderMarks::Custom { marks } => marks
                .iter()
                .filter_map(|mark| {
                    let (value, percentage) = match mark.value {
                        SliderMarkValue::Value(value) => (value, percent(min, max, value)),
                        SliderMarkValue::Percentage(percentage) => {
                            (percentage.mul_add(max - min, min), percentage)
                        }
                    };
                    if !(0.0..=1.0).contains(&percentage) {
                        crate::utils::dev_warn!(
                            "A slider mark lies outside the slider's range: {mark:?}"
                        );
                        return None;
                    }
                    Some(ComputedSliderMark {
                        percentage,
                        in_range: in_range(value),
                        name: mark.name.clone(),
                    })
                })
                .collect(),
        })
    })
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;

    fn marks(
        marks: SliderMarks,
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
        Owner::new().with(|| {
            let values = RwSignal::new(vec![5.0]);
            let marks =
                marks(SliderMarks::Automatic { create_names: true }, values, 2.5).get_untracked();
            let percentages: Vec<f64> = marks.iter().map(|m| m.percentage).collect();
            assert_that!(percentages).is_equal_to(vec![0.0, 0.25, 0.5, 0.75, 1.0]);
            assert_that!(marks[1].name.as_deref()).is_equal_to(Some("2.5"));
        });
    }

    #[test]
    fn long_ranges_mark_every_nth_step() {
        Owner::new().with(|| {
            let values = RwSignal::new(vec![0.0]);
            let marks = marks(
                SliderMarks::Automatic {
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
        Owner::new().with(|| {
            let values = RwSignal::new(vec![5.0]);
            let marks = marks(
                SliderMarks::Automatic {
                    create_names: false,
                },
                values,
                2.5,
            )
            .get_untracked();
            let in_range = || {
                marks
                    .iter()
                    .map(|m| m.in_range.get_untracked())
                    .collect::<Vec<_>>()
            };
            assert_that!(in_range()).is_equal_to(vec![true, true, true, false, false]);
            values.set(vec![2.5, 7.5]);
            assert_that!(in_range()).is_equal_to(vec![false, true, true, true, false]);
        });
    }

    #[test]
    fn custom_marks_outside_the_range_are_dropped() {
        Owner::new().with(|| {
            let values = RwSignal::new(vec![0.0]);
            let custom = SliderMarks::Custom {
                marks: vec![
                    SliderMark {
                        value: SliderMarkValue::Value(5.0),
                        name: Some("Half".into()),
                    },
                    SliderMark {
                        value: SliderMarkValue::Percentage(0.1),
                        name: None,
                    },
                    SliderMark {
                        value: SliderMarkValue::Value(11.0),
                        name: None,
                    },
                ],
            };
            let marks = marks(custom, values, 1.0).get_untracked();
            assert_that!(marks.len()).is_equal_to(2);
            assert_that!(marks[0].percentage).is_equal_to(0.5);
            assert_that!(marks[1].percentage).is_equal_to(0.1);
        });
    }
}
