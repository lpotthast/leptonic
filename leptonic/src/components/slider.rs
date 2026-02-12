use leptos::prelude::*;
use ordered_float::OrderedFloat;

// Re-export the popover enum from atoms.
pub use crate::atoms::slider::SliderPopover;
// Re-export mark types from hooks for backward compatibility.
pub use crate::hooks::{SliderMark, SliderMarkValue, SliderMarks};
use crate::{
    atoms::slider::{
        Slider as SliderAtom, SliderMark, SliderMarks, SliderThumb, SliderThumbTooltip,
        SliderTrack, SliderTrackFill,
    },
    hooks::SliderValues,
    utils::{classes::Classes, styles::Styles},
    Out,
};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum SliderVariant {
    Block,
    #[default]
    Round,
}

impl SliderVariant {
    const fn to_str(self) -> &'static str {
        match self {
            Self::Block => "block",
            Self::Round => "round",
        }
    }
}

/// Shared implementation for single and range sliders.
#[component]
#[allow(clippy::too_many_arguments)]
fn SliderInner(
    values: SliderValues,
    min: f64,
    max: f64,
    step: Option<f64>,
    disabled: Signal<bool>,
    on_change: Option<Callback<Vec<f64>>>,
    variant: &'static str,
    marks: SliderMarks,
    value_display: Option<Callback<f64, String>>,
    popover: SliderPopover,
    num_thumbs: usize,
    classes: Classes,
    styles: Styles,
) -> impl IntoView {
    view! {
        <SliderAtom
            values=values
            min=min
            max=max
            nostrip:step=step
            disabled=disabled
            nostrip:on_change=on_change
            classes=classes.add("leptonic-slider")
            styles
            attr:data-variant=variant
        >
            <SliderTrack classes=Classes::from("track")>
                <SliderTrackFill classes=Classes::from("fill") />
                <For
                    each=move || 0..num_thumbs
                    key=move |thumb_idx| *thumb_idx
                    children=move |thumb_idx| view! {
                        <SliderThumb index=thumb_idx classes=Classes::from("thumb")>
                            <SliderThumbTooltip
                                popover=popover
                                nostrip:value_display=value_display
                                classes=Classes::from("tooltip")
                            />
                        </SliderThumb>
                    }
                />
            </SliderTrack>
            <SliderMarks
                marks=marks
                nostrip:value_display=value_display
                classes=Classes::from("marks")
                let:marks
            >
                <For
                    each=move || marks.get()
                    key=move |mark| OrderedFloat::from(mark.percentage)
                    children=move |mark| view! {
                        <SliderMark mark=mark.clone() classes=Classes::from("mark")>
                            {match mark.name {
                                Some(name) => {
                                    view! { <div class="title">{name}</div> }.into_any()
                                }
                                None => ().into_any(),
                            }}
                        </SliderMark>
                    }
                />
            </SliderMarks>
        </SliderAtom>
    }
}

/// A single-thumb slider component that composes the slider atoms.
///
/// Renders a `<div class="leptonic-slider">` with a bar, range fill, knob,
/// and optional marks. Full keyboard navigation is built in.
#[component]
pub fn Slider(
    #[prop(into)] value: Signal<f64>,
    #[prop(into)] set_value: Out<f64>,
    min: f64,
    max: f64,
    #[prop(optional)] step: Option<f64>,
    #[prop(optional)] variant: SliderVariant,
    #[prop(optional)] popover: SliderPopover,
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(optional)] marks: SliderMarks,
    #[prop(into, optional)] value_display: Option<Callback<f64, String>>,
    #[prop(optional)] classes: Classes,
    #[prop(optional)] styles: Styles,
) -> impl IntoView {
    let values = Signal::derive(move || vec![value.get()]);
    let on_change = Callback::new(move |vals: Vec<f64>| {
        if let Some(v) = vals.first() {
            set_value.set(*v);
        }
    });

    view! {
        <SliderInner
            values=SliderValues::Controlled(values)
            min
            max
            step
            disabled
            on_change=Some(on_change)
            variant=variant.to_str()
            marks
            value_display
            popover
            num_thumbs=1
            classes
            styles
        />
    }
}

/// A two-thumb range slider component.
///
/// Renders a `<div class="leptonic-slider">` with two knobs allowing selection
/// of a value range. Thumbs cannot cross each other.
#[component]
#[allow(clippy::similar_names)]
pub fn RangeSlider(
    #[prop(into)] value_a: Signal<f64>,
    #[prop(into)] value_b: Signal<f64>,
    #[prop(into)] set_value_a: Out<f64>,
    #[prop(into)] set_value_b: Out<f64>,
    min: f64,
    max: f64,
    #[prop(optional)] step: Option<f64>,
    #[prop(optional)] variant: SliderVariant,
    #[prop(optional)] popover: SliderPopover,
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(optional)] marks: SliderMarks,
    #[prop(into, optional)] value_display: Option<Callback<f64, String>>,
    #[prop(optional)] classes: Classes,
    #[prop(optional)] styles: Styles,
) -> impl IntoView {
    let values = Signal::derive(move || vec![value_a.get(), value_b.get()]);
    let on_change = Callback::new(move |vals: Vec<f64>| {
        if let Some(v) = vals.first() {
            set_value_a.set(*v);
        }
        if let Some(v) = vals.get(1) {
            set_value_b.set(*v);
        }
    });

    view! {
        <SliderInner
            values=SliderValues::Controlled(values)
            min
            max
            step
            disabled
            on_change=Some(on_change)
            variant=variant.to_str()
            marks
            value_display
            popover
            num_thumbs=2
            classes
            styles
        />
    }
}
