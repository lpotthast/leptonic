use crate::hooks::*;
use crate::utils::classes::Classes;
use crate::utils::styles::{
    Style::{Bottom, Height, Left, Position, Top, Transform, Width},
    Styles,
};
use crate::utils::CapturedElement;
use leptos::context::Provider;
use leptos::prelude::*;
use std::borrow::Cow;
use std::sync::atomic::AtomicUsize;
use std::sync::Arc;

/// Controls when the slider thumb tooltip is displayed.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum SliderPopover {
    /// Never show the tooltip.
    #[default]
    Never,
    /// Show the tooltip conditionally based on hover and drag state.
    When {
        /// Show the tooltip when the thumb is hovered.
        hovered: bool,
        /// Show the tooltip when the thumb is being dragged.
        dragged: bool,
    },
    /// Always show the tooltip.
    Always,
}

#[derive(Debug, Clone)]
pub(crate) struct SliderCtx {
    pub(crate) state: UseSliderStateReturn,

    pub(crate) output_attrs: UseSliderOutputAttrs,
    pub(crate) track_attrs: UseSliderTrackAttrs,
    pub(crate) track: CapturedElement,

    pub(crate) is_rtl: bool,

    pub(crate) next_thumb_idx: Arc<AtomicUsize>,
}

impl SliderCtx {
    pub fn next_thumb_idx(&self) -> usize {
        self.next_thumb_idx
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst)
    }
}

#[component]
pub fn Slider(
    #[prop(into)] values: SliderValues,
    #[prop(into, optional, default = 0.0)] min: f64,
    #[prop(into, optional, default = 100.0)] max: f64,
    #[prop(optional, default = Some(1.0))] step: Option<f64>,
    #[prop(into, optional)] orientation: Signal<SliderOrientation>,
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(optional)] on_change: Option<Callback<Vec<f64>>>,
    #[prop(optional)] on_change_end: Option<Callback<Vec<f64>>>,
    #[prop(into, optional)] aria_label: Option<&'static str>,
    #[prop(into, optional)] aria_labelledby: Option<String>,
    #[prop(into, optional)] is_rtl: bool,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let state = use_slider_state(UseSliderStateInput {
        values,
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
        label_props: _,
        output_props,
    } = use_slider(UseSliderInput {
        state,
        aria_label,
        aria_labelledby,
        is_rtl,
    });

    let ctx = SliderCtx {
        state,
        output_attrs: output_props.into_attrs(),
        track_attrs: track_props.into_attrs(),
        track: track_ref,
        is_rtl,
        next_thumb_idx: Arc::new(AtomicUsize::new(0)),
    };

    view! {
        <Provider value=ctx>
            <div {..group_props.into_attrs()} class=classes style=styles>
                {children()}
            </div>
        </Provider>
    }
}

#[component(transparent)]
pub fn SliderOutput<C, V>(children: C) -> impl IntoView
where
    C: Fn(UseSliderOutputAttrs, Signal<Vec<f64>>) -> V + 'static,
    V: IntoView + 'static,
{
    let ctx = expect_context::<SliderCtx>();
    children(ctx.output_attrs.clone(), ctx.state.values)
}

