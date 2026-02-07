use crate::atoms::focus_ring::FocusRing;
use crate::hooks::*;
use crate::utils::classes::Classes;
use crate::utils::styles::{
    Style::{Bottom, Height, Left, Position, Top, Transform, Width},
    Styles,
};
use crate::utils::CapturedElement;
use leptos::prelude::*;
use std::borrow::Cow;
use std::sync::atomic::AtomicUsize;
use std::sync::Arc;

#[derive(Debug, Clone)]
struct SliderCtx {
    state: UseSliderStateReturn,

    _label_props: UseSliderLabelProps,
    output_props: UseSliderOutputProps,
    track_props: UseSliderTrackProps,
    track: CapturedElement,

    is_rtl: bool,

    next_thumb_idx: Arc<AtomicUsize>,
}

impl SliderCtx {
    pub fn next_thumb_idx(&self) -> usize {
        self.next_thumb_idx
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst)
    }
}

#[component]
pub fn Slider(
    #[prop(into)] default_values: Vec<f64>,
    #[prop(into, optional, default = 0.0)] min: f64,
    #[prop(into, optional, default = 100.0)] max: f64,
    #[prop(into, optional, default = 1.0)] step: f64,
    #[prop(into, optional)] orientation: Signal<SliderOrientation>,
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(into, optional)] on_change: Option<Callback<Vec<f64>>>,
    #[prop(into, optional)] on_change_end: Option<Callback<Vec<f64>>>,
    #[prop(into, optional)] aria_label: Option<&'static str>,
    #[prop(into, optional)] aria_labelledby: Option<String>,
    #[prop(into, optional)] is_rtl: bool,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let state = use_slider_state(UseSliderStateInput {
        default_values,
        min_value: min,
        max_value: max,
        step,
        disabled,
        orientation,
        on_change,
        on_change_end,
    });

    let UseSliderReturn {
        group_props,
        track_props,
        track_ref,
        label_props,
        output_props,
    } = use_slider(UseSliderInput {
        state,
        aria_label,
        aria_labelledby,
        is_rtl,
    });

    provide_context(SliderCtx {
        state,
        _label_props: label_props,
        output_props,
        track_props,
        track: track_ref,
        is_rtl,
        next_thumb_idx: Arc::new(AtomicUsize::new(0)),
    });

    view! {
        <div {..group_props.into_attrs()} class=classes style=styles>
            {children()}
        </div>
    }
}

#[component(transparent)]
pub fn SliderOutput<C, V>(children: C) -> impl IntoView
where
    C: Fn(UseSliderOutputProps, Signal<Vec<f64>>) -> V + 'static,
    V: IntoView + 'static,
{
    let ctx = expect_context::<SliderCtx>();
    children(ctx.output_props.clone(), ctx.state.values)
}

#[component]
pub fn SliderTrack(
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let ctx = expect_context::<SliderCtx>();
    let track_attrs = ctx.track_props.into_attrs();
    view! {
        <div {..track_attrs} class=classes style=styles>
            {children()}
        </div>
    }
}

