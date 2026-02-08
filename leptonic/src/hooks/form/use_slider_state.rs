use crate::utils::math::{
    calculate_page_size, decimal_precision, percentage_in_range, snap_value_to_step,
};
use leptos::prelude::*;

/// The orientation of a slider.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SliderOrientation {
    /// Horizontal layout.
    #[default]
    Horizontal,

    /// Vertical layout.
    Vertical,
}

/// Specifies how slider values are managed.
#[derive(Clone)]
pub enum SliderValues {
    /// Uncontrolled mode: the hook manages state internally, initialized with these values.
    Uncontrolled(Vec<f64>),
    /// Controlled mode: an external signal is the source of truth.
    Controlled(Signal<Vec<f64>>),
}

/// Input parameters for creating slider state.
#[derive(Clone)]
pub struct UseSliderStateInput {
    /// How the slider values are managed (uncontrolled or controlled).
    pub values: SliderValues,

    /// The minimum value of the slider.
    pub min_value: f64,

    /// The maximum value of the slider.
    pub max_value: f64,

    /// The step increment. When `None`, the slider operates in continuous mode.
    pub step: Option<f64>,

    /// Whether the slider is disabled.
    pub disabled: Signal<bool>,

    /// The current orientation of the slider.
    pub orientation: Signal<SliderOrientation>,

    /// Callback fired when any value changes during interaction.
    pub on_change: Option<Callback<Vec<f64>>>,

    /// Callback fired when the user finishes dragging (all thumbs released).
    pub on_change_end: Option<Callback<Vec<f64>>>,
}

/// State for managing multi-thumb slider values.
#[derive(Debug, Clone, Copy)]
pub struct UseSliderStateReturn {
    /// Whether the slider is disabled.
    pub disabled: Signal<bool>,

    /// The current orientation of the slider.
    pub orientation: Signal<SliderOrientation>,

    /// The current values of all thumbs.
    pub values: Signal<Vec<f64>>,

    /// Get the value of a specific thumb.
    pub get_thumb_value: Callback<usize, f64>,

    /// Set the value of a specific thumb.
    pub set_thumb_value: Callback<(usize, f64)>,

    /// Set a thumb's value as a percentage (0.0-1.0).
    pub set_thumb_percent: Callback<(usize, f64)>,

    /// Get a thumb's value as a percentage (0.0-1.0).
    pub get_thumb_percent: Callback<usize, f64>,

    /// Get the minimum allowed value for a thumb (constrained by previous thumb).
    pub get_thumb_min_value: Callback<usize, f64>,

    /// Get the maximum allowed value for a thumb (constrained by next thumb).
    pub get_thumb_max_value: Callback<usize, f64>,

    /// Increment a thumb's value. Optional `step_size` for Shift+Arrow.
    pub increment_thumb: Callback<(usize, Option<f64>)>,

    /// Decrement a thumb's value. Optional `step_size` for Shift+Arrow.
    pub decrement_thumb: Callback<(usize, Option<f64>)>,

    /// Check if a specific thumb is being dragged.
    pub is_thumb_dragging: Callback<usize, bool>,

    /// Set the dragging state for a specific thumb.
    pub set_thumb_dragging: Callback<(usize, bool)>,

    /// The currently focused thumb index (if any).
    pub focused_thumb: Signal<Option<usize>>,

    /// Set which thumb is focused.
    pub set_focused_thumb: Callback<Option<usize>>,

    /// The step value (`None` for continuous mode).
    pub step: Option<f64>,

    /// The page size for PageUp/PageDown and Shift+Arrow.
    pub page_size: f64,

    /// The minimum value.
    pub min_value: f64,

    /// The maximum value.
    pub max_value: f64,

    /// The number of thumbs.
    pub num_thumbs: usize,

    /// Convert any value to a percentage (0.0-1.0).
    pub get_value_percent: Callback<f64, f64>,

    /// Convert a percentage (0.0-1.0) to a value.
    pub get_percent_value: Callback<f64, f64>,

    /// Check if a specific thumb is editable.
    pub is_thumb_editable: Callback<usize, bool>,

