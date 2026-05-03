use std::borrow::Cow;

use leptos::prelude::*;

use crate::{
    hooks::slider::use_slider_state::UseSliderStateReturn, utils::math::percentage_in_range,
};

// Note: This hooks is not available in react-aria.

//
// No intentional deviations from the react-aria implementation.
//

/// Configuration for slider marks.
#[derive(Default, Debug, Clone)]
#[allow(variant_size_differences)]
pub enum SliderMarks {
    /// No marks.
    #[default]
    None,

    /// Automatically generate marks from the step value.
    /// Note that marks can only be automatically generated if a step value is provided
    /// (the slider is not "contiguous").
    Automatic {
        /// Whether to create name labels for each mark.
        create_names: bool,
    },

    /// Manually specified marks.
    Custom {
        /// The list of custom marks.
        marks: Vec<SliderMark>,
    },
}

/// A single slider mark.
#[derive(Debug, Clone)]
pub struct SliderMark {
    /// The value marked by this mark. Can be an absolute value in the sliders value-range or a
    /// percentage.
    pub value: SliderMarkValue,

    /// An optional name/label for the mark.
    pub name: Option<Cow<'static, str>>,
}

/// How a mark's position is specified.
#[derive(Debug, Clone, Copy)]
pub enum SliderMarkValue {
    /// An absolute value within the slider range.
    Value(f64),

    /// A percentage (0.0-1.0) along the track.
    Percentage(f64),
}

/// A computed mark with reactive in-range state.
#[derive(Clone)]
pub struct ComputedSliderMark {
    /// Position along the track (0.0-1.0).
    pub percentage: f64,

    /// Whether this mark is within the selected range (reactive).
    pub in_range: Signal<bool>,

    /// Optional display name.
    pub name: Option<Cow<'static, str>>,
}

/// Input for the `use_slider_marks` hook.
pub struct UseSliderMarksInput {
    /// The slider state.
    pub state: UseSliderStateReturn,

    /// The marks configuration.
    pub marks: SliderMarks,

    /// Optional callback to format values for display.
    pub value_display: Option<Callback<f64, String>>,
}

/// Return value of the `use_slider_marks` hook.
pub struct UseSliderMarksReturn {
    /// The computed marks (reactive).
    pub marks: Signal<Vec<ComputedSliderMark>>,
}

