use leptos::prelude::*;

use crate::{
    atoms::{
        field::Label,
        progress_bar::{ProgressBar as ProgressBarAtom, ProgressBarFill, ProgressBarValueText},
    },
    utils::{
        classes::Classes,
        number_formatter::NumberFormatOptions,
        number_value::{NumberValue, OptionalNumberSignal},
        styles::Styles,
    },
};

/// A themed progress bar: a track filled up to the progress, with the value text on top. `None`
/// shows indeterminate progress (an animated fill).
///
/// ```ignore
/// view! { <ProgressBar value=uploaded max_value=total_bytes aria_label="Upload" /> }
/// ```
#[allow(clippy::too_many_arguments)]
#[component]
pub fn ProgressBar<T: NumberValue>(
    /// The progress: a number, an `Option`, or any signal of them. `None`: indeterminate.
    #[prop(into)]
    value: OptionalNumberSignal<T>,
    /// Default: 0.
    #[prop(into, optional)]
    min_value: Option<Signal<T>>,
    /// Default: 100.
    #[prop(into, optional)]
    max_value: Option<Signal<T>>,
    /// How the value text is formatted. Default: percent.
    #[prop(into, optional)]
    format_options: Option<Signal<NumberFormatOptions>>,
    /// Replaces the formatted value text (e.g. "1 of 4").
    #[prop(into, optional)]
    value_label: MaybeProp<String>,
    /// The visible label above the bar. Without it, set `aria_label`.
    #[prop(into, optional)]
    label: Option<String>,
    #[prop(into, optional)] aria_label: MaybeProp<String>,
    /// Whether the value text shows in the bar. Default: true.
    #[prop(into, default = true)]
    show_value: bool,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
) -> impl IntoView {
    if label.is_none() && aria_label.get_untracked().is_none() {
        crate::utils::dev_warn!("A <ProgressBar> needs a `label` or an `aria_label`.");
    }
    view! {
        <ProgressBarAtom
            value
            nostrip:min_value=min_value
            nostrip:max_value=max_value
            nostrip:format_options=format_options
            value_label
            aria_label
            classes=classes.add("leptonic-progress-bar")
            styles
        >
            {label.map(|label| view! { <Label classes="leptonic-progress-bar-label">{label}</Label> })}
            <div class="leptonic-progress-bar-background">
                <ProgressBarFill classes="leptonic-progress-bar-fill" />
                {show_value.then(|| view! { <ProgressBarValueText classes="leptonic-progress-info" /> })}
            </div>
        </ProgressBarAtom>
    }
}
