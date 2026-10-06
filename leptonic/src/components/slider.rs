use std::ops::RangeInclusive;

use leptos::prelude::*;
use ordered_float::OrderedFloat;

use crate::{
    Out,
    atoms::slider::{
        Slider as SliderAtom, SliderFill, SliderMark as SliderMarkAtom,
        SliderMarks as SliderMarksAtom, SliderThumb, SliderThumbTooltip, SliderTrack,
    },
    utils::number_value::NumberSignal,
    utils::{
        classes::Classes, number_formatter::NumberFormatOptions, number_value::NumberValue,
        orientation::Orientation, styles::Styles,
    },
};
pub use crate::{
    atoms::slider::SliderPopover,
    hooks::{SliderMark, SliderMarkValue, SliderMarks},
};

/// The look of a themed slider's thumbs.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum SliderVariant {
    Block,
    #[default]
    Round,
}

impl SliderVariant {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Block => "block",
            Self::Round => "round",
        }
    }
}

/// The themed parts inside the slider atom: track, fill, thumbs with tooltips, marks.
fn slider_parts(
    thumbs: Vec<(usize, MaybeProp<String>)>,
    popover: SliderPopover,
    marks: SliderMarks,
) -> impl IntoView {
    view! {
        <SliderTrack classes="track">
            <SliderFill classes="fill" />
            {thumbs
                .into_iter()
                .map(|(index, aria_label)| {
                    view! {
                        <SliderThumb index aria_label classes="thumb">
                            <SliderThumbTooltip popover classes="tooltip" />
                        </SliderThumb>
                    }
                })
                .collect_view()}
        </SliderTrack>
        <SliderMarksAtom marks classes="marks" let:marks>
            <For
                each=move || marks.get()
                key=|mark| OrderedFloat::from(mark.percentage)
                let:mark
            >
                <SliderMarkAtom mark=mark.clone() classes="mark">
                    {mark.name.map(|name| view! { <div class="title">{name}</div> })}
                </SliderMarkAtom>
            </For>
        </SliderMarksAtom>
    }
}

/// A themed slider picking one value.
///
/// ```ignore
/// let volume = RwSignal::new(30);
/// view! { <Slider value=volume set_value=volume min_value=0 max_value=100 aria_label="Volume"/> }
/// ```
#[component]
pub fn Slider<T: NumberValue>(
    /// The value: a number or any signal of one.
    #[prop(into)]
    value: NumberSignal<T>,
    /// Receives the new value: an `RwSignal`, `WriteSignal`, closure, `Callback`, ...
    #[prop(into)]
    set_value: Out<T>,
    #[prop(into)] min_value: Signal<T>,
    #[prop(into)] max_value: Signal<T>,
    /// The step between values. Default: 1.
    #[prop(into, default = Signal::stored(T::ONE))]
    step: Signal<T>,
    #[prop(into, default = Signal::stored(Orientation::Horizontal))] orientation: Signal<
        Orientation,
    >,
    #[prop(optional)] variant: SliderVariant,
    /// When the value shows above the thumb.
    #[prop(optional)]
    popover: SliderPopover,
    #[prop(optional)] marks: SliderMarks,
    /// How the value is formatted (tooltip, marks, assistive technology).
    #[prop(into, optional)]
    format_options: Signal<NumberFormatOptions>,
    #[prop(into, optional)] is_disabled: Signal<bool>,
    /// Names the slider (there is no visible label).
    #[prop(into, optional)]
    aria_label: MaybeProp<String>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
) -> impl IntoView {
    let value = value.into_signal();
    let values = Signal::derive(move || vec![value.get()]);
    let set_values = Out::new_callback(move |values: Vec<T>| {
        if let Some(first) = values.first() {
            set_value.set(*first);
        }
    });
    view! {
        <SliderAtom
            values
            set_values
            min_value
            max_value
            step
            orientation
            format_options
            is_disabled
            aria_label
            classes=classes.add("leptonic-slider")
            styles
            attr:data-variant=variant.as_str()
        >
            {slider_parts(vec![(0, MaybeProp::default())], popover, marks)}
        </SliderAtom>
    }
}

/// A themed slider picking a range with two thumbs, which can't pass each other.
///
/// ```ignore
/// let price = RwSignal::new(20..=80);
/// view! { <RangeSlider value=price set_value=price min_value=0 max_value=100 aria_label="Price"/> }
/// ```
#[component]
pub fn RangeSlider<T: NumberValue>(
    /// The range: a plain value or any signal.
    #[prop(into)]
    value: Signal<RangeInclusive<T>>,
    /// Receives the new range: an `RwSignal`, `WriteSignal`, closure, `Callback`, ...
    #[prop(into)]
    set_value: Out<RangeInclusive<T>>,
    #[prop(into)] min_value: Signal<T>,
    #[prop(into)] max_value: Signal<T>,
    /// The step between values. Default: 1.
    #[prop(into, default = Signal::stored(T::ONE))]
    step: Signal<T>,
    #[prop(into, default = Signal::stored(Orientation::Horizontal))] orientation: Signal<
        Orientation,
    >,
    #[prop(optional)] variant: SliderVariant,
    #[prop(optional)] popover: SliderPopover,
    #[prop(optional)] marks: SliderMarks,
    #[prop(into, optional)] format_options: Signal<NumberFormatOptions>,
    #[prop(into, optional)] is_disabled: Signal<bool>,
    /// Names the slider (there is no visible label).
    #[prop(into, optional)]
    aria_label: MaybeProp<String>,
    /// Names the thumbs next to the slider's name. Default: "Minimum", "Maximum".
    #[prop(into, default = Signal::stored(("Minimum".to_owned(), "Maximum".to_owned())))]
    thumb_labels: Signal<(String, String)>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
) -> impl IntoView {
    let values = Signal::derive(move || value.with(|range| vec![*range.start(), *range.end()]));
    let set_values = Out::new_callback(move |values: Vec<T>| {
        if let [start, end] = values.as_slice() {
            set_value.set(*start..=*end);
        }
    });
    let start_label = MaybeProp::derive(move || Some(thumb_labels.get().0));
    let end_label = MaybeProp::derive(move || Some(thumb_labels.get().1));
    view! {
        <SliderAtom
            values
            set_values
            min_value
            max_value
            step
            orientation
            format_options
            is_disabled
            aria_label
            classes=classes.add("leptonic-slider")
            styles
            attr:data-variant=variant.as_str()
        >
            {slider_parts(vec![(0, start_label), (1, end_label)], popover, marks)}
        </SliderAtom>
    }
}
