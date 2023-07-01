use leptos::*;
use leptos_use::{use_mouse, use_resize_observer, UseMouseReturn};
use uuid::Uuid;

use crate::contexts::global_mouseup_event::GlobalMouseupEvent;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum SliderVariant {
    Block,
    #[default]
    Round,
}

impl SliderVariant {
    fn to_str(&self) -> &'static str {
        match self {
            SliderVariant::Block => "block",
            SliderVariant::Round => "round",
        }
    }
}

#[component]
pub fn Slider<S>(
    cx: Scope,
    #[prop(into)] value: MaybeSignal<f64>,
    set_value: S,
    min: f64,
    max: f64,
    step: f64,
    #[prop(optional)] variant: SliderVariant,
    #[prop(optional)] active: bool,
    #[prop(optional)] disabled: bool,
) -> impl IntoView
where
    S: Fn(f64) + 'static,
{
    let uuid = Uuid::new_v4();
    let id = format!("{}", uuid);

    let UseMouseReturn {
        x: cursor_x,
        y: cursor_y,
        ..
    } = use_mouse(cx);

    let el = create_node_ref(cx);

    let (bar_left, set_bar_left) = create_signal(cx, 0.0);
    let (bar_top, set_bar_top) = create_signal(cx, 0.0);
    let (bar_width, set_bar_width) = create_signal(cx, 0.0);
    let (bar_height, set_bar_height) = create_signal(cx, 0.0);

    use_resize_observer(cx, el, move |entries, _observer| {
        use wasm_bindgen::JsCast;
        let target = entries[0]
            .target()
            .dyn_into::<web_sys::HtmlElement>()
            .expect("Observed element to be an HtmlElement.");
        let rect = entries[0].content_rect();
        set_bar_left.set(target.offset_left() as f64);
        set_bar_top.set(target.offset_top() as f64);
        set_bar_width.set(rect.width());
        set_bar_height.set(rect.height());
    });

    let range = create_memo(cx, move |_| max - min);

    let cursor_rel_pos_percent = Signal::derive(cx, move || {
        let x = cursor_x.get() - bar_left.get();
        let y = cursor_y.get() - bar_top.get();
        // Using custom x,y instead of event.offset_x/y,
        // because event.offset was computed for the direct target, which must not be the target we got now.
        let mut px = x / bar_width.get();
        let mut py = y / bar_height.get();
        px = f64::max(0.0, f64::min(1.0, px));
        py = f64::max(0.0, f64::min(1.0, py));
        (px, py)
    });

    let clipped_value_from_cursor = create_memo(cx, move |_| {
        let value_in_range: f64 = cursor_rel_pos_percent.get().0 * range.get() + min;
        (value_in_range / step).round() * step
    });

    let clipped_value = Signal::derive(cx, move || {
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
        (clipped / step).round() * step
    });

    // We could use clipped.get(), bu expect our value to also be clipped already.
    let clipped_value_percent = Signal::derive(cx, move || {
        ((min.abs() - clipped_value.get()) / range.get()).abs()
    });

    let knob_style = Signal::derive(cx, move || {
        format!("left: {}%", clipped_value_percent.get() * 100.0)
    });

    let (listening, set_listening) = create_signal(cx, false);

    // Stop listening whenever any mouseup event got fired.
    let GlobalMouseupEvent {
        read_signal: mouse_up,
        ..
    } = expect_context(cx);
    create_effect(cx, move |_| {
        if mouse_up.get().is_some() {
            set_listening.set(false);
        }
    });

    // While this slider is "listening", propagate the value.
    create_effect(cx, move |_| {
        if listening.get() {
            set_value(clipped_value_from_cursor.get())
        }
    });

    view! {cx,
        <leptonic-slider
            variant=variant.to_str()
            class:active=active
            class:disabled=disabled
            // Note(lukas): Setting set_listening to false is handled though capturing a global mouseup event,
            // as the user may click, drag and move the cursor outside of this element.
            on:mousedown=move |_e| set_listening.set(true)
            on:touchstart=move |_e| set_listening.set(true)
            on:touchmove=move |e| {
                if listening.get_untracked() {
                    e.prevent_default();
                    e.stop_propagation();
                }
            }
            on:touchend=move |_e| set_listening.set(false)
        >
            <div node_ref=el id=id class="bar">
                <div class="knob-wrapper">
                    <div class="knob" style=move || knob_style.get()></div>
                </div>
            </div>
        </leptonic-slider>
    }
}

