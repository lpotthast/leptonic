use crate::{
    contexts::global_mouseup_event::GlobalMouseupEvent, math::project_into_range, prelude::*,
    RelativeMousePosition, TrackedElementClientBoundingRect,
};
use indoc::formatdoc;
use leptos::{leptos_dom::Callback, *};

#[component]
pub fn ColorPreview(
    #[prop(into)] rgb: Signal<RGB8>,
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,
) -> impl IntoView {
    let background_color = move || {
        let RGB8 { r, g, b } = rgb.get();
        format!("rgb({r}, {g}, {b})")
    };

    view! {
        <leptonic-color-preview id=id class=class style=style style:background-color=background_color>
        </leptonic-color-preview>
    }
}

#[component]
pub fn ColorPalette(
    #[prop(into)] hsv: Signal<HSV>,
    #[prop(into)] set_saturation: Out<f64>,
    #[prop(into)] set_value: Out<f64>,
    #[prop(into, optional)] id: Option<AttributeValue>,
    //#[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,
) -> impl IntoView {
    let rgb_from_hue_only = Signal::derive(move || {
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

    let palette_el: NodeRef<html::Div> = create_node_ref();
    let palette = TrackedElementClientBoundingRect::new(palette_el);
    let cursor = RelativeMousePosition::new(palette);
    let (knob_listening, set_knob_listening) = create_signal(false);

    let knob_left = move || format!("{}%", hsv.get().saturation * 100.0);
    let knob_bottom = move || format!("{}%", hsv.get().value * 100.0);

    // Stop listening whenever any mouseup event got fired.
    let GlobalMouseupEvent {
        read_signal: mouse_up,
        ..
    } = expect_context();
    create_effect(move |_| {
        if mouse_up.get().is_some() {
            set_knob_listening.set(false);
        }
    });

    // Project the relative cursor position into the sliders value range.
    let projected_value_from_cursor_x =
        create_memo(move |_| project_into_range(cursor.rel_mouse_pos.get().0, 1.0, 0.0, None));
    let projected_value_from_cursor_y = create_memo(move |_| {
        1.0 - project_into_range(cursor.rel_mouse_pos.get().1, 1.0, 0.0, None)
    });

    // While this knob is "listening", propagate the projected values.
    create_effect(move |_| {
        if knob_listening.get() {
            set_saturation.set(projected_value_from_cursor_x.get());
            set_value.set(projected_value_from_cursor_y.get());
        }
    });

    view! {
        <div class="leptonic-color-palette"
            node_ref=palette_el
            id=id
            style=style
            style:background=background_simple
            // Note(lukas): Setting set_listening to false is handled though capturing a global mouseup event,
            // as the user may click, drag and move the cursor outside of this element.
            on:mousedown=move |_e| {
                palette.track_client_rect();
                set_knob_listening.set(true);
            }
            on:touchstart=move |e| {
                palette.track_client_rect();
                set_knob_listening.set(true);
                e.prevent_default();
                e.stop_propagation();
            }
            on:touchmove=move |e| {
                if knob_listening.get_untracked() {
                    e.prevent_default();
                    e.stop_propagation();
                }
            }
            on:touchend=move |_e| set_knob_listening.set(false)
        >
            <leptonic-color-palette-knob-wrapper style="">
                <leptonic-color-palette-knob data-variant="round" style:left=knob_left style:bottom=knob_bottom style=("--color-palette-knob-background-color", knob_background_color)>
                </leptonic-color-palette-knob>
            </leptonic-color-palette-knob-wrapper>
        </div>
    }
}

#[component]
pub fn HueSlider(#[prop(into)] hue: Signal<f64>, #[prop(into)] set_hue: Out<f64>) -> impl IntoView {
    let rgb = Signal::derive(move || {
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
    view! {
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
pub fn ColorPicker(
    #[prop(into)] hsv: Signal<HSV>,
    #[prop(into)] set_hsv: Out<HSV>,
) -> impl IntoView {
    let hue = Signal::derive(move || hsv.get().hue);
    let saturation = Signal::derive(move || hsv.get().saturation);
    let value = Signal::derive(move || hsv.get().value);

    let set_hsv_c1 = set_hsv.clone();
    let set_hsv_c2 = set_hsv.clone();

    let set_hue =
        Callback::new(move |new_hue| set_hsv_c1.set(hsv.get_untracked().with_hue(new_hue)));

    let set_saturation = Callback::new(move |new_saturation| {
        set_hsv_c2.set(hsv.get_untracked().with_saturation(new_saturation))
    });

    let set_value =
        Callback::new(move |new_value| set_hsv.set(hsv.get_untracked().with_value(new_value)));

    let rgb = Signal::derive(move || RGB8::from(hsv.get()));

    view! {
        <leptonic-color-picker>
            <div style="display: flex; flex-direction: row; justify-content: center; align-items: center; height: 20em;">
                <ColorPreview rgb=rgb style="width: 20%; height: 100%;"/>
                <ColorPalette hsv=hsv
                    set_saturation=set_saturation.clone()
                    set_value=set_value.clone()
                    style="width: 80%; height: 100%;"
                />
            </div>

            <HueSlider hue=hue set_hue=set_hue.clone()/>

            <div style="display: flex; flex-direction: row;">
                <Field style="width: 32%; margin-right: 2%;">
                    <FieldLabel>"Hue"</FieldLabel>
                    <NumberInput min=0.0 max=360.0 step=1.0
                        get=hue
                        set=set_hue
                    />
                </Field>
                <Field style="width: 32%; margin-right: 2%;">
                    <FieldLabel>"Saturation"</FieldLabel>
                    <NumberInput min=0.0 max=1.0 step=0.01
                        get=saturation
                        set=set_saturation
                    />
                </Field>
                <Field style="width: 32%; margin-right: 0%;">
                    <FieldLabel>"Value"</FieldLabel>
                    <NumberInput min=0.0 max=1.0 step=0.01
                        get=value
                        set=set_value
                    />
                </Field>
            </div>

            <div style="display: flex; flex-direction: row;">
                <Field style="width: 32%; margin-right: 2%;">
                    <FieldLabel>"R"</FieldLabel>
                    <NumberInput min=0.0 max=255.0 step=1.0
                        get=Signal::derive(move || rgb.get().r as f64)
                    />
                </Field>
                <Field style="width: 32%; margin-right: 2%;">
                    <FieldLabel>"G"</FieldLabel>
                    <NumberInput min=0.0 max=255.0 step=1.0
                        get=Signal::derive(move || rgb.get().g as f64)
                    />
                </Field>
                <Field style="width: 32%; margin-right: 0%;">
                    <FieldLabel>"B"</FieldLabel>
                    <NumberInput min=0.0 max=255.0 step=1.0
                        get=Signal::derive(move || rgb.get().b as f64)
                    />
                </Field>
            </div>

            <P>"Hex: #"{move || format!("{:X}", rgb.get())}</P>
        </leptonic-color-picker>
    }
}