#[component]
#[allow(clippy::similar_names)]
pub fn SliderTrackFill(
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
) -> impl IntoView {
    let ctx = expect_context::<SliderCtx>();
    let values = ctx.state.values;

    match ctx.state.num_thumbs {
        1 => {
            let percentage = Signal::derive(move || values.get().first().copied().unwrap_or(0.0));
            let styles = styles
                .add((Position, "absolute"))
                .add((Left, "0"))
                .add((Top, move || match ctx.state.orientation.get() {
                    SliderOrientation::Horizontal => Some("0".into()),
                    SliderOrientation::Vertical => None,
                }))
                .add((Bottom, move || match ctx.state.orientation.get() {
                    SliderOrientation::Horizontal => None,
                    SliderOrientation::Vertical => Some("0".into()),
                }))
                .add((Height, move || match ctx.state.orientation.get() {
                    SliderOrientation::Horizontal => Some("100%".into()),
                    SliderOrientation::Vertical => Some(format!("{}%", percentage.get())),
                }))
                .add((Width, move || match ctx.state.orientation.get() {
                    SliderOrientation::Horizontal => Some(format!("{}%", percentage.get())),
                    SliderOrientation::Vertical => Some("100%".into()),
                }));
            view! { <div class=classes style=styles /> }
            .into_any()
        }
        2 => {
            let first_percentage =
                Signal::derive(move || values.get().first().copied().unwrap_or(0.0));
            let difference = Signal::derive(move || {
                let values = values.get();
                let v1 = values.first().copied().unwrap_or(0.0);
                let v2 = values.get(1).copied().unwrap_or(0.0);
                v2 - v1
            });
            let styles = styles
                .add((Position, "absolute"))
                .add((Top, move || match ctx.state.orientation.get() {
                    SliderOrientation::Horizontal => Some("0".into()),
                    SliderOrientation::Vertical => None,
                }))
                .add((Bottom, move || match ctx.state.orientation.get() {
                    SliderOrientation::Horizontal => None,
                    SliderOrientation::Vertical => Some(format!("{}%", first_percentage.get())),
                }))
                .add((Left, move || match ctx.state.orientation.get() {
                    SliderOrientation::Horizontal => Some(format!("{}%", first_percentage.get())),
                    SliderOrientation::Vertical => Some("0".into()),
                }))
                .add((Height, move || match ctx.state.orientation.get() {
                    SliderOrientation::Horizontal => Some("100%".into()),
                    SliderOrientation::Vertical => Some(format!("{}%", difference.get())),
                }))
                .add((Width, move || match ctx.state.orientation.get() {
                    SliderOrientation::Horizontal => Some(format!("{}%", difference.get())),
                    SliderOrientation::Vertical => Some("100%".into()),
                }));
            view! { <div class=classes style=styles /> }
            .into_any()
        }
        n => {
            tracing::warn!("SliderTrackFill: {n}-thumb fill not yet supported");
            ().into_any()
        }
    }
}

#[component]
pub fn SliderThumb(
    #[prop(into, optional)] index: Option<usize>,
    #[prop(into, optional)] name: Option<&'static str>,
    #[prop(into, optional)] aria_label: Option<Cow<'static, str>>,
    #[prop(into, optional)] aria_labelledby: Option<&'static str>,
    #[prop(into, optional)] aria_describedby: Option<&'static str>,
    #[prop(into, optional)] aria_details: Option<&'static str>,
    #[prop(into, optional)] aria_errormessage: Option<&'static str>,
    #[prop(into, optional)] validation_state: Option<ValidationState>,
    #[prop(into, optional)] decimal_places: Option<usize>,
    #[prop(into, optional)] is_required: bool,
    #[prop(into, optional)] is_rtl: Option<bool>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
) -> impl IntoView {
    let ctx = expect_context::<SliderCtx>();

    let thumb_index = index.unwrap_or_else(|| ctx.next_thumb_idx());

    let UseSliderThumbReturn {
        thumb_props,
        input_props,
        is_dragging,
        is_focused: _,
        is_focus_visible: _,
        percentage,
        value: _,
        display_value: _,
        thumb_id: _,
    } = use_slider_thumb(UseSliderThumbInput {
        state: ctx.state,
        track: ctx.track,
        index: thumb_index,
        name,
        aria_label,
        aria_labelledby,
        disabled: ctx.state.disabled,
        validation_state: validation_state.unwrap_or(ValidationState::Valid),
        is_rtl: is_rtl.unwrap_or(ctx.is_rtl),
        decimal_places,
        is_required,
        aria_describedby,
        aria_details,
        aria_errormessage,
    });

    let data_dragging = Signal::derive(move || {
        if is_dragging.get() {
            Some("true")
        } else {
            None
        }
    });

    let styles = styles
        .add((Position, "absolute"))
        .add((Top, move || match ctx.state.orientation.get() {
            SliderOrientation::Horizontal => Some("50%".into()),
            SliderOrientation::Vertical => None,
        }))
        .add((Left, move || match ctx.state.orientation.get() {
            SliderOrientation::Horizontal => Some(format!("{}%", percentage.get())),
            SliderOrientation::Vertical => Some("50%".into()),
        }))
        .add((Bottom, move || match ctx.state.orientation.get() {
            SliderOrientation::Horizontal => None,
            SliderOrientation::Vertical => Some(format!("{}%", percentage.get())),
        }))
        .add((Transform, move || match ctx.state.orientation.get() {
            SliderOrientation::Horizontal => Some("translate(-50%, -50%)".into()),
            SliderOrientation::Vertical => Some("translate(-50%, 50%)".into()),
        }));

    view! {
        <FocusRing>
            <div {..thumb_props} class=classes style=styles attr:data-dragging=data_dragging>
                <input {..input_props.into_attrs()} />
            </div>
        </FocusRing>
    }
}