#[component]
pub fn RangeSlider<Sa, Sb>(
    cx: Scope,
    #[prop(into)] value_a: MaybeSignal<f64>,
    #[prop(into)] value_b: MaybeSignal<f64>,
    set_value_a: Sa,
    set_value_b: Sb,
    min: f64,
    max: f64,
    step: f64,
    #[prop(optional)] variant: SliderVariant,
    #[prop(optional)] active: bool,
    #[prop(optional)] disabled: bool,
) -> impl IntoView
where
    Sa: Fn(f64) + 'static,
    Sb: Fn(f64) + 'static,
{
    let uuid = Uuid::new_v4();
    let id = format!("{}", uuid);

    let UseMouseReturn {
        x: cursor_x,
        y: cursor_y,
        ..
    } = use_mouse(cx);

    let el = create_node_ref(cx);

    let (bar_left, set_bar_left) = create_signal(cx, 0.0);
    let (bar_top, set_bar_top) = create_signal(cx, 0.0);
    let (bar_width, set_bar_width) = create_signal(cx, 0.0);
    let (bar_height, set_bar_height) = create_signal(cx, 0.0);

    use_resize_observer(cx, el, move |entries, _observer| {
        use wasm_bindgen::JsCast;
        let target = entries[0]
            .target()
            .dyn_into::<web_sys::HtmlElement>()
            .expect("Observed element to be an HtmlElement.");
        let rect = entries[0].content_rect();
        set_bar_left.set(target.offset_left() as f64);
        set_bar_top.set(target.offset_top() as f64);
        set_bar_width.set(rect.width());
        set_bar_height.set(rect.height());
    });

    let range = create_memo(cx, move |_| max - min);

    let cursor_rel_pos_percent = Signal::derive(cx, move || {
        let x = cursor_x.get() - bar_left.get();
        let y = cursor_y.get() - bar_top.get();
        // Using custom x,y instead of event.offset_x/y,
        // because event.offset was computed for the direct target, which must not be the target we got now.
        let mut px = x / bar_width.get();
        let mut py = y / bar_height.get();
        px = f64::max(0.0, f64::min(1.0, px));
        py = f64::max(0.0, f64::min(1.0, py));
        (px, py)
    });

    // This is the actual value which can be returned to the user of this component.
    let clipped_value_from_cursor = create_memo(cx, move |_| {
        let value_in_range: f64 = cursor_rel_pos_percent.get().0 * range.get() + min;
        (value_in_range / step).round() * step
    });

    // A
    let clipped_value_a = Signal::derive(cx, move || {
        let value = value_a.get();
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
        (clipped / step).round() * step
    });

    // B
    let clipped_value_b = Signal::derive(cx, move || {
        let value = value_b.get();
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
        (clipped / step).round() * step
    });

    // We could use clipped.get(), bu expect our value to also be clipped already.
    let clipped_value_percent_a = Signal::derive(cx, move || {
        ((min.abs() - clipped_value_a.get()) / range.get()).abs()
    });

    // We could use clipped.get(), bu expect our value to also be clipped already.
    let clipped_value_percent_b = Signal::derive(cx, move || {
        ((min.abs() - clipped_value_b.get()) / range.get()).abs()
    });

    let knob_style_a = Signal::derive(cx, move || {
        format!("left: {}%", clipped_value_percent_a.get() * 100.0)
    });
    let knob_style_b = Signal::derive(cx, move || {
        format!("left: {}%", clipped_value_percent_b.get() * 100.0)
    });
    let range_style = Signal::derive(cx, move || {
        format!(
            "left: {}%; width: {}%;",
            clipped_value_percent_a.get() * 100.0,
            clipped_value_percent_b.get() * 100.0 - clipped_value_percent_a.get() * 100.0
        )
    });

    let (listening_a, set_listening_a) = create_signal(cx, false);
    let (listening_b, set_listening_b) = create_signal(cx, false);

    // Stop listening whenever any mouseup event got fired.
    let GlobalMouseupEvent {
        read_signal: mouse_up,
        ..
    } = expect_context(cx);
    create_effect(cx, move |_| {
        if mouse_up.get().is_some() {
            set_listening_a.set(false);
            set_listening_b.set(false);
        }
    });

    // While this slider is "listening", propagate the value.
    create_effect(cx, move |_| {
        let clipped_value_from_cursor = clipped_value_from_cursor.get();

        if listening_a.get() {
            let b = value_b.get_untracked();
            if clipped_value_from_cursor > b {
                set_value_a(b);
                set_value_b(clipped_value_from_cursor);

                set_listening_a.set(false);
                set_listening_b.set(true);
            } else {
                set_value_a(clipped_value_from_cursor);
            }
        }
        if listening_b.get() {
            let a = value_a.get_untracked();
            if clipped_value_from_cursor < a {
                set_value_b(a);
                set_value_a(clipped_value_from_cursor);

                set_listening_b.set(false);
                set_listening_a.set(true);
            } else {
                set_value_b(clipped_value_from_cursor);
            }
        }
    });

    view! {cx,
        <leptonic-slider
            variant=variant.to_str()
            class:active=active
            class:disabled=disabled
            // Note(lukas): Setting set_listening to false is handled though capturing a global mouseup event,
            // as the user may click, drag and move the cursor outside of this element.
            on:mousedown=move |_e| {
                let could_be = clipped_value_from_cursor.get();
                let distance_to_a = (value_a.get() - could_be).abs();
                let distance_to_b = (value_b.get() - could_be).abs();
                if distance_to_a < distance_to_b {
                    set_listening_a.set(true)
                } else {
                    set_listening_b.set(true)
                }
            }
            // Note(lukas): We do not use on:touchstart event here to trigger the listening functionality.
            // Instead, the code handling it lives in on:touchmove.
            // The reason for this is that the use_mouse function must receive the initial on:touchstart event FIRST,
            // so that a correct clipped_value_from_cursor can be computed. We can only then check whether or not the user
            // touched more towards the left or right knob.
            // Limitation: The initial touch event no longer results in a direct value change. But the value is set after touchmove or touchend.
            on:touchmove=move |e| {
                if listening_a.get_untracked() {
                    e.prevent_default();
                    e.stop_propagation();
                } else if listening_b.get_untracked() {
                    e.prevent_default();
                    e.stop_propagation();
                } else {
                    let could_be = clipped_value_from_cursor.get();
                    let distance_to_a = (value_a.get() - could_be).abs();
                    let distance_to_b = (value_b.get() - could_be).abs();
                    if distance_to_a < distance_to_b {
                        set_listening_a.set(true)
                    } else {
                        set_listening_b.set(true)
                    }
                }
            }
            on:touchend=move |_e| {
                set_listening_a.set(false);
                set_listening_b.set(false);
            }
        >
            <div node_ref=el id=id class="bar">
                <div class="knob-wrapper">
                    <div class="knob" style=move || knob_style_a.get()></div>
                </div>
                <div class="range" style=move || range_style.get()></div>
                <div class="knob-wrapper">
                    <div class="knob" style=move || knob_style_b.get()></div>
                </div>
            </div>
        </leptonic-slider>
    }
}
