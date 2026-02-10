use crate::utils::math::percentage_in_range;
use leptos::attr;
use leptos::attr::Attr;
use leptos::prelude::*;
use uuid::Uuid;

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/progress/src/useProgressBar.ts

/// Input parameters for the `use_progress_bar` hook.
#[derive(Debug, Clone)]
pub struct UseProgressBarInput {
    /// The current value (None for indeterminate).
    pub value: Signal<Option<f64>>,

    /// The minimum value.
    pub min_value: f64,

    /// The maximum value.
    pub max_value: f64,

    /// The label for the progress bar.
    pub label: Option<String>,

    /// Whether to show the value label.
    pub show_value_label: bool,

    /// Whether the progress is indeterminate.
    pub is_indeterminate: bool,
}

impl Default for UseProgressBarInput {
    fn default() -> Self {
        Self {
            value: Signal::derive(|| Some(0.0)),
            min_value: 0.0,
            max_value: 100.0,
            label: None,
            show_value_label: true,
            is_indeterminate: false,
        }
    }
}

/// The return value of the `use_progress_bar` hook.
pub struct UseProgressBarReturn {
    /// Props for the progress bar container element.
    pub progress_props: UseProgressBarAttrs,

    /// Props for the label element.
    pub label_props: UseProgressBarLabelProps,

    /// The percentage value (0-100), None if indeterminate.
    pub percentage: Signal<Option<f64>>,

    /// The formatted value label.
    pub value_label: Signal<String>,

    /// Whether the progress is indeterminate.
    pub is_indeterminate: bool,

    /// The ID of the progress bar.
    pub progress_id: String,
}

/// Attributes for the progress bar container element.
pub type UseProgressBarAttrs = (
    Attr<attr::Id, String>,
    Attr<attr::Role, &'static str>,
    Attr<attr::AriaValuenow, Signal<Option<String>>>,
    Attr<attr::AriaValuemin, String>,
    Attr<attr::AriaValuemax, String>,
    Attr<attr::AriaValuetext, Signal<Option<String>>>,
    Attr<attr::AriaLabelledby, Option<String>>,
);

/// Props for the progress bar label element.
#[derive(Debug, Clone)]
pub struct UseProgressBarLabelProps {
    /// The ID of the label.
    pub id: String,
}

/// Provides the behavior and accessibility for a progress bar.
///
/// A progress bar shows completion status of a task. It can be determinate
/// (showing specific progress) or indeterminate (showing activity without specific progress).
///
/// # Example
///
/// ```ignore
/// let progress = use_progress_bar(UseProgressBarInput {
///     value: Signal::derive(|| Some(45.0)),
///     label: Some("Uploading...".to_string()),
///     ..Default::default()
/// });
///
/// view! {
///     <div>
///         <label id=progress.label_props.id>"Uploading..."</label>
///         <div {..progress.progress_props}>
///             <div style=format!("width: {}%", progress.percentage.get().unwrap_or(0.0))></div>
///         </div>
///         <span>{move || progress.value_label.get()}</span>
///     </div>
/// }
/// ```
#[allow(clippy::needless_pass_by_value)]
pub fn use_progress_bar(input: UseProgressBarInput) -> UseProgressBarReturn {
    let UseProgressBarInput {
        value,
        min_value,
        max_value,
        label,
        show_value_label,
        is_indeterminate,
    } = input;

    let base_id = Uuid::new_v4();
    let progress_id = format!("progress-{base_id}");
    let label_id = format!("progress-label-{base_id}");

    // Compute percentage
    let percentage = Signal::derive(move || {
        if is_indeterminate {
            return None;
        }

        value
            .get()
            .map(|v| (percentage_in_range(min_value, max_value, v) * 100.0).clamp(0.0, 100.0))
    });

    // Compute value label
    let value_label = Signal::derive(move || {
        if is_indeterminate {
            return String::new();
        }

        percentage
            .get()
            .map(|pct| format!("{pct:.0}%"))
            .unwrap_or_default()
    });

    // ARIA value attributes (None for indeterminate)
    let aria_valuenow = Signal::derive(move || {
        if is_indeterminate {
            None
        } else {
            value.get().map(|v| v.to_string())
        }
    });

    let aria_valuetext = Signal::derive(move || {
        if is_indeterminate {
            None
        } else {
            let label = value_label.get();
            if label.is_empty() {
                None
            } else {
                Some(label)
            }
        }
    });

    let aria_labelledby = if label.is_some() {
        Some(label_id.clone())
    } else {
        None
    };

    UseProgressBarReturn {
        progress_props: (
            Attr(attr::Id, progress_id.clone()),
            Attr(attr::Role, "progressbar"),
            Attr(attr::AriaValuenow, aria_valuenow),
            Attr(attr::AriaValuemin, min_value.to_string()),
            Attr(attr::AriaValuemax, max_value.to_string()),
            Attr(attr::AriaValuetext, aria_valuetext),
            Attr(attr::AriaLabelledby, aria_labelledby),
        ),
        label_props: UseProgressBarLabelProps { id: label_id },
        percentage,
        value_label,
        is_indeterminate,
        progress_id,
    }
}
