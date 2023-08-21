use crate::{contexts::global_mouseup_event::GlobalMouseupEvent, prelude::*};
use indoc::formatdoc;
use leptos::*;
use leptos_use::{use_mouse, UseMouseReturn};

#[component]
pub fn ColorPreview(
    cx: Scope,
    #[prop(into)] rgb: Signal<RGB8>,
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,
) -> impl IntoView {
    let background_color = move || {
        let RGB8 { r, g, b } = rgb.get();
        format!("rgb({r}, {g}, {b})")
    };

    view! {cx,
        <leptonic-color-preview id=id class=class style=style style:background-color=background_color>
        </leptonic-color-preview>
    }
}

#[component]
pub fn ColorPalette(
    cx: Scope,
    #[prop(into)] hsv: Signal<HSV>,
    #[prop(into)] set_saturation: Out<f64>,
    #[prop(into)] set_value: Out<f64>,
    #[prop(into, optional)] id: Option<AttributeValue>,
    //#[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,
) -> impl IntoView {
    let rgb_from_hue_only = Signal::derive(cx, move || {
        let hsv = hsv.get();
        RGB8::from(HSV {
            hue: hsv.hue,
            saturation: 1.0,
            value: 1.0,
        })
    });

    let background_simple = move || {
        let RGB8 { r, g, b } = rgb_from_hue_only.get();
        formatdoc!(
            r#"
            linear-gradient(to top,
                rgb(0, 0, 0) 0%,
                rgba(255, 255, 255, 0) 100%
            ),
            linear-gradient(to right,
                rgb(255, 255, 255) 0%,
                rgb({r}, {g}, {b}) 100%
            ),
            rgb({r}, {g}, {b})
        "#
        )
    };

    let knob_background_color = move || {
        let RGB8 { r, g, b } = rgb_from_hue_only.get();
        format!("rgb({r}, {g}, {b})")
    };

    let palette_el: NodeRef<html::Div> = create_node_ref(cx);
    let palette = PaletteControl::new(cx, palette_el);
    let cursor = Cursor2DControl::new(cx, 0.0, 1.0, None, 1.0, -1.0, None, palette);
    let knob = Knob2DControl::new(cx);

    let knob_left = move || format!("{}%", hsv.get().saturation * 100.0);
    let knob_bottom = move || format!("{}%", hsv.get().value * 100.0);

    // Stop listening whenever any mouseup event got fired.
    let GlobalMouseupEvent {
        read_signal: mouse_up,
        ..
    } = expect_context(cx);
    create_effect(cx, move |_| {
        if mouse_up.get().is_some() {
            knob.set_listening.set(false);
        }
    });

    // While this knob is "listening", propagate the value.
    create_effect(cx, move |_| {
        if knob.listening.get() {
            set_saturation.set(cursor.clipped_value_x.get());
            set_value.set(cursor.clipped_value_y.get());
        }
    });

    view! {cx,
        <div class="leptonic-color-palette"
            node_ref=palette_el
            id=id
            style=style
            style:background=background_simple
            // Note(lukas): Setting set_listening to false is handled though capturing a global mouseup event,
            // as the user may click, drag and move the cursor outside of this element.
            on:mousedown=move |_e| {
                palette.track_client_rect();
                knob.set_listening.set(true);
            }
            on:touchstart=move |_e| {
                palette.track_client_rect();
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
            <leptonic-color-palette-knob-wrapper style="">
                <leptonic-color-palette-knob data-variant="round" style:left=knob_left style:bottom=knob_bottom style=("--color-palette-knob-background-color", knob_background_color)>
                </leptonic-color-palette-knob>
            </leptonic-color-palette-knob-wrapper>
        </div>
    }
}

#[component]
pub fn HueSlider(
    cx: Scope,
    #[prop(into)] hue: Signal<f64>,
    #[prop(into)] set_hue: Out<f64>,
) -> impl IntoView {
    let rgb = Signal::derive(cx, move || {
        RGB8::from(HSV {
            hue: hue.get(),
            saturation: 1.0,
            value: 1.0,
        })
    });
    let rgb_css = move || {
        let RGB8 { r, g, b } = rgb.get();
        format!("rgb({r}, {g}, {b})")
    };
    let style = move || {
        format!(
            "--slider-knob-background-color: {0}; --slider-knob-halo-background-color: {0};",
            rgb_css()
        )
    };
    view! {cx,
        <leptonic-hue-slider>
            <Slider min=0.0 max=360.0
                value=hue set_value=set_hue
                marks=SliderMarks::None
                popover=SliderPopover::Never
                class="hue-slider"
                style=style
            />
        </leptonic-hue-slider>
    }
}

#[component]
pub fn ColorPicker(cx: Scope) -> impl IntoView {
    let (hsv, set_hsv) = create_signal(
        cx,
        HSV {
            hue: 0.0,
            saturation: 1.0,
            value: 1.0,
        },
    );

    let hue = Signal::derive(cx, move || hsv.get().hue);
    let set_hue = create_callback(cx, move |new_hue| set_hsv.update(|hsv| hsv.hue = new_hue));

    let rgb = Signal::derive(cx, move || RGB8::from(hsv.get()));

    view! {cx,
        <leptonic-color-picker>
            <div style="display: flex; flex-direction: row; justify-content: center; align-items: center; height: 20em;">
                <ColorPreview rgb=rgb style="width: 20%; height: 100%;"/>
                <ColorPalette hsv=hsv
                    set_saturation=create_callback(cx, move |new_saturation| set_hsv.update(|hsv| hsv.saturation = new_saturation))
                    set_value=create_callback(cx, move |new_value| set_hsv.update(|hsv| hsv.value = new_value))
                    style="width: 80%; height: 100%;"
                />
            </div>

            <HueSlider hue=hue set_hue=set_hue/>

            <P>"H: "{move || hsv.get().hue}</P>
            <P>"S: "{move || hsv.get().saturation}</P>
            <P>"V: "{move || hsv.get().value}</P>

            <P>"R: "{move || rgb.get().r}</P>
            <P>"G: "{move || rgb.get().g}</P>
            <P>"B: "{move || rgb.get().b}</P>
        </leptonic-color-picker>
    }
}

#[derive(Clone, Copy)]
struct PaletteControl {
    bar: NodeRef<html::Div>,
    bar_left: ReadSignal<f64>,
    set_bar_left: WriteSignal<f64>,
    bar_top: ReadSignal<f64>,
    set_bar_top: WriteSignal<f64>,
    bar_width: ReadSignal<f64>,
    set_bar_width: WriteSignal<f64>,
    bar_height: ReadSignal<f64>,
    set_bar_height: WriteSignal<f64>,
}

impl PaletteControl {
    pub fn new(cx: Scope, bar: NodeRef<html::Div>) -> Self {
        let (bar_left, set_bar_left) = create_signal(cx, 0.0);
        let (bar_top, set_bar_top) = create_signal(cx, 0.0);
        let (bar_width, set_bar_width) = create_signal(cx, 0.0);
        let (bar_height, set_bar_height) = create_signal(cx, 0.0);

        Self {
            bar,
            bar_left,
            set_bar_left,
            bar_top,
            set_bar_top,
            bar_width,
            set_bar_width,
            bar_height,
            set_bar_height,
        }
    }

    fn track_client_rect(&self) {
        if let Some(rect) = self.bar.get().map(|el| el.get_bounding_client_rect()) {
            self.set_bar_left.set(rect.left());
            self.set_bar_top.set(rect.top());
            self.set_bar_width.set(rect.width());
            self.set_bar_height.set(rect.height());
        }
    }
}

struct Cursor2DControl {
    clipped_value_x: Memo<f64>,
    clipped_value_y: Memo<f64>,
}

impl Cursor2DControl {
    pub fn new(
        cx: Scope,
        min_x: f64,
        range_x: f64,
        step_x: Option<f64>,
        min_y: f64,
        range_y: f64,
        step_y: Option<f64>,
        palette_control: PaletteControl,
    ) -> Self {
        let UseMouseReturn {
            x: cursor_x,
            y: cursor_y,
            ..
        } = use_mouse(cx);
        let cursor_rel_pos_percent = Signal::derive(cx, move || {
            let x = cursor_x.get() - palette_control.bar_left.get();
            let y = cursor_y.get() - palette_control.bar_top.get();
            // Using custom x,y instead of event.offset_x/y,
            // because event.offset was computed for the direct target, which must not be the target we got now.
            let mut px = x / palette_control.bar_width.get();
            let mut py = y / palette_control.bar_height.get();
            px = f64::max(0.0, f64::min(1.0, px));
            py = f64::max(0.0, f64::min(1.0, py));
            (px, py)
        });
        let clipped_value_x = create_memo(cx, move |_| {
            let value_in_range: f64 = cursor_rel_pos_percent.get().0 * range_x + min_x;
            // Round to the nearest step value if a step is provided.
            match step_x {
                Some(step_x) => (value_in_range / step_x).round() * step_x,
                None => value_in_range,
            }
        });
        let clipped_value_y = create_memo(cx, move |_| {
            let value_in_range: f64 = cursor_rel_pos_percent.get().1 * range_y + min_y;
            // Round to the nearest step value if a step is provided.
            match step_y {
                Some(step_y) => (value_in_range / step_y).round() * step_y,
                None => value_in_range,
            }
        });
        Self {
            clipped_value_x,
            clipped_value_y,
        }
    }
}

struct Knob2DControl {
    listening: ReadSignal<bool>,
    set_listening: WriteSignal<bool>,
}

impl Knob2DControl {
    pub fn new(cx: Scope) -> Self {
        let (listening, set_listening) = create_signal(cx, false);
        Self {
            listening,
            set_listening,
        }
    }
}