#[component]
pub fn SliderTrack(
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let ctx = expect_context::<SliderCtx>();
    view! {
        <div {..ctx.track_attrs.clone()} class=classes style=styles>
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
    let state = ctx.state;
    let values = state.values;

    match state.num_thumbs {
        1 => {
            let percentage = Signal::derive(move || {
                let val = values.get().first().copied().unwrap_or(state.min_value);
                state.get_value_percent.run(val) * 100.0
            });
            let styles = styles
                .add((Position, "absolute"))
                .add((Left, "0"))
                .add((Top, move || match state.orientation.get() {
                    SliderOrientation::Horizontal => Some("0".into()),
                    SliderOrientation::Vertical => None,
                }))
                .add((Bottom, move || match state.orientation.get() {
                    SliderOrientation::Horizontal => None,
                    SliderOrientation::Vertical => Some("0".into()),
                }))
                .add((Height, move || match state.orientation.get() {
                    SliderOrientation::Horizontal => Some("100%".into()),
                    SliderOrientation::Vertical => Some(format!("{}%", percentage.get())),
                }))
                .add((Width, move || match state.orientation.get() {
                    SliderOrientation::Horizontal => Some(format!("{}%", percentage.get())),
                    SliderOrientation::Vertical => Some("100%".into()),
                }));
            view! { <div class=classes style=styles /> }.into_any()
        }
        2 => {
            let first_percentage = Signal::derive(move || {
                let val = values.get().first().copied().unwrap_or(state.min_value);
                state.get_value_percent.run(val) * 100.0
            });
            let difference = Signal::derive(move || {
                let vals = values.get();
                let v1 = vals.first().copied().unwrap_or(state.min_value);
                let v2 = vals.get(1).copied().unwrap_or(state.min_value);
                ((state.get_value_percent.run(v2) - state.get_value_percent.run(v1)) * 100.0)
                    .max(0.0)
            });
            let styles = styles
                .add((Position, "absolute"))
                .add((Top, move || match state.orientation.get() {
                    SliderOrientation::Horizontal => Some("0".into()),
                    SliderOrientation::Vertical => None,
                }))
                .add((Bottom, move || match state.orientation.get() {
                    SliderOrientation::Horizontal => None,
                    SliderOrientation::Vertical => Some(format!("{}%", first_percentage.get())),
                }))
                .add((Left, move || match state.orientation.get() {
                    SliderOrientation::Horizontal => Some(format!("{}%", first_percentage.get())),
                    SliderOrientation::Vertical => Some("0".into()),
                }))
                .add((Height, move || match state.orientation.get() {
                    SliderOrientation::Horizontal => Some("100%".into()),
                    SliderOrientation::Vertical => Some(format!("{}%", difference.get())),
                }))
                .add((Width, move || match state.orientation.get() {
                    SliderOrientation::Horizontal => Some(format!("{}%", difference.get())),
                    SliderOrientation::Vertical => Some("100%".into()),
                }));
            view! { <div class=classes style=styles /> }.into_any()
        }
        n => {
            tracing::warn!("SliderTrackFill: {n}-thumb fill not yet supported");
            ().into_any()
        }
    }
}

/// Context provided by `SliderThumb` to its children, exposing the thumb's
/// interaction state and value so that child atoms (e.g. `SliderThumbTooltip`)
/// can render based on thumb state.
#[derive(Debug, Clone, Copy)]
pub struct SliderThumbCtx {
    /// Whether the pointer is currently hovering over the thumb.
    pub is_hovered: Signal<bool>,
    /// Whether the thumb is currently being dragged.
    pub is_dragging: Signal<bool>,
    /// The current value of this thumb.
    pub value: Signal<f64>,
    /// The formatted display value (respects `decimal_places`).
    pub display_value: Signal<String>,
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
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let ctx = expect_context::<SliderCtx>();

    let thumb_index = index.unwrap_or_else(|| ctx.next_thumb_idx());

    let UseSliderThumbReturn {
        thumb_props,
        input_props,
        is_dragging,
        is_hovered,
        is_focused: _,
        is_focus_visible: _,
        percentage,
        value,
        display_value,
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

    let thumb_ctx = SliderThumbCtx {
        is_hovered,
        is_dragging,
        value,
        display_value,
    };

    view! {
        <Provider value=thumb_ctx>
            <div
                {..thumb_props.into_attrs()}
                class=classes
                style=styles
                attr:data-dragging=data_dragging
            >
                <input {..input_props.into_attrs()} />

                {children.map(|c| c())}
            </div>
        </Provider>
    }
}

/// Atom component that renders a tooltip above the slider thumb.
/// Expects to be used as a child of `SliderThumb` (requires `SliderThumbCtx`).
///
/// Renders a single `<div>` whose visibility is controlled by the `popover`
/// configuration via a `data-visible` attribute. CSS should hide the element
/// when `data-visible` is absent.
#[component]
pub fn SliderThumbTooltip(
    #[prop(optional)] popover: SliderPopover,
    #[prop(into, optional)] value_display: Option<Callback<f64, String>>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
) -> impl IntoView {
    let ctx = expect_context::<SliderThumbCtx>();

    let data_visible = Signal::derive(move || {
        let visible = match popover {
            SliderPopover::Never => false,
            SliderPopover::When { hovered, dragged } => {
                (hovered && ctx.is_hovered.get()) || (dragged && ctx.is_dragging.get())
            }
            SliderPopover::Always => true,
        };
        visible.then_some("")
    });

    let text = Signal::derive(move || match value_display {
        Some(cb) => cb.run(ctx.value.get()),
        None => ctx.display_value.get(),
    });

    view! {
        <div class=classes style=styles attr:data-visible=data_visible>
            {move || text.get()}
        </div>
    }
}

/// Atom component that renders slider marks.
/// Expects to be used within a `Slider` atom (requires `SliderCtx`).
#[component]
pub fn SliderMarks<C, V>(
    #[prop(into)] marks: SliderMarks,
    #[prop(optional)] value_display: Option<Callback<f64, String>>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: C,
) -> impl IntoView
where
    C: Fn(Signal<Vec<ComputedSliderMark>>) -> V,
    V: IntoView,
{
    let ctx = expect_context::<SliderCtx>();

    let computed = use_slider_marks(UseSliderMarksInput {
        state: ctx.state,
        marks,
        value_display,
    });

    view! {
        <div class=classes style=styles>
            { children(computed.marks) }
        </div>
    }
}

/// NOTE: Ignores the `mark`s `name`. It must be rendered manually as children.
#[component]
pub fn SliderMark(
    #[prop(into)] mark: ComputedSliderMark,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    // TODO: We could use a data attribute instead.
    let classes = classes.add(("in-range", mark.in_range));

    let styles = styles
        .add((Position, "absolute"))
        .add((Left, format!("{}%", mark.percentage * 100.0)));

    view! {
        <div
            class=classes
            style=styles
        >
            { children() }
        </div>
    }
}
