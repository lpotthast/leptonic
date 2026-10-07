// Upstream: react-aria-components/src/ProgressBar.tsx @ 99e6102368
use leptos::{context::Provider, prelude::*};

use crate::{
    atoms::field::{LabelContext, LabelPresence},
    hooks::{IntoAttrs, UseProgressBarInput, UseProgressBarReturn, use_progress_bar},
    utils::{
        classes::Classes,
        css::{computed_pct, computed_size},
        data_attributes::flag,
        default_class::with_default_class,
        number_formatter::NumberFormatOptions,
        number_value::{NumberValue, OptionalNumberSignal},
        style::WidthProperty,
        styles::Styles,
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - As `use_progress_bar`: `value: None` is indeterminate (no `isIndeterminate`), generic over
//   the value type.
// - Render props (`percentage`, `valueText`, `isIndeterminate`) become the parts
//   `ProgressBarFill` (sized to the percentage) and `ProgressBarValueText`, plus
//   `data-indeterminate`. Reason: a component's children aren't a function of its state in
//   Leptos.
// - No slots (`ProgressBarContext`) and no `render` prop.
//
// =============================================================================

/// What the parts of a progress bar or meter show.
#[derive(Debug, Clone, Copy)]
pub(crate) struct ValueContext {
    pub percentage: Signal<Option<f64>>,
    pub value_text: Signal<Option<String>>,
}

/// The progress of an operation over time, known (`value`) or not (`value: None`). Label it with
/// a [`Label`](super::field::Label) (or `aria_label`); show it with a [`ProgressBarFill`] in a
/// track of your own and a [`ProgressBarValueText`].
///
/// ```ignore
/// view! {
///     <ProgressBar value=Some(uploaded) max_value=total>
///         <Label>"Uploading"</Label>
///         <ProgressBarValueText />
///         <div class="track"><ProgressBarFill classes="fill" /></div>
///     </ProgressBar>
/// }
/// ```
///
/// Data attributes: `data-indeterminate`.
///
/// Default class: `leptonic-ProgressBar`.
#[allow(clippy::too_many_arguments)]
#[component]
pub fn ProgressBar<T: NumberValue>(
    /// The progress, clamped to the range: a number, an `Option`, or any signal of them. `None`:
    /// indeterminate.
    #[prop(into)]
    value: OptionalNumberSignal<T>,
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
    /// Replaces the formatted value text (e.g. "1 of 4").
    #[prop(into, optional)]
    value_label: MaybeProp<String>,
    #[prop(into, optional)] id: Option<String>,
    /// Names the progress bar when it has no `Label`.
    #[prop(into, optional)]
    aria_label: MaybeProp<String>,
    #[prop(into, optional)] aria_labelledby: Option<String>,
    #[prop(into, optional)] aria_describedby: Option<String>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let classes = with_default_class("leptonic-ProgressBar", classes);
    let value = value.into_signal();
    let defaults = UseProgressBarInput::<T>::default();
    let label_presence = LabelPresence::new(aria_label, aria_labelledby.as_ref());
    let has_label = label_presence.has_label;
    let UseProgressBarReturn {
        props,
        label_props,
        percentage,
        value_text,
    } = use_progress_bar(UseProgressBarInput {
        value,
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
    let is_indeterminate = Signal::derive(move || value.with(Option::is_none));
    view! {
        <Provider value=LabelContext::span(label_props).with_presence(label_presence)>
            <Provider value=ValueContext { percentage, value_text }>
                <div
                    {..props.into_attrs()}
                    class=classes
                    style=styles
                    data-indeterminate=flag(is_indeterminate)
                >
                    {children.map(|children| children())}
                </div>
            </Provider>
        </Provider>
    }
}

/// The filled part of a [`ProgressBar`]'s track: as wide as the progress (in percent of its
/// container). While indeterminate, it has no width of its own: animate it with CSS on
/// `[data-indeterminate]`.
///
/// Data attributes: `data-indeterminate`.
///
/// Default class: `leptonic-ProgressBarFill`.
#[component]
pub fn ProgressBarFill(
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
) -> impl IntoView {
    let classes = with_default_class("leptonic-ProgressBarFill", classes);
    fill(classes, styles)
}

/// The formatted value of a [`ProgressBar`] (its `aria-valuetext`); empty while indeterminate.
///
/// Default class: `leptonic-ProgressBarValueText`.
#[component]
pub fn ProgressBarValueText(
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
) -> impl IntoView {
    let classes = with_default_class("leptonic-ProgressBarValueText", classes);
    value_text(classes, styles)
}

/// A fill sized to the percentage of the surrounding progress bar or meter.
pub(crate) fn fill(classes: Classes, styles: Styles) -> impl IntoView {
    let ValueContext { percentage, .. } = expect_context::<ValueContext>();
    let styles = Styles::new()
        .add_optional(move || {
            percentage
                .get()
                .map(|percentage| WidthProperty.declare(computed_size(computed_pct(percentage))))
        })
        .merge(styles);
    view! {
        <div
            class=classes
            style=styles
            data-indeterminate=flag(Signal::derive(move || percentage.with(Option::is_none)))
        />
    }
}

/// The value text of the surrounding progress bar or meter.
pub(crate) fn value_text(classes: Classes, styles: Styles) -> impl IntoView {
    let ValueContext { value_text, .. } = expect_context::<ValueContext>();
    view! { <span class=classes style=styles>{value_text}</span> }
}
