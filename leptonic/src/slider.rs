use std::borrow::Cow;

use leptos::*;
use leptos_use::use_element_hover;

use crate::{
    contexts::global_mouseup_event::GlobalMouseupEvent,
    math::project_into_range,
    prelude::{Callable, Callback, Popover},
    Out, RelativeMousePosition, TrackedElementClientBoundingRect,
};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum SliderVariant {
    Block,
    #[default]
    Round,
}

impl SliderVariant {
    fn to_str(self) -> &'static str {
        match self {
            SliderVariant::Block => "block",
            SliderVariant::Round => "round",
        }
    }
}

#[derive(Default, Debug, Clone)]
pub enum SliderMarks {
    #[default]
    None,
    /// Note that marks can only be automatically generated if a step value is provided!
    Automatic {
        create_names: bool,
    },
    Custom(Vec<SliderMark>),
}

#[derive(Debug, Clone)]

pub struct SliderMark {
    value: SliderMarkValue,
    name: Option<Cow<'static, str>>,
}

#[derive(Debug, Clone, Copy)]
pub enum SliderMarkValue {
    Value(f64),
    /// In 0..1 range.
    Percentage(f64),
}

#[derive(Clone)]
struct Mark {
    percentage: f64,
    in_range: Signal<bool>,
    name: Option<Cow<'static, str>>,
}

#[allow(clippy::too_many_arguments)]
fn create_marks<F: Fn(f64) -> Signal<bool> + 'static>(
    min: f64,
    max: f64,
    step: Option<f64>,
    range: Memo<f64>,
    in_range: F,
    marks: SliderMarks,
    value_display: Option<Callback<f64, String>>,
) -> Signal<Vec<Mark>> {
    match marks {
        SliderMarks::None => Signal::derive(Vec::new),
        SliderMarks::Automatic { create_names } => Signal::derive(move || match step {
            Some(step) => {
                let mut marks_at = Vec::new();
                let cap = 20.0;
                let estimate = range.get() / step;
                let step_multiplier = f64::max(1.0, f64::round(estimate / cap));
                let mut current = min;

                let rounding_error_offset = 0.000001;
                loop {
                    if max > min {
                        if current > max + rounding_error_offset {
                            break;
                        }
                    } else if current < max - rounding_error_offset {
                        break;
                    }
                    marks_at.push(Mark {
                        percentage: crate::math::percentage_in_range(min, max, current),
                        in_range: in_range(current),
                        name: match create_names {
                            true => Some(Cow::Owned(match value_display {
                                Some(callback) => callback.call(current),
                                None => format!("{current}"),
                            })),
                            false => None,
                        },
                    });
                    if max > min {
                        if current <= max + rounding_error_offset {
                            current += step * step_multiplier;
                        }
                    } else if current >= max - rounding_error_offset {
                        current -= step * step_multiplier;
                    };
                }
                marks_at
            }
            None => Vec::new(),
        }),
        SliderMarks::Custom(marks) => Signal::derive(move || {
            marks
                .iter()
                .map(|mark| {
                    let value = match mark.value {
                        SliderMarkValue::Value(value) => value,
                        SliderMarkValue::Percentage(percentage) => {
                            crate::math::value_in_range(min, max, percentage)
                        }
                    };
                    Mark {
                        percentage: match mark.value {
                            SliderMarkValue::Value(value) => {
                                crate::math::percentage_in_range(min, max, value)
                            }
                            SliderMarkValue::Percentage(percentage) => percentage,
                        },
                        in_range: in_range(value),
                        name: mark.name.clone(),
                    }
                })
                .collect()
        }),
    }
}

