use leptos::{attr, attr::Attr, prelude::*};
use uuid::Uuid;

use crate::{
    hooks::IntoAttrs,
    utils::{aria::AriaRole, math::percentage_in_range},
};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/meter/src/useMeter.ts

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// No intentional deviations from the react-aria implementation.
//
// =============================================================================

/// Input parameters for the `use_meter` hook.
#[derive(Debug, Clone)]
pub struct UseMeterInput {
    /// The current value.
    pub value: Signal<f64>,

    /// The minimum value.
    pub min_value: f64,

    /// The maximum value.
    pub max_value: f64,

    /// The label for the meter.
    pub label: Option<String>,

    /// Whether to show the value label.
    pub show_value_label: bool,

    /// Custom format for the value label.
    pub format_options: Option<MeterFormatOptions>,
}

/// Format options for meter value display.
#[derive(Debug, Clone)]
pub struct MeterFormatOptions {
    /// The style (percent or decimal).
    pub style: MeterFormatStyle,

    /// Number of decimal places.
    pub decimals: usize,
}

/// The format style for meter values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MeterFormatStyle {
    /// Show as percentage (e.g., "75%").
    #[default]
    Percent,
    /// Show as decimal (e.g., "0.75").
    Decimal,
}

impl Default for UseMeterInput {
    fn default() -> Self {
        Self {
            value: Signal::derive(|| 0.0),
            min_value: 0.0,
            max_value: 100.0,
            label: None,
            show_value_label: true,
            format_options: None,
        }
    }
}

/// The return value of the `use_meter` hook.
pub struct UseMeterReturn {
    /// Props for the meter container element.
    pub meter_props: UseMeterProps,

    /// Props for the label element.
    pub label_props: UseMeterLabelProps,

    /// The percentage value (0-100).
    pub percentage: Signal<f64>,

    /// The formatted value label.
    pub value_label: Signal<String>,

    /// The ID of the meter.
    pub meter_id: String,
}

/// Props from `use_meter` that can be extracted and merged programmatically.
#[derive(Debug)]
pub struct UseMeterProps {
    pub id: String,
    pub role: AriaRole,
    pub aria_valuenow: Signal<String>,
    pub aria_valuemin: String,
    pub aria_valuemax: String,
    pub aria_valuetext: Signal<String>,
    pub aria_labelledby: Option<String>,
}

impl IntoAttrs for UseMeterProps {
    type Attrs = UseMeterAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Id, self.id),
            Attr(attr::Role, self.role),
            Attr(attr::AriaValuenow, self.aria_valuenow),
            Attr(attr::AriaValuemin, self.aria_valuemin),
            Attr(attr::AriaValuemax, self.aria_valuemax),
            Attr(attr::AriaValuetext, self.aria_valuetext),
            Attr(attr::AriaLabelledby, self.aria_labelledby),
        )
    }
}

/// Attributes for the meter container element.
pub type UseMeterAttrs = (
    Attr<attr::Id, String>,
    Attr<attr::Role, AriaRole>,
    Attr<attr::AriaValuenow, Signal<String>>,
    Attr<attr::AriaValuemin, String>,
    Attr<attr::AriaValuemax, String>,
    Attr<attr::AriaValuetext, Signal<String>>,
    Attr<attr::AriaLabelledby, Option<String>>,
);

/// Props for the meter label element.
#[derive(Debug)]
pub struct UseMeterLabelProps {
    /// The ID of the label.
    pub id: String,
}

/// Provides the behavior and accessibility for a meter.
///
/// A meter represents a scalar value within a known range.
/// Unlike a progress bar, a meter is for static values (e.g., disk usage).
///
/// # Example
///
/// ```ignore
/// let meter = use_meter(UseMeterInput {
///     value: Signal::derive(|| 75.0),
///     label: Some("Disk Usage".to_string()),
///     ..Default::default()
/// });
///
/// view! {
///     <div>
///         <label id=meter.label_props.id>"Disk Usage"</label>
///         <div {..meter.meter_props}>
///             <div style=format!("width: {}%", meter.percentage.get())></div>
///         </div>
///         <span>{move || meter.value_label.get()}</span>
///     </div>
/// }
/// ```
pub fn use_meter(input: UseMeterInput) -> UseMeterReturn {
    let UseMeterInput {
        value,
        min_value,
        max_value,
        label,
        show_value_label,
        format_options,
    } = input;

    let base_id = Uuid::new_v4();
    let meter_id = format!("meter-{base_id}");
    let label_id = format!("meter-label-{base_id}");

    // Compute percentage
    let percentage = Signal::derive(move || {
        let v = value.get();
        (percentage_in_range(min_value, max_value, v) * 100.0).clamp(0.0, 100.0)
    });

    // Compute value label
    let value_label = Signal::derive(move || {
        let v = value.get();
        let pct = percentage.get();

        match &format_options {
            Some(opts) => match opts.style {
                MeterFormatStyle::Percent => {
                    let prec = opts.decimals;
                    format!("{pct:.prec$}%")
                }
                MeterFormatStyle::Decimal => {
                    let prec = opts.decimals;
                    format!("{v:.prec$}")
                }
            },
            None => format!("{pct:.0}%"),
        }
    });

    // ARIA value attributes
    let aria_valuenow = Signal::derive(move || value.get().to_string());
    let aria_valuetext = value_label;

    let aria_labelledby = if label.is_some() {
        Some(label_id.clone())
    } else {
        None
    };

    UseMeterReturn {
        meter_props: UseMeterProps {
            id: meter_id.clone(),
            role: AriaRole::Meter,
            aria_valuenow,
            aria_valuemin: min_value.to_string(),
            aria_valuemax: max_value.to_string(),
            aria_valuetext,
            aria_labelledby,
        },
        label_props: UseMeterLabelProps { id: label_id },
        percentage,
        value_label,
        meter_id,
    }
}