    /// Set whether a specific thumb is editable.
    pub set_thumb_editable: Callback<(usize, bool)>,
}

/// Clamp a value within a range without step snapping (continuous mode).
/// Handles reversed ranges where `min > max`.
fn clamp_value(value: f64, min: f64, max: f64) -> f64 {
    let lower = f64::min(min, max);
    let upper = f64::max(min, max);
    value.clamp(lower, upper)
}

/// Internal representation of slider state ownership.
#[derive(Clone, Copy)]
enum InternalState {
    Uncontrolled {
        read: ReadSignal<Vec<f64>>,
        write: WriteSignal<Vec<f64>>,
    },
    Controlled,
}

/// Creates sharable state for a multi-thumb slider component.
#[allow(clippy::too_many_lines, clippy::needless_pass_by_value)]
pub fn use_slider_state(input: UseSliderStateInput) -> UseSliderStateReturn {
    let min_value = input.min_value;
    let max_value = input.max_value;
    let step = input.step;
    let disabled = input.disabled;
    let orientation = input.orientation;
    let on_change = input.on_change;
    let on_change_end = input.on_change_end;

    // Precompute precision once to avoid repeated string allocations
    let precision = step.map(decimal_precision);

    let (num_thumbs, state, values) = match input.values {
        SliderValues::Uncontrolled(defaults) => {
            let mut processed: Vec<f64> = defaults
                .iter()
                .map(|v| match (step, precision) {
                    (Some(s), Some(p)) => snap_value_to_step(*v, min_value, max_value, s, p),
                    _ => clamp_value(*v, min_value, max_value),
                })
                .collect();
            processed.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            let num = processed.len();
            let (read, write) = signal(processed);
            let values: Signal<Vec<f64>> = read.into();
            (num, InternalState::Uncontrolled { read, write }, values)
        }
        SliderValues::Controlled(ext) => {
            let num = ext.with_untracked(Vec::len);
            (num, InternalState::Controlled, ext)
        }
    };

    let (dragging_thumbs, set_dragging_thumbs) = signal(vec![false; num_thumbs]);
    let (focused_thumb, set_focused_thumb) = signal(Option::<usize>::None);
    let (editable_thumbs, set_editable_thumbs) = signal(vec![true; num_thumbs]);

    // Page size: use step-based calculation when step is present, else 10% of range.
    let page_size = match step {
        Some(s) => calculate_page_size(min_value, max_value, s),
        None => (max_value - min_value) / 10.0,
    };

    // Default keyboard increment: step if present, else 1% of range.
    let default_increment = match step {
        Some(s) => s,
        None => (max_value - min_value) / 100.0,
    };

    // Helper to constrain a value for the given thumb index.
    let constrain_value = move |index: usize, new_value: f64| -> f64 {
        let vals = values.get_untracked();

        let thumb_min = if index == 0 {
            min_value
        } else {
            vals.get(index - 1).copied().unwrap_or(min_value)
        };
        let thumb_max = if index >= vals.len() - 1 {
            max_value
        } else {
            vals.get(index + 1).copied().unwrap_or(max_value)
        };

        match (step, precision) {
            (Some(s), Some(p)) => snap_value_to_step(new_value, thumb_min, thumb_max, s, p),
            _ => clamp_value(new_value, thumb_min, thumb_max),
        }
    };

    // Helper to set a thumb's value with constraints
    let update_thumb_value = move |index: usize, new_value: f64| {
        if disabled.get_untracked() {
            return;
        }

        let snapped = constrain_value(index, new_value);

        match state {
            InternalState::Uncontrolled { read, write } => {
                write.update(|vals| {
                    if let Some(v) = vals.get_mut(index) {
                        *v = snapped;
                    }
                });
                if let Some(on_change) = on_change {
                    on_change.run(read.get_untracked());
                }
            }
            InternalState::Controlled => {
                if let Some(on_change) = on_change {
                    let mut new_vals = values.get_untracked();
                    if let Some(v) = new_vals.get_mut(index) {
                        *v = snapped;
                    }
                    on_change.run(new_vals);
                }
            }
        }
    };

    // Check if all thumbs have stopped dragging for on_change_end
    let check_change_end = move || {
        if let Some(on_change_end) = on_change_end {
            if dragging_thumbs.get_untracked().iter().all(|d| !d) {
                on_change_end.run(values.get_untracked());
            }
        }
    };

    UseSliderStateReturn {
        disabled,
        orientation,
        values,
        get_thumb_value: Callback::new(move |index: usize| {
            values
                .get_untracked()
                .get(index)
                .copied()
                .unwrap_or(min_value)
        }),
        set_thumb_value: Callback::new(move |(index, value): (usize, f64)| {
            update_thumb_value(index, value);
        }),
        set_thumb_percent: Callback::new(move |(index, percent): (usize, f64)| {
            let range = max_value - min_value;
            let new_value = min_value + percent.clamp(0.0, 1.0) * range;
            update_thumb_value(index, new_value);
        }),
        get_thumb_percent: Callback::new(move |index: usize| {
            let val = values
                .get_untracked()
                .get(index)
                .copied()
                .unwrap_or(min_value);
            percentage_in_range(min_value, max_value, val)
        }),
        get_thumb_min_value: Callback::new(move |index: usize| {
            if index == 0 {
                min_value
            } else {
                values
                    .get_untracked()
                    .get(index - 1)
                    .copied()
                    .unwrap_or(min_value)
            }
        }),
        get_thumb_max_value: Callback::new(move |index: usize| {
            let vals = values.get_untracked();
            if index >= vals.len() - 1 {
                max_value
            } else {
                vals.get(index + 1).copied().unwrap_or(max_value)
            }
        }),
        increment_thumb: Callback::new(move |(index, step_size): (usize, Option<f64>)| {
            let current = values
                .get_untracked()
                .get(index)
                .copied()
                .unwrap_or(min_value);
            let increment = step_size.unwrap_or(default_increment);
            update_thumb_value(index, current + increment);
        }),
        decrement_thumb: Callback::new(move |(index, step_size): (usize, Option<f64>)| {
            let current = values
                .get_untracked()
                .get(index)
                .copied()
                .unwrap_or(min_value);
            let decrement = step_size.unwrap_or(default_increment);
            update_thumb_value(index, current - decrement);
        }),
        is_thumb_dragging: Callback::new(move |index: usize| {
            dragging_thumbs
                .get_untracked()
                .get(index)
                .copied()
                .unwrap_or(false)
        }),
        set_thumb_dragging: Callback::new(move |(index, dragging): (usize, bool)| {
            if disabled.get_untracked() {
                return;
            }

            set_dragging_thumbs.update(|thumbs| {
                if let Some(t) = thumbs.get_mut(index) {
                    *t = dragging;
                }
            });
            if !dragging {
                check_change_end();
            }
        }),
        focused_thumb: focused_thumb.into(),
        set_focused_thumb: Callback::new(move |thumb: Option<usize>| {
            set_focused_thumb.set(thumb);
        }),
        step,
        page_size,
        min_value,
        max_value,
        num_thumbs,
        get_value_percent: Callback::new(move |value: f64| {
            percentage_in_range(min_value, max_value, value).clamp(0.0, 1.0)
        }),
        get_percent_value: Callback::new(move |percent: f64| {
            let clamped = percent.clamp(0.0, 1.0);
            let raw_value = min_value + clamped * (max_value - min_value);
            match (step, precision) {
                (Some(s), Some(p)) => snap_value_to_step(raw_value, min_value, max_value, s, p),
                _ => clamp_value(raw_value, min_value, max_value),
            }
        }),
        is_thumb_editable: Callback::new(move |index: usize| {
            editable_thumbs
                .get_untracked()
                .get(index)
                .copied()
                .unwrap_or(true)
        }),
        set_thumb_editable: Callback::new(move |(index, editable): (usize, bool)| {
            set_editable_thumbs.update(|thumbs| {
                if let Some(t) = thumbs.get_mut(index) {
                    *t = editable;
                }
            });
        }),
    }
}
