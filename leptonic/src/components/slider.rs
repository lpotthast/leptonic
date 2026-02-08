use leptos::html;
use leptos::prelude::*;
use leptos_use::{use_element_bounding, use_element_hover};

use crate::{
    atoms::slider::SliderCtx,
    components::popover::{Popover, PopoverContent},
    hooks::{
        use_slider_marks, use_slider_state, use_slider_thumb, SliderValues, UseSliderMarksInput,
        UseSliderStateInput, UseSliderThumbInput, ValidationState,
    },
    Out, Size, UseElementBoundingReturnReadOnly,
};

// Re-export mark types from hooks for backward compatibility.
pub use crate::hooks::{SliderMark, SliderMarkValue, SliderMarks};

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

#[derive(Debug, Clone, Copy)]
pub enum SliderPopover {
    Never,
    When { hovered: bool, dragged: bool },
    Always,
}

impl Default for SliderPopover {
    fn default() -> Self {
        Self::When {
            hovered: true,
            dragged: true,
        }
    }
}

impl SliderPopover {
    fn to_signal(self, is_dragging: Signal<bool>, is_hovered: Signal<bool>) -> Signal<bool> {
        match self {
            Self::Never => Signal::from(false),
            Self::When { hovered, dragged } => match (hovered, dragged) {
                (true, true) => {
                    Signal::derive(move || is_hovered.get() || is_dragging.get())
                }
                (true, false) => is_hovered,
                (false, true) => is_dragging,
                (false, false) => Signal::from(false),
            },
            Self::Always => Signal::from(true),
        }
    }
}

/// A single-thumb slider component that uses the hooks/atoms infrastructure.
///
/// Renders a `<leptonic-slider>` custom element with a bar, range fill, knob, optional popover,
/// and optional marks. Full keyboard navigation is built in.
#[component]
#[allow(clippy::too_many_lines)]
pub fn Slider(
    #[prop(into)] value: Signal<f64>,
    #[prop(into)] set_value: Out<f64>,
    min: f64,
    max: f64,
    #[prop(optional)] step: Option<f64>,
    #[prop(optional)] variant: SliderVariant,
    #[prop(optional)] popover: SliderPopover,
    #[prop(optional)] active: bool,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] marks: SliderMarks,
    #[prop(into, optional)] value_display: Option<Callback<f64, String>>,
) -> impl IntoView {
    // Create controlled values signal from the single value prop.
    let values = Signal::derive(move || vec![value.get()]);

    // Bridge on_change to the single-value set_value.
    let on_change = Callback::new(move |vals: Vec<f64>| {
        if let Some(v) = vals.first() {
            set_value.set(*v);
        }
    });

    let state = use_slider_state(UseSliderStateInput {
        values: SliderValues::Controlled(values),
        min_value: min,
        max_value: max,
        step,
        disabled: Signal::from(disabled),
        orientation: Signal::default(),
        on_change: Some(on_change),
        on_change_end: None,
    });

    let crate::hooks::UseSliderReturn {
        group_props: _,
        track_props,
        track_ref,
        label_props: _,
        output_props: _,
    } = crate::hooks::use_slider(crate::hooks::UseSliderInput {
        state,
        aria_label: None,
        aria_labelledby: None,
        is_rtl: false,
    });

    // Provide context for the inner thumb component.
    provide_context(SliderCtx {
        state,
        _label_props: crate::hooks::UseSliderLabelProps {
            id: String::new(),
        },
        output_props: crate::hooks::UseSliderOutputProps {
            id: String::new(),
            html_for: String::new(),
            aria_live: "off",
        },
        track_props,
        track: track_ref,
        is_rtl: false,
        next_thumb_idx: std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0)),
    });

    // Compute marks.
    let marks_computed = use_slider_marks(UseSliderMarksInput {
        state,
        marks,
        value_display,
    });

    view! {
        <leptonic-slider
            data-variant=variant.to_str()
            class:active=active
            class:disabled=disabled
        >
            <div class="bar-wrapper">
                <ComponentSliderTrack
                    popover=popover
                    value_display=value_display
                />
            </div>

            <ComponentMarks marks=marks_computed.marks />
        </leptonic-slider>
    }
}

