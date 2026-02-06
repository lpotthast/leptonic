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
        on_change: None,
        on_change_end: None,
    });

    let UseSliderReturn {
        group_props,
        track_props,
        track_ref,
        label_props,
        output_props,
    } = use_slider(UseSliderInput {
        state,
        is_rtl: false,
        aria_label: None,
        aria_labelledby: None,
    });

    provide_context(SliderCtx {
        state,
        _label_props: label_props,
        output_props,
        track_props,
        track: track_ref,
        next_thumb_idx: Arc::new(AtomicUsize::new(0)),
    });

    view! {
        <div {..group_props.into_attrs()} class=classes style=styles>
            { children() }

            // Hidden input for form submission
            //<input {..input_props} />
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
            { children() }
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
    move || {
        let values = ctx.state.values.get();
        let styles = styles.clone();
        match values.len() {
            1 => {
                let percentage = values[0];
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
                        SliderOrientation::Vertical => Some(format!("{percentage}%")),
                    }))
                    .add((Width, move || match ctx.state.orientation.get() {
                        SliderOrientation::Horizontal => Some(format!("{percentage}%")),
                        SliderOrientation::Vertical => Some("100%".into()),
                    }));
                view! {
                    // Left fill for single thumb.
                    //  background: var(--brand-color); border-radius: 4px;
                    <div class=classes.clone() style=styles/>
                }
                .into_any()
            }
            2 => {
                let value1 = values[0];
                let value2 = values[1];
                let first_percentage = value1;
                let difference = value2 - value1;
                let styles = styles
                    .add((Position, "absolute"))
                    .add((Top, move || match ctx.state.orientation.get() {
                        SliderOrientation::Horizontal => Some("0".into()),
                        SliderOrientation::Vertical => None,
                    }))
                    .add((Bottom, move || match ctx.state.orientation.get() {
                        SliderOrientation::Horizontal => None,
                        SliderOrientation::Vertical => Some(format!("{first_percentage}%")),
                    }))
                    .add((Left, move || match ctx.state.orientation.get() {
                        SliderOrientation::Horizontal => Some(format!("{first_percentage}%")),
                        SliderOrientation::Vertical => Some("0".into()),
                    }))
                    .add((Height, move || match ctx.state.orientation.get() {
                        SliderOrientation::Horizontal => Some("100%".into()),
                        SliderOrientation::Vertical => Some(format!("{difference}%")),
                    }))
                    .add((Width, move || match ctx.state.orientation.get() {
                        SliderOrientation::Horizontal => Some(format!("{difference}%")),
                        SliderOrientation::Vertical => Some("100%".into()),
                    }));
                view! {
                    // Fill between for two thumbs.
                    // background: #4a90d9; border-radius: 4px;
                    <div class=classes.clone() style=styles/>
                }
                .into_any()
            }
            _ => ().into_any(),
        }
    }
}

#[component]
pub fn SliderThumb(
    #[prop(into, optional)] aria_label: Option<Cow<'static, str>>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
) -> impl IntoView {
    let ctx = expect_context::<SliderCtx>();

    let UseSliderThumbReturn {
        thumb_props,
        input_props,
        is_dragging: _,
        is_focused: _,
        is_focus_visible: _,
        percentage,
        value: _,
        display_value: _,
        thumb_id: _,
    } = use_slider_thumb(UseSliderThumbInput {
        state: ctx.state,
        track: ctx.track,
        index: ctx.next_thumb_idx(), // NOTE: Assumes that `<SliderThumb>`s are rendered in order of their appearance in the view! macro.
        name: None,
        aria_label,
        aria_labelledby: None,
        disabled: ctx.state.disabled,
        validation_state: ValidationState::Valid,
        is_rtl: false,
        decimal_places: None,
        is_required: false,
        aria_describedby: None,
        aria_details: None,
        aria_errormessage: None,
    });

    let styles = styles
        .add((Position, "absolute"))
        .add((Top, move || match ctx.state.orientation.get() {
            SliderOrientation::Horizontal => Some("50%".into()),
            SliderOrientation::Vertical => None,
        }))
        .add((Left, move || match ctx.state.orientation.get() {
            SliderOrientation::Horizontal => None,
            SliderOrientation::Vertical => Some("50%".into()),
        }))
        .add((Transform, move || match ctx.state.orientation.get() {
            SliderOrientation::Horizontal => Some("translate(-50%, -50%)".into()),
            SliderOrientation::Vertical => Some("translate(-50%, 50%)".into()),
        }))
        .add((Left, move || match ctx.state.orientation.get() {
            SliderOrientation::Horizontal => Some(format!("{left}%", left = percentage.get())),
            SliderOrientation::Vertical => None,
        }))
        .add((Bottom, move || match ctx.state.orientation.get() {
            SliderOrientation::Horizontal => None,
            SliderOrientation::Vertical => Some(format!("{bottom}%", bottom = percentage.get())),
        }));

    view! {
        <FocusRing>
            <div {..thumb_props} class=classes style=styles>
                <input {..input_props.into_attrs()}/>
            </div>
        </FocusRing>
    }
}
