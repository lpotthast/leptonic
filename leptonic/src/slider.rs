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
        if value < min || value > max {
            tracing::warn!(
                "Slider was given the value {value} which is outside the allowed range [{min}-{max}]. Value will be clipped on first use of this slider."
            );
        }
        let clipped: f64 = f64::min(f64::max(value, min), max);
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
            on:mousedown=move |_| set_listening.set(true)
        >
            <div node_ref=el id=id class="bar">
                <div class="knob-wrapper">
                    <div class="knob" style=move || knob_style.get()></div>
                </div>
            </div>
        </leptonic-slider>
    }
}
