use leptos::attr;
use leptos::attr::Attr;
use leptos::prelude::*;
use uuid::Uuid;

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/meter/src/useMeter.ts

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
    pub meter_props: UseMeterAttrs,

    /// Props for the label element.
    pub label_props: UseMeterLabelProps,

    /// The percentage value (0-100).
    pub percentage: Signal<f64>,

    /// The formatted value label.
    pub value_label: Signal<String>,

    /// The ID of the meter.
    pub meter_id: String,
}

/// Attributes for the meter container element.
pub type UseMeterAttrs = (
    Attr<attr::Id, String>,
    Attr<attr::Role, &'static str>,
    Attr<attr::AriaValuenow, Signal<String>>,
    Attr<attr::AriaValuemin, String>,
    Attr<attr::AriaValuemax, String>,
    Attr<attr::AriaValuetext, Signal<String>>,
    Attr<attr::AriaLabelledby, Option<String>>,
);

/// Props for the meter label element.
#[derive(Debug, Clone)]
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
    let base_id = Uuid::new_v4();
    let meter_id = format!("meter-{base_id}");
    let label_id = format!("meter-label-{base_id}");

    let value = input.value;
    let min_value = input.min_value;
    let max_value = input.max_value;
    let format_options = input.format_options;

    // Compute percentage
    let percentage = Signal::derive(move || {
        let v = value.get();
        let range = max_value - min_value;
        if range == 0.0 {
            0.0
        } else {
            ((v - min_value) / range * 100.0).clamp(0.0, 100.0)
        }
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

    let aria_labelledby = if input.label.is_some() {
        Some(label_id.clone())
    } else {
        None
    };

    UseMeterReturn {
        meter_props: (
            Attr(attr::Id, meter_id.clone()),
            Attr(attr::Role, "meter"),
            Attr(attr::AriaValuenow, aria_valuenow),
            Attr(attr::AriaValuemin, min_value.to_string()),
            Attr(attr::AriaValuemax, max_value.to_string()),
            Attr(attr::AriaValuetext, aria_valuetext),
            Attr(attr::AriaLabelledby, aria_labelledby),
        ),
        label_props: UseMeterLabelProps { id: label_id },
        percentage,
        value_label,
        meter_id,
    }
}
