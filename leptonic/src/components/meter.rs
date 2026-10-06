use leptos::prelude::*;

use crate::{
    atoms::{
        field::Label,
        meter::{Meter as MeterAtom, MeterFill, MeterValueText},
    },
    utils::{
        classes::Classes,
        number_formatter::NumberFormatOptions,
        number_value::{NumberSignal, NumberValue},
        styles::Styles,
    },
};

/// A themed meter: a value within a known range (disk usage, battery level) as a filled track,
/// with its label and value text above.
///
/// ```ignore
/// view! { <Meter value=used_gb max_value=64.0 label="Storage" /> }
/// ```
#[allow(clippy::too_many_arguments)]
#[component]
pub fn Meter<T: NumberValue>(
    /// The value: a number or any signal of one.
    #[prop(into)]
    value: NumberSignal<T>,
    /// Default: 0.
    #[prop(into, optional)]
    min_value: Option<Signal<T>>,
    /// Default: 100.
    #[prop(into, optional)]
    max_value: Option<Signal<T>>,
    /// How the value text is formatted. Default: percent.
    #[prop(into, optional)]
    format_options: Option<Signal<NumberFormatOptions>>,
    /// Replaces the formatted value text (e.g. "3 of 4 GB").
    #[prop(into, optional)]
    value_label: MaybeProp<String>,
    /// The visible label above the track. Without it, set `aria_label`.
    #[prop(into, optional)]
    label: Option<String>,
    #[prop(into, optional)] aria_label: MaybeProp<String>,
    /// Whether the value text shows next to the label. Default: true.
    #[prop(into, default = true)]
    show_value: bool,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
) -> impl IntoView {
    if label.is_none() && aria_label.get_untracked().is_none() {
        crate::utils::dev_warn!("A <Meter> needs a `label` or an `aria_label`.");
    }
    let has_label = label.is_some();
    view! {
        <MeterAtom
            value
            nostrip:min_value=min_value
            nostrip:max_value=max_value
            nostrip:format_options=format_options
            value_label
            aria_label
            classes=classes.add("leptonic-meter")
            styles
        >
            // Built inside the atom: the value text reads the atom's context.
            {(has_label || show_value)
                .then(|| {
                    view! {
                        <div class="leptonic-meter-header">
                            {label.map(|label| view! { <Label classes="leptonic-meter-label">{label}</Label> })}
                            {show_value.then(|| view! { <MeterValueText classes="leptonic-meter-value" /> })}
                        </div>
                    }
                })}
            <div class="leptonic-meter-track">
                <MeterFill classes="leptonic-meter-fill" />
            </div>
        </MeterAtom>
    }
}