/// A two-thumb range slider component.
///
/// Renders a `<leptonic-slider>` custom element with two knobs allowing selection
/// of a value range. Thumbs cannot cross each other.
#[component]
#[allow(clippy::similar_names)]
#[allow(clippy::too_many_lines)]
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
    #[prop(optional)] active: bool,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] marks: SliderMarks,
    #[prop(into, optional)] value_display: Option<Callback<f64, String>>,
) -> impl IntoView {
    // Create controlled values signal from both value props.
    let values = Signal::derive(move || vec![value_a.get(), value_b.get()]);

    // Bridge on_change to both setters.
    let on_change = Callback::new(move |vals: Vec<f64>| {
        if let Some(v) = vals.first() {
            set_value_a.set(*v);
        }
        if let Some(v) = vals.get(1) {
            set_value_b.set(*v);
        }
    });

    let state = use_slider_state(UseSliderStateInput {
        values: SliderValues::Controlled(values),
        min_value: min,
        max_value: max,
        step,
        disabled: Signal::from(disabled),
        orientation: Signal::default(),
        on_change: Some(on_change),
        on_change_end: None,
    });

    let crate::hooks::UseSliderReturn {
        group_props: _,
        track_props,
        track_ref,
        label_props: _,
        output_props: _,
    } = crate::hooks::use_slider(crate::hooks::UseSliderInput {
        state,
        aria_label: None,
        aria_labelledby: None,
        is_rtl: false,
    });

    // Provide context for inner thumb components.
    provide_context(SliderCtx {
        state,
        _label_props: crate::hooks::UseSliderLabelProps {
            id: String::new(),
        },
        output_props: crate::hooks::UseSliderOutputProps {
            id: String::new(),
            html_for: String::new(),
            aria_live: "off",
        },
        track_props,
        track: track_ref,
        is_rtl: false,
        next_thumb_idx: std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0)),
    });

    // Compute marks.
    let marks_computed = use_slider_marks(UseSliderMarksInput {
        state,
        marks,
        value_display,
    });

    view! {
        <leptonic-slider
            data-variant=variant.to_str()
            class:active=active
            class:disabled=disabled
        >
            <div class="bar-wrapper">
                <ComponentRangeSliderTrack
                    popover=popover
                    value_display=value_display
                />
            </div>

            <ComponentMarks marks=marks_computed.marks />
        </leptonic-slider>
    }
}

