// Upstream: react-aria-components/src/Meter.tsx @ 99e6102368
use leptos::{context::Provider, prelude::*};
use leptos_classes::Classes;

use crate::{
    IntoAttrs,
    atoms::{
        field::{LabelContext, LabelPresence},
        progress_bar::{ValueContext, fill, value_text},
    },
    hooks::{
        meter::{UseMeterInput, use_meter},
        progress::UseProgressBarReturn,
    },
    utils::{
        default_class::with_default_class,
        number_formatter::NumberFormatOptions,
        number_value::{NumberSignal, NumberValue},
        styles::Styles,
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Generic over the value type, as `use_meter`.
// - Render props (`percentage`, `valueText`) become the parts `MeterFill` (sized to the
//   percentage) and `MeterValueText`. Reason: a component's children aren't a function of its
//   state in Leptos.
// - No slots (`MeterContext`) and no `render` prop.
//
// =============================================================================

/// A value within a known range (disk usage, battery level). Label it with a
/// [`Label`](super::field::Label) (or `aria_label`); show it with a [`MeterFill`] in a track of
/// your own and a [`MeterValueText`].
///
/// ```ignore
/// view! {
///     <Meter value=storage_used>
///         <Label>"Storage"</Label>
///         <MeterValueText />
///         <div class="track"><MeterFill classes="fill" /></div>
///     </Meter>
/// }
/// ```
///
/// Default class: `leptonic-Meter`.
#[allow(clippy::too_many_arguments)]
#[component]
pub fn Meter<T: NumberValue>(
    /// The value, clamped to the range: a number or any signal of one.
    #[prop(into)]
    value: NumberSignal<T>,
    /// Default: 0.
    #[prop(into, optional)]
    min_value: Option<Signal<T>>,
    /// Default: 100.
    #[prop(into, optional)]
    max_value: Option<Signal<T>>,
    /// How the value text is formatted. A percent style formats the percentage, other styles the
    /// value. Default: percent.
    #[prop(into, optional)]
    format_options: Option<Signal<NumberFormatOptions>>,
    /// Replaces the formatted value text (e.g. "3 of 4 GB").
    #[prop(into, optional)]
    value_label: MaybeProp<String>,
    #[prop(into, optional)] id: Option<String>,
    /// Names the meter when it has no `Label`.
    #[prop(into, optional)]
    aria_label: MaybeProp<String>,
    #[prop(into, optional)] aria_labelledby: Option<String>,
    #[prop(into, optional)] aria_describedby: Option<String>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let classes = with_default_class("leptonic-Meter", classes);
    let defaults = UseMeterInput::<T>::default();
    let label_presence = LabelPresence::new(aria_label, aria_labelledby.as_ref());
    let has_label = label_presence.has_label;
    let UseProgressBarReturn {
        props,
        label_props,
        percentage,
        value_text,
    } = use_meter(UseMeterInput {
        value: value.into_signal(),
        min_value: min_value.unwrap_or(defaults.min_value),
        max_value: max_value.unwrap_or(defaults.max_value),
        format_options: format_options.unwrap_or(defaults.format_options),
        value_label,
        id,
        has_label,
        aria_label,
        aria_labelledby,
        aria_describedby,
    });
    view! {
        <Provider value=LabelContext::span(label_props).with_presence(label_presence)>
            <Provider value=ValueContext { percentage, value_text }>
                <div {..props.into_attrs()} class=classes style=styles>
                    {children.map(|children| children())}
                </div>
            </Provider>
        </Provider>
    }
}

/// The filled part of a [`Meter`]'s track: as wide as the value (in percent of its container),
/// which `--percent` holds too.
///
/// Default class: `leptonic-MeterFill`.
#[component]
pub fn MeterFill(
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
) -> impl IntoView {
    let classes = with_default_class("leptonic-MeterFill", classes);
    fill("MeterFill", classes, styles)
}

/// The formatted value of a [`Meter`] (its `aria-valuetext`).
///
/// Default class: `leptonic-MeterValueText`.
#[component]
pub fn MeterValueText(
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
) -> impl IntoView {
    let classes = with_default_class("leptonic-MeterValueText", classes);
    value_text("MeterValueText", classes, styles)
}