/// Computes slider marks with reactive in-range state.
#[allow(clippy::too_many_lines)]
pub fn use_slider_marks(input: UseSliderMarksInput) -> UseSliderMarksReturn {
    let UseSliderMarksInput {
        state,
        marks,
        value_display,
    } = input;

    let min = state.min_value;
    let max = state.max_value;
    let step = state.step;
    let values = state.values;
    let num_thumbs = state.num_thumbs;

    // Helper to create an in_range signal for a given value.
    let make_in_range = move |v: f64| -> Signal<bool> {
        match num_thumbs {
            1 => {
                // Single thumb: mark is in range if value <= thumb value (or >= for reversed axis).
                let first = values.get().first().copied().unwrap_or(min);
                if max > min {
                    Signal::derive(move || v <= first)
                } else {
                    Signal::derive(move || v >= first)
                }
            }
            2 => {
                // Two thumbs: mark is in range if between the two thumb values.
                let vals = values.get();
                let a = vals.first().copied().unwrap_or(min);
                let b = vals.get(1).copied().unwrap_or(max);

                if max > min {
                    Signal::derive(move || v >= a && v <= b)
                } else {
                    Signal::derive(move || v <= a && v >= b)
                }
            }
            _ => Signal::derive(|| false),
        }
    };

    let marks = match marks {
        SliderMarks::None => Signal::derive(Vec::new),
        SliderMarks::Automatic { create_names } => Signal::derive(move || match step {
            Some(step) => {
                let range = (max - min).abs();
                let mut computed = Vec::new();
                let cap = 20.0;
                let estimate = range / step;
                let step_multiplier = f64::max(1.0, f64::round(estimate / cap));
                let mut current = min;

                let rounding_error_offset = 0.000_001;
                loop {
                    if max > min {
                        if current > max + rounding_error_offset {
                            break;
                        }
                    } else if current < max - rounding_error_offset {
                        break;
                    }
                    computed.push(ComputedSliderMark {
                        percentage: percentage_in_range(min, max, current),
                        in_range: make_in_range(current),
                        name: if create_names {
                            Some(Cow::Owned(match &value_display {
                                Some(callback) => callback.run(current),
                                None => format!("{current}"),
                            }))
                        } else {
                            None
                        },
                    });
                    if max > min {
                        if current <= max + rounding_error_offset {
                            current += step * step_multiplier;
                        }
                    } else if current >= max - rounding_error_offset {
                        current -= step * step_multiplier;
                    }
                }
                computed
            }
            None => Vec::new(),
        }),
        SliderMarks::Custom { marks } => Signal::derive(move || {
            marks
                .iter()
                .filter(|mark| match mark.value {
                    SliderMarkValue::Value(value) => {
                        let lower = f64::min(min, max);
                        let upper = f64::max(min, max);
                        if value < lower || value > upper {
                            tracing::warn!(
                                ?mark,
                                min,
                                max,
                                "value of custom slider mark is outside slider range"
                            );
                            false
                        } else {
                            true
                        }
                    }
                    SliderMarkValue::Percentage(percentage) => {
                        if (0.0..=1.0).contains(&percentage) {
                            true
                        } else {
                            tracing::warn!(
                                ?mark,
                                "percentage of custom slider mark is outside 0..1 range"
                            );
                            false
                        }
                    }
                })
                .map(|mark| {
                    let value = match mark.value {
                        SliderMarkValue::Value(value) => value,
                        SliderMarkValue::Percentage(percentage) => {
                            crate::utils::math::value_in_range(min, max, percentage)
                        }
                    };
                    ComputedSliderMark {
                        percentage: match mark.value {
                            SliderMarkValue::Value(value) => percentage_in_range(min, max, value),
                            SliderMarkValue::Percentage(percentage) => percentage,
                        },
                        in_range: make_in_range(value),
                        name: mark.name.clone(),
                    }
                })
                .collect()
        }),
    };

    UseSliderMarksReturn { marks }
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;
    use reactive_graph::owner::Owner;

    use super::*;
    use crate::hooks::{
        SliderOrientation,
        slider::use_slider_state::{
            SliderValues, UseSliderStateInput, UseSliderStateReturn, use_slider_state,
        },
    };

    /// Helper: create slider state within the current reactive owner.
    fn make_state(
        default_values: Vec<f64>,
        min: f64,
        max: f64,
        step: Option<f64>,
    ) -> UseSliderStateReturn {
        use_slider_state(UseSliderStateInput {
            values: SliderValues::Uncontrolled(default_values),
            min_value: min,
            max_value: max,
            step,
            disabled: false.into(),
            orientation: SliderOrientation::default().into(),
            on_change: None,
            on_change_end: None,
        })
    }

    #[test]
    fn none_produces_empty_marks() {
        let owner = Owner::new();
        owner.with(|| {
            let state = make_state(vec![50.0], 0.0, 100.0, Some(10.0));
            let result = use_slider_marks(UseSliderMarksInput {
                state,
                marks: SliderMarks::None,
                value_display: None,
            });
            assert_that(result.marks.get().len()).is_equal_to(0);
        });
    }

    #[test]
    fn automatic_generates_marks_at_each_step() {
        let owner = Owner::new();
        owner.with(|| {
            let state = make_state(vec![50.0], 0.0, 100.0, Some(10.0));
            let result = use_slider_marks(UseSliderMarksInput {
                state,
                marks: SliderMarks::Automatic {
                    create_names: false,
                },
                value_display: None,
            });
            let marks = result.marks.get();
            // 0, 10, 20, ..., 100 = 11 marks
            assert_that(marks.len()).is_equal_to(11);
            // First mark at 0%
            assert_that(marks[0].percentage).is_close_to(0.0, 0.001);
            // Last mark at 100%
            assert_that(marks[10].percentage).is_close_to(1.0, 0.001);
        });
    }

    #[test]
    fn automatic_creates_names_when_requested() {
        let owner = Owner::new();
        owner.with(|| {
            let state = make_state(vec![5.0], 0.0, 10.0, Some(5.0));
            let result = use_slider_marks(UseSliderMarksInput {
                state,
                marks: SliderMarks::Automatic { create_names: true },
                value_display: None,
            });
            let marks = result.marks.get();
            // 0, 5, 10 = 3 marks
            assert_that(marks.len()).is_equal_to(3);
            assert_that(marks[0].name.as_deref()).is_equal_to(Some("0"));
            assert_that(marks[1].name.as_deref()).is_equal_to(Some("5"));
            assert_that(marks[2].name.as_deref()).is_equal_to(Some("10"));
        });
    }

    #[test]
    fn automatic_omits_names_when_not_requested() {
        let owner = Owner::new();
        owner.with(|| {
            let state = make_state(vec![5.0], 0.0, 10.0, Some(5.0));
            let result = use_slider_marks(UseSliderMarksInput {
                state,
                marks: SliderMarks::Automatic {
                    create_names: false,
                },
                value_display: None,
            });
            let marks = result.marks.get();
            assert_that(marks.len()).is_equal_to(3);
            for mark in &marks {
                assert_that(mark.name.is_none()).is_true();
            }
        });
    }

    #[test]
    fn automatic_caps_at_roughly_20_marks() {
        let owner = Owner::new();
        owner.with(|| {
            // 0..100 step 1 = 101 possible marks, should be capped
            let state = make_state(vec![50.0], 0.0, 100.0, Some(1.0));
            let result = use_slider_marks(UseSliderMarksInput {
                state,
                marks: SliderMarks::Automatic {
                    create_names: false,
                },
                value_display: None,
            });
            let marks = result.marks.get();
            assert_that(marks.len()).is_less_or_equal_to(21);
        });
    }

    #[test]
    fn automatic_returns_empty_for_continuous_mode() {
        let owner = Owner::new();
        owner.with(|| {
            let state = make_state(vec![50.0], 0.0, 100.0, None);
            let result = use_slider_marks(UseSliderMarksInput {
                state,
                marks: SliderMarks::Automatic { create_names: true },
                value_display: None,
            });
            let marks = result.marks.get();
            assert_that(marks.len()).is_equal_to(0);
        });
    }

    #[test]
    fn automatic_handles_reversed_range() {
        let owner = Owner::new();
        owner.with(|| {
            // Reversed: min=10, max=0
            let state = make_state(vec![5.0], 10.0, 0.0, Some(2.0));
            let result = use_slider_marks(UseSliderMarksInput {
                state,
                marks: SliderMarks::Automatic { create_names: true },
                value_display: None,
            });
            let marks = result.marks.get();
            // 10, 8, 6, 4, 2, 0 = 6 marks
            assert_that(marks.len()).is_equal_to(6);
            // First mark at percentage 0 (which is value=min=10)
            assert_that(marks[0].percentage).is_close_to(0.0, 0.001);
            // Last mark at percentage 1 (which is value=max=0)
            assert_that(marks[5].percentage).is_close_to(1.0, 0.001);
        });
    }

    #[test]
    fn automatic_uses_value_display_callback() {
        let owner = Owner::new();
        owner.with(|| {
            let state = make_state(vec![0.0], 0.0, 10.0, Some(5.0));
            let result = use_slider_marks(UseSliderMarksInput {
                state,
                marks: SliderMarks::Automatic { create_names: true },
                value_display: Some(Callback::new(|v: f64| format!("{v:.1}!"))),
            });
            let marks = result.marks.get();
            assert_that(marks[0].name.as_deref()).is_equal_to(Some("0.0!"));
            assert_that(marks[1].name.as_deref()).is_equal_to(Some("5.0!"));
            assert_that(marks[2].name.as_deref()).is_equal_to(Some("10.0!"));
        });
    }

    #[test]
    fn custom_marks_at_values() {
        let owner = Owner::new();
        owner.with(|| {
            let state = make_state(vec![50.0], 0.0, 100.0, Some(1.0));
            let result = use_slider_marks(UseSliderMarksInput {
                state,
                marks: SliderMarks::Custom {
                    marks: vec![
                        SliderMark {
                            value: SliderMarkValue::Value(25.0),
                            name: Some("25".into()),
                        },
                        SliderMark {
                            value: SliderMarkValue::Value(75.0),
                            name: Some("75".into()),
                        },
                    ],
                },
                value_display: None,
            });
            let marks = result.marks.get();
            assert_that(marks.len()).is_equal_to(2);
            assert_that(marks[0].percentage).is_close_to(0.25, 0.001);
            assert_that(marks[0].name.as_deref()).is_equal_to(Some("25"));
            assert_that(marks[1].percentage).is_close_to(0.75, 0.001);
            assert_that(marks[1].name.as_deref()).is_equal_to(Some("75"));
        });
    }

    #[test]
    fn custom_marks_at_percentages() {
        let owner = Owner::new();
        owner.with(|| {
            let state = make_state(vec![50.0], 0.0, 200.0, Some(1.0));
            let result = use_slider_marks(UseSliderMarksInput {
                state,
                marks: SliderMarks::Custom {
                    marks: vec![SliderMark {
                        value: SliderMarkValue::Percentage(0.3),
                        name: Some("30%".into()),
                    }],
                },
                value_display: None,
            });
            let marks = result.marks.get();
            assert_that(marks.len()).is_equal_to(1);
            assert_that(marks[0].percentage).is_close_to(0.3, 0.001);
        });
    }

    #[test]
    fn custom_filters_out_of_range_values() {
        let owner = Owner::new();
        owner.with(|| {
            let state = make_state(vec![50.0], 0.0, 100.0, Some(1.0));
            let result = use_slider_marks(UseSliderMarksInput {
                state,
                marks: SliderMarks::Custom {
                    marks: vec![
                        SliderMark {
                            value: SliderMarkValue::Value(50.0),
                            name: Some("valid".into()),
                        },
                        SliderMark {
                            value: SliderMarkValue::Value(150.0),
                            name: Some("out of range".into()),
                        },
                        SliderMark {
                            value: SliderMarkValue::Value(-10.0),
                            name: Some("below min".into()),
                        },
                    ],
                },
                value_display: None,
            });
            let marks = result.marks.get();
            assert_that(marks.len()).is_equal_to(1);
            assert_that(marks[0].name.as_deref()).is_equal_to(Some("valid"));
        });
    }

    #[test]
    fn custom_filters_out_of_range_percentages() {
        let owner = Owner::new();
        owner.with(|| {
            let state = make_state(vec![50.0], 0.0, 100.0, Some(1.0));
            let result = use_slider_marks(UseSliderMarksInput {
                state,
                marks: SliderMarks::Custom {
                    marks: vec![
                        SliderMark {
                            value: SliderMarkValue::Percentage(0.5),
                            name: Some("valid".into()),
                        },
                        SliderMark {
                            value: SliderMarkValue::Percentage(1.5),
                            name: Some("too high".into()),
                        },
                        SliderMark {
                            value: SliderMarkValue::Percentage(-0.1),
                            name: Some("too low".into()),
                        },
                    ],
                },
                value_display: None,
            });
            let marks = result.marks.get();
            assert_that(marks.len()).is_equal_to(1);
            assert_that(marks[0].name.as_deref()).is_equal_to(Some("valid"));
        });
    }

    #[test]
    fn single_thumb_in_range() {
        let owner = Owner::new();
        owner.with(|| {
            let state = make_state(vec![50.0], 0.0, 100.0, Some(10.0));
            let result = use_slider_marks(UseSliderMarksInput {
                state,
                marks: SliderMarks::Custom {
                    marks: vec![
                        SliderMark {
                            value: SliderMarkValue::Value(30.0),
                            name: None,
                        },
                        SliderMark {
                            value: SliderMarkValue::Value(70.0),
                            name: None,
                        },
                    ],
                },
                value_display: None,
            });
            let marks = result.marks.get();
            // Thumb at 50: mark at 30 is in range, mark at 70 is not
            assert_that(marks[0].in_range.get()).is_true();
            assert_that(marks[1].in_range.get()).is_false();
        });
    }

    #[test]
    fn dual_thumb_in_range() {
        let owner = Owner::new();
        owner.with(|| {
            let state = make_state(vec![20.0, 80.0], 0.0, 100.0, Some(10.0));
            let result = use_slider_marks(UseSliderMarksInput {
                state,
                marks: SliderMarks::Custom {
                    marks: vec![
                        SliderMark {
                            value: SliderMarkValue::Value(10.0),
                            name: None,
                        },
                        SliderMark {
                            value: SliderMarkValue::Value(50.0),
                            name: None,
                        },
                        SliderMark {
                            value: SliderMarkValue::Value(90.0),
                            name: None,
                        },
                    ],
                },
                value_display: None,
            });
            let marks = result.marks.get();
            // Thumbs at 20 and 80: mark at 10 is NOT in range, 50 IS, 90 is NOT
            assert_that(marks[0].in_range.get()).is_false();
            assert_that(marks[1].in_range.get()).is_true();
            assert_that(marks[2].in_range.get()).is_false();
        });
    }

    #[test]
    fn reversed_range_automatic_mark_positions() {
        let owner = Owner::new();
        owner.with(|| {
            // Reversed: min=10, max=0, step=2
            // Automatic marks should go: 10, 8, 6, 4, 2, 0
            let state = make_state(vec![10.0], 10.0, 0.0, Some(2.0));
            let result = use_slider_marks(UseSliderMarksInput {
                state,
                marks: SliderMarks::Automatic { create_names: true },
                value_display: None,
            });
            let marks = result.marks.get();
            // 10, 8, 6, 4, 2, 0 = 6 marks
            assert_that(marks.len()).is_equal_to(6);
            // First mark at percentage 0 (which is value=min=10)
            assert_that(marks[0].percentage).is_close_to(0.0, 0.001);
            assert_that(marks[0].name.as_deref()).is_equal_to(Some("10"));
            // Last mark at percentage 1 (which is value=max=0)
            assert_that(marks[5].percentage).is_close_to(1.0, 0.001);
            assert_that(marks[5].name.as_deref()).is_equal_to(Some("0"));
            // Middle mark at value=6, percentage = (6-10)/(0-10) = -4/-10 = 0.4
            assert_that(marks[2].percentage).is_close_to(0.4, 0.001);
        });
    }

    #[test]
    fn custom_marks_accepted_in_reversed_range() {
        let owner = Owner::new();
        owner.with(|| {
            // Reversed: min=100, max=0. Values 70 and 30 are within [0, 100].
            let state = make_state(vec![100.0], 100.0, 0.0, Some(10.0));
            let result = use_slider_marks(UseSliderMarksInput {
                state,
                marks: SliderMarks::Custom {
                    marks: vec![
                        SliderMark {
                            value: SliderMarkValue::Value(70.0),
                            name: Some("70".into()),
                        },
                        SliderMark {
                            value: SliderMarkValue::Value(30.0),
                            name: Some("30".into()),
                        },
                        SliderMark {
                            value: SliderMarkValue::Value(150.0),
                            name: Some("out of range".into()),
                        },
                    ],
                },
                value_display: None,
            });
            let marks = result.marks.get();
            // 70 and 30 are within [0, 100], 150 is not
            assert_that(marks.len()).is_equal_to(2);
            assert_that(marks[0].name.as_deref()).is_equal_to(Some("70"));
            assert_that(marks[1].name.as_deref()).is_equal_to(Some("30"));
        });
    }

    #[test]
    fn reversed_range_custom_marks_in_range() {
        let owner = Owner::new();
        owner.with(|| {
            // Reversed: min=100, max=0, thumb at 100 (min value).
            // In reversed range, in_range means v >= thumb_value.
            let state = make_state(vec![60.0], 100.0, 0.0, Some(10.0));
            let result = use_slider_marks(UseSliderMarksInput {
                state,
                marks: SliderMarks::Custom {
                    marks: vec![
                        SliderMark {
                            value: SliderMarkValue::Value(80.0),
                            name: None,
                        },
                        SliderMark {
                            value: SliderMarkValue::Value(40.0),
                            name: None,
                        },
                    ],
                },
                value_display: None,
            });
            let marks = result.marks.get();
            assert_that(marks.len()).is_equal_to(2);
            // Thumb at 60. Reversed: in_range = v >= thumb.
            // 80 >= 60: in range
            assert_that(marks[0].in_range.get()).is_true();
            // 40 >= 60: NOT in range
            assert_that(marks[1].in_range.get()).is_false();
        });
    }

    #[test]
    fn three_or_more_thumbs_always_false() {
        let owner = Owner::new();
        owner.with(|| {
            let state = make_state(vec![20.0, 50.0, 80.0], 0.0, 100.0, Some(10.0));
            let result = use_slider_marks(UseSliderMarksInput {
                state,
                marks: SliderMarks::Custom {
                    marks: vec![SliderMark {
                        value: SliderMarkValue::Value(50.0),
                        name: None,
                    }],
                },
                value_display: None,
            });
            let marks = result.marks.get();
            // 3+ thumbs: in_range is always false
            assert_that(marks[0].in_range.get()).is_false();
        });
    }
}