/// Internal: renders the track with a single thumb + popover.
#[component]
fn ComponentSliderTrack(
    popover: SliderPopover,
    value_display: Option<Callback<f64, String>>,
) -> impl IntoView {
    let ctx = expect_context::<SliderCtx>();
    let track_attrs = ctx.track_props.into_attrs();

    let thumb = use_slider_thumb(UseSliderThumbInput {
        state: ctx.state,
        track: ctx.track,
        index: 0,
        name: None,
        aria_label: None,
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

    let percentage = thumb.percentage;
    let is_dragging = thumb.is_dragging;
    let thumb_value = thumb.value;

    let range_style = Signal::derive(move || {
        format!(
            "left: 0%; width: {}%;",
            percentage.get()
        )
    });

    let knob_style = Signal::derive(move || {
        format!("left: {}%", percentage.get())
    });

    // Popover: determine show signal.
    let knob_el: NodeRef<html::Div> = NodeRef::new();
    let knob_is_hovered = use_element_hover(knob_el);
    let show_popover = popover.to_signal(
        is_dragging,
        knob_is_hovered,
    );

    // Popover position.
    let bar_el: NodeRef<html::Div> = NodeRef::new();
    let bar_bounds = use_element_bounding(bar_el);
    let pos_x = Callback::new(move |pop_bounds: UseElementBoundingReturnReadOnly| {
        format!(
            "calc({}px + {}px - {}px)",
            bar_bounds.x.get(),
            (percentage.get() / 100.0 * bar_bounds.width.get()),
            (pop_bounds.width.get() / 2.0)
        )
    });

    view! {
        <div {..track_attrs} node_ref=bar_el class="bar">
            <div class="range" style=move || range_style.get()></div>

            <div class="knob-wrapper">
                <Popover show=show_popover position_x=pos_x margin=Size::Em(1.2)>
                    <PopoverContent slot>
                        {move || {
                            let v = thumb_value.get();
                            match &value_display {
                                Some(callback) => callback.run(v),
                                None => format!("{v}"),
                            }
                        }}
                    </PopoverContent>

                    <div
                        node_ref=knob_el
                        class="knob"
                        class:is-dragged=move || is_dragging.get()
                        {..thumb.thumb_props}
                        style=move || knob_style.get()
                    />
                </Popover>
            </div>
            <input {..thumb.input_props.into_attrs()} />
        </div>
    }
}

/// Internal: renders the track with two thumbs + popovers for a range slider.
#[component]
#[allow(clippy::similar_names)]
fn ComponentRangeSliderTrack(
    popover: SliderPopover,
    value_display: Option<Callback<f64, String>>,
) -> impl IntoView {
    let ctx = expect_context::<SliderCtx>();
    let track_attrs = ctx.track_props.into_attrs();

    let thumb_a = use_slider_thumb(UseSliderThumbInput {
        state: ctx.state,
        track: ctx.track,
        index: 0,
        name: None,
        aria_label: None,
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

    let thumb_b = use_slider_thumb(UseSliderThumbInput {
        state: ctx.state,
        track: ctx.track,
        index: 1,
        name: None,
        aria_label: None,
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

    let percentage_a = thumb_a.percentage;
    let percentage_b = thumb_b.percentage;
    let is_dragging_a = thumb_a.is_dragging;
    let is_dragging_b = thumb_b.is_dragging;
    let value_a = thumb_a.value;
    let value_b = thumb_b.value;

    let range_style = Signal::derive(move || {
        format!(
            "left: {}%; width: {}%;",
            percentage_a.get(),
            (percentage_b.get() - percentage_a.get()).max(0.0)
        )
    });

    let knob_a_style = Signal::derive(move || {
        format!("left: {}%", percentage_a.get())
    });

    let knob_b_style = Signal::derive(move || {
        format!("left: {}%", percentage_b.get())
    });

    // Popovers for knob A.
    let knob_a_el: NodeRef<html::Div> = NodeRef::new();
    let knob_a_hovered = use_element_hover(knob_a_el);
    let show_a_popover = popover.to_signal(is_dragging_a, knob_a_hovered);

    // Popovers for knob B.
    let knob_b_el: NodeRef<html::Div> = NodeRef::new();
    let knob_b_hovered = use_element_hover(knob_b_el);
    let show_b_popover = popover.to_signal(is_dragging_b, knob_b_hovered);

    // Popover positions.
    let bar_el: NodeRef<html::Div> = NodeRef::new();
    let bar_bounds = use_element_bounding(bar_el);
    let knob_a_pos_x = Callback::new(move |pop_bounds: UseElementBoundingReturnReadOnly| {
        format!(
            "calc({}px + {}px - {}px)",
            bar_bounds.x.get(),
            (percentage_a.get() / 100.0 * bar_bounds.width.get()),
            (pop_bounds.width.get() / 2.0)
        )
    });
    let knob_b_pos_x = Callback::new(move |pop_bounds: UseElementBoundingReturnReadOnly| {
        format!(
            "calc({}px + {}px - {}px)",
            bar_bounds.x.get(),
            (percentage_b.get() / 100.0 * bar_bounds.width.get()),
            (pop_bounds.width.get() / 2.0)
        )
    });

    let value_display_a = value_display;
    let value_display_b = value_display;

    view! {
        <div {..track_attrs} node_ref=bar_el class="bar">
            <div class="knob-wrapper">
                <Popover show=show_a_popover position_x=knob_a_pos_x margin=Size::Em(1.2)>
                    <PopoverContent slot>
                        {move || {
                            let v = value_a.get();
                            match &value_display_a {
                                Some(callback) => callback.run(v),
                                None => format!("{v}"),
                            }
                        }}
                    </PopoverContent>

                    <div
                        node_ref=knob_a_el
                        class="knob"
                        class:is-dragged=move || is_dragging_a.get()
                        {..thumb_a.thumb_props}
                        style=move || knob_a_style.get()
                    />
                </Popover>
            </div>
            <div class="range" style=move || range_style.get()></div>
            <div class="knob-wrapper">
                <Popover show=show_b_popover position_x=knob_b_pos_x margin=Size::Em(1.2)>
                    <PopoverContent slot>
                        {move || {
                            let v = value_b.get();
                            match &value_display_b {
                                Some(callback) => callback.run(v),
                                None => format!("{v}"),
                            }
                        }}
                    </PopoverContent>

                    <div
                        node_ref=knob_b_el
                        class="knob"
                        class:is-dragged=move || is_dragging_b.get()
                        {..thumb_b.thumb_props}
                        style=move || knob_b_style.get()
                    />
                </Popover>
            </div>
            <input {..thumb_a.input_props.into_attrs()} />
            <input {..thumb_b.input_props.into_attrs()} />
        </div>
    }
}

/// Internal: renders marks.
#[component]
fn ComponentMarks(marks: Signal<Vec<crate::hooks::ComputedSliderMark>>) -> impl IntoView {
    view! {
        <div class="marks">
            {move || {
                marks
                    .get()
                    .into_iter()
                    .map(|mark| {
                        let style = format!("left: {}%", mark.percentage * 100.0);
                        let in_range = mark.in_range;
                        view! {
                            <div
                                class="mark"
                                class:in-range=move || in_range.get()
                                style=style
                            >
                                {match &mark.name {
                                    Some(name) => {
                                        view! { <div class="title">{name.clone()}</div> }.into_any()
                                    }
                                    None => ().into_any(),
                                }}
                            </div>
                        }
                    })
                    .collect_view()
            }}
        </div>
    }
}