#[component]
fn Marks(marks: Signal<Vec<Mark>>) -> impl IntoView {
    view! {
        <div class="marks">
            {
                move || marks.get().into_iter()
                    .map(|mark| {
                        let style = format!("left: {}%", mark.percentage * 100.0);
                        view! {
                            <div class="mark" class:in-range=move || mark.in_range.get() style=style>
                                { match &mark.name {
                                    Some(name) => view! {
                                        <div class="title">
                                            {name.clone()}
                                        </div>
                                    }.into_view(),
                                    None => ().into_view()
                                } }
                            </div>
                        }
                    })
                    .collect_view()
            }
        </div>
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
    fn to_maybe_signal(self, knob_el: NodeRef<html::Div>, knob: &KnobControl) -> MaybeSignal<bool> {
        match self {
            SliderPopover::Never => MaybeSignal::Static(false),
            SliderPopover::When { hovered, dragged } => match (hovered, dragged) {
                (true, true) => {
                    let knob_is_hovered = use_element_hover(knob_el);
                    let listening = knob.listening;
                    MaybeSignal::Dynamic(Signal::derive(move || {
                        knob_is_hovered.get() || listening.get()
                    }))
                }
                (true, false) => {
                    let knob_is_hovered = use_element_hover(knob_el);
                    MaybeSignal::Dynamic(knob_is_hovered)
                }
                (false, true) => MaybeSignal::Dynamic(knob.listening.into()),
                (false, false) => MaybeSignal::Static(false),
            },
            SliderPopover::Always => MaybeSignal::Static(true),
        }
    }
}

#[component]
pub fn Slider(
    #[prop(into)] value: MaybeSignal<f64>,
    #[prop(into)] set_value: Out<f64>,
    min: f64,
    max: f64,
    #[prop(optional)] step: Option<f64>,
    #[prop(optional)] variant: SliderVariant,
    #[prop(optional)] popover: SliderPopover,
    #[prop(optional)] active: bool,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] marks: SliderMarks,
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    #[prop(into, optional)] value_display: Option<Callback<f64, String>>,
) -> impl IntoView {
    let range = create_memo(move |_| max - min);

    let bar_el: NodeRef<html::Div> = create_node_ref();
    let bar = TrackedElementClientBoundingRect::new(bar_el);
    let cursor = RelativeMousePosition::new(bar);
    let knob_el: NodeRef<html::Div> = create_node_ref();
    let knob = KnobControl::new(min, max, step, value);
    let show_popover = popover.to_maybe_signal(knob_el, &knob);

    let range_style = Signal::derive(move || {
        format!(
            "left: 0%; width: {}%;",
            knob.clipped_value_percent.get() * 100.0
        )
    });

    // Stop listening whenever any mouseup event got fired.
    let GlobalMouseupEvent {
        read_signal: mouse_up,
        ..
    } = expect_context();
    create_effect(move |_| {
        if mouse_up.get().is_some() {
            knob.set_listening.set(false);
        }
    });

    // While this slider is "listening", project the relative cursor position into the sliders value range and propagate.
    create_effect(move |_| {
        if knob.listening.get() {
            set_value.set(project_into_range(
                cursor.rel_mouse_pos.get().0,
                range.get(),
                min,
                step,
            ))
        }
    });

    let marks = create_marks(
        min,
        max,
        step,
        range,
        move |v| match max > min {
            true => Signal::derive(move || v <= value.get()),
            false => Signal::derive(move || v >= value.get()),
        },
        marks,
        value_display,
    );

    view! {
        <leptonic-slider
            id=id
            data-variant=variant.to_str()
            class=class
            class:active=active
            class:disabled=disabled
            style=style
            // Note(lukas): Setting set_listening to false is handled though capturing a global mouseup event,
            // as the user may click, drag and move the cursor outside of this element.
            on:mousedown=move |_e| {
                bar.track_client_rect();
                knob.set_listening.set(true);
            }
            on:touchstart=move |_e| {
                bar.track_client_rect();
                knob.set_listening.set(true);
            }
            on:touchmove=move |e| {
                if knob.listening.get_untracked() {
                    e.prevent_default();
                    e.stop_propagation();
                }
            }
            on:touchend=move |_e| knob.set_listening.set(false)
        >
            <div class="bar-wrapper">
                <div node_ref=bar_el class="bar">
                    <div class="range" style=move || range_style.get()></div>
                    <div class="knob-wrapper">
                        <div class="knob" node_ref=knob_el class:is-dragged=move || knob.listening.get() tabindex=0 style=move || knob.style.get()>
                            <Popover show=move || show_popover.get()>
                                {
                                    move || {
                                        let value = value.get();
                                        match value_display {
                                            Some(callback) => callback.call(value) ,
                                            None => format!("{value}"),
                                        }
                                    }
                                }
                            </Popover>
                        </div>
                    </div>
                </div>
            </div>

            <Marks marks=marks/>

        </leptonic-slider>
    }
}

#[component]
pub fn RangeSlider(
    #[prop(into)] value_a: MaybeSignal<f64>,
    #[prop(into)] value_b: MaybeSignal<f64>,
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
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    #[prop(into, optional)] value_display: Option<Callback<f64, String>>,
) -> impl IntoView {
    let range = create_memo(move |_| max - min);

    let bar_el: NodeRef<html::Div> = create_node_ref();
    let bar = TrackedElementClientBoundingRect::new(bar_el);
    let cursor = RelativeMousePosition::new(bar);
    let knob_a_el: NodeRef<html::Div> = create_node_ref();
    let knob_b_el: NodeRef<html::Div> = create_node_ref();
    let knob_a = KnobControl::new(min, max, step, value_a);
    let knob_b = KnobControl::new(min, max, step, value_b);
    let show_a_popover = popover.to_maybe_signal(knob_a_el, &knob_a);
    let show_b_popover = popover.to_maybe_signal(knob_b_el, &knob_b);

    let range_style = Signal::derive(move || {
        format!(
            "left: {}%; width: {}%;",
            knob_a.clipped_value_percent.get() * 100.0,
            knob_b.clipped_value_percent.get() * 100.0 - knob_a.clipped_value_percent.get() * 100.0
        )
    });

    // Stop listening whenever any mouseup event got fired.
    let GlobalMouseupEvent {
        read_signal: mouse_up,
        ..
    } = expect_context();
    create_effect(move |_| {
        if mouse_up.get().is_some() {
            knob_a.set_listening.set(false);
            knob_b.set_listening.set(false);
        }
    });

    // Project the relative cursor position into the sliders value range.
    let projected_value_from_cursor = create_memo(move |_| {
        project_into_range(cursor.rel_mouse_pos.get().0, range.get(), min, step)
    });

    // While this slider is "listening", propagate the projected value.
    create_effect(move |_| {
        if knob_a.listening.get() {
            let projected_value_from_cursor = projected_value_from_cursor.get();
            let b = value_b.get_untracked();
            if projected_value_from_cursor > b {
                set_value_a.set(b);
                set_value_b.set(projected_value_from_cursor);

                knob_a.set_listening.set(false);
                knob_b.set_listening.set(true);
            } else {
                set_value_a.set(projected_value_from_cursor);
            }
        }
        if knob_b.listening.get() {
            let projected_value_from_cursor = projected_value_from_cursor.get();
            let a = value_a.get_untracked();
            if projected_value_from_cursor < a {
                set_value_b.set(a);
                set_value_a.set(projected_value_from_cursor);

                knob_b.set_listening.set(false);
                knob_a.set_listening.set(true);
            } else {
                set_value_b.set(projected_value_from_cursor);
            }
        }
    });

    let marks = create_marks(
        min,
        max,
        step,
        range,
        move |v| match max > min {
            true => Signal::derive(move || v >= value_a.get() && v <= value_b.get()),
            false => Signal::derive(move || v <= value_a.get() && v >= value_b.get()),
        },
        marks,
        value_display,
    );

    view! {
        <leptonic-slider
            id=id
            data-variant=variant.to_str()
            class=class
            class:active=active
            class:disabled=disabled
            style=style
            // Note(lukas): Setting set_listening to false is handled though capturing a global mouseup event,
            // as the user may click, drag and move the cursor outside of this element.
            on:mousedown=move |_e| {
                bar.track_client_rect();
                let could_be = projected_value_from_cursor.get();
                let distance_to_a = (value_a.get() - could_be).abs();
                let distance_to_b = (value_b.get() - could_be).abs();
                if distance_to_a < distance_to_b {
                    knob_a.set_listening.set(true)
                } else {
                    knob_b.set_listening.set(true)
                }
            }
            on:touchstart=move |_e| {
                bar.track_client_rect();
            }
            // Note(lukas): We do not use on:touchstart event here to trigger the listening functionality.
            // Instead, the code handling it lives in on:touchmove.
            // The reason for this is that the use_mouse function must receive the initial on:touchstart event FIRST,
            // so that a correct cursor.clipped_value can be computed. We can only then check whether or not the user
            // touched more towards the left or right knob.
            // Limitation: The initial touch event no longer results in a direct value change. But the value is set after touchmove or touchend.
            on:touchmove=move |e| {
                if knob_a.listening.get_untracked() || knob_b.listening.get_untracked() {
                    e.prevent_default();
                    e.stop_propagation();
                } else {
                    let could_be = projected_value_from_cursor.get();
                    let distance_to_a = (value_a.get() - could_be).abs();
                    let distance_to_b = (value_b.get() - could_be).abs();
                    if distance_to_a < distance_to_b {
                        knob_a.set_listening.set(true)
                    } else {
                        knob_b.set_listening.set(true)
                    }
                }
            }
            on:touchend=move |_e| {
                knob_a.set_listening.set(false);
                knob_b.set_listening.set(false);
            }
        >
            <div class="bar-wrapper">
                <div node_ref=bar_el class="bar">
                    <div class="knob-wrapper">
                        <div class="knob" node_ref=knob_a_el class:is-dragged=move || knob_a.listening.get() tabindex=0 style=move || knob_a.style.get()>
                            <Popover show=move || show_a_popover.get()>
                                {
                                    move || {
                                        let value = value_a.get();
                                        match value_display {
                                            Some(callback) => callback.call(value) ,
                                            None => format!("{value}"),
                                        }
                                    }
                                }
                            </Popover>
                        </div>
                    </div>
                    <div class="range" style=move || range_style.get()></div>
                    <div class="knob-wrapper">
                        <div class="knob" node_ref=knob_b_el class:is-dragged=move || knob_b.listening.get() tabindex=0 style=move || knob_b.style.get()>
                            <Popover show=move || show_b_popover.get()>
                                {
                                    move || {
                                        let value = value_b.get();
                                        match value_display {
                                            Some(callback) => callback.call(value) ,
                                            None => format!("{value}"),
                                        }
                                    }
                                }
                            </Popover>
                        </div>
                    </div>
                </div>
            </div>

            <Marks marks=marks/>
        </leptonic-slider>
    }
}

struct KnobControl {
    #[allow(unused)]
    clipped_value: Signal<f64>,
    clipped_value_percent: Signal<f64>,
    style: Signal<String>,
    listening: ReadSignal<bool>,
    set_listening: WriteSignal<bool>,
}

impl KnobControl {
    pub fn new(min: f64, max: f64, step: Option<f64>, value: MaybeSignal<f64>) -> Self {
        let range = create_memo(move |_| max - min);
        let clipped_value = Signal::derive(move || {
            let value = value.get();
            if !(min..=max).contains(&value) && !(max..=min).contains(&value) {
                tracing::warn!(
                    "Slider was given the value {value} which is outside the range [{min}, {max}]. Value will be clipped on first use of this slider."
                );
            }
            let clipped: f64 = if min < max {
                f64::min(f64::max(value, min), max)
            } else {
                f64::min(f64::max(value, max), min)
            };
            // Round to the nearest step if a step-value was provided.
            match step {
                Some(step) => (clipped / step).round() * step,
                None => clipped,
            }
        });
        let clipped_value_percent =
            Signal::derive(move || ((min.abs() - clipped_value.get()) / range.get()).abs());
        let style =
            Signal::derive(move || format!("left: {}%", clipped_value_percent.get() * 100.0));
        let (listening, set_listening) = create_signal(false);
        Self {
            clipped_value,
            clipped_value_percent,
            style,
            listening,
            set_listening,
        }
    }
}
