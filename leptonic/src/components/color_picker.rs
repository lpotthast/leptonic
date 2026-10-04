use leptos::prelude::*;

use crate::{
    atoms::color_swatch::ColorSwatch,
    components::{
        field::{Field, FieldLabel},
        input::NumberInput,
        prelude::{Slider, SliderMarks},
    },
    hooks::{IntoAttrs, MoveConstraint, NormalizedPosition, UseMoveInput, use_move},
    prelude::*,
    utils::{
        classes::Classes,
        color::{HSV, RGB8},
        css::{
            CssColor, LengthPercentageAuto, computed_pct, computed_size, css_custom_property, em,
            pct,
        },
        style::{BottomProperty, HeightProperty, LeftProperty, MarginRightProperty, WidthProperty},
        styles::Styles,
    },
};

css_custom_property!(KNOB_BACKGROUND_COLOR: CssColor = "--color-palette-knob-background-color");
css_custom_property!(SLIDER_THUMB_BACKGROUND_COLOR: CssColor = "--slider-thumb-background-color");
css_custom_property!(
    SLIDER_THUMB_HALO_BACKGROUND_COLOR: CssColor = "--slider-thumb-halo-background-color"
);

fn width_height(width: f64, height: f64) -> Styles {
    Styles::builder()
        .with(WidthProperty.declare(computed_size(pct(width))))
        .with(HeightProperty.declare(computed_size(pct(height))))
        .build()
}

#[component]
pub fn ColorPreview(
    #[prop(into)] rgb: Signal<RGB8>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
) -> impl IntoView {
    view! {
        <ColorSwatch<RGB8> color=rgb classes=classes.add("leptonic-color-preview") styles=styles />
    }
}

/// 2D color gradient area for selecting saturation (X) and brightness (Y).
///
/// Uses `use_move` with `MoveConstraint` for pointer/touch/keyboard interaction.
/// The `hsv` prop is the single source of truth — all visuals are derived from it.
/// Move callbacks write to the parent imperatively (no signal-writing Effects).
#[component]
pub fn ColorPalette(
    #[prop(into)] hsv: Signal<HSV>,
    #[prop(into)] set_saturation: Out<f64>,
    #[prop(into)] set_value: Out<f64>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
) -> impl IntoView {
    let initial_sat = hsv.get_untracked().saturation;
    let initial_val = hsv.get_untracked().value;

    let move_return = use_move(UseMoveInput {
        disabled: Signal::derive(|| false),
        axis: Signal::derive(|| None),
        is_rtl: false,
        on_move_start: None,
        on_move: None,
        on_move_end: None,
        on_position_change: Some(Callback::new(move |pos: NormalizedPosition| {
            set_saturation.set(pos.x.clamp(0.0, 1.0));
            set_value.set((1.0 - pos.y).clamp(0.0, 1.0));
        })),
        constraint: Some(MoveConstraint::Center),
        allow_container_click: true,
        initial_position: Some(NormalizedPosition {
            x: initial_sat,
            y: 1.0 - initial_val,
        }),
    });

    let constraint_return = move_return.constraint.expect("MoveConstraint was provided");

    let norm_pos = constraint_return.normalized_position;

    // A gradient has no checked grammar in `leptos-css` yet.
    let styles = styles.add_optional_unchecked("background", move || {
        let color = hsv.get();
        let rgb = RGB8::from(HSV {
            hue: color.hue,
            saturation: 1.0,
            value: 1.0,
        });
        let top_right = format!("rgb({}, {}, {})", rgb.r, rgb.g, rgb.b);
        Some(format!(
            "linear-gradient(to top, rgb(0, 0, 0) 0%, transparent 100%), \
             linear-gradient(to right, rgb(255, 255, 255) 0%, {top_right} 100%)"
        ))
    });

    let knob_styles = Styles::new()
        .add_reactive(move || {
            LeftProperty.declare(LengthPercentageAuto::from(computed_pct(
                norm_pos.get().x * 100.0,
            )))
        })
        .add_reactive(move || {
            BottomProperty.declare(LengthPercentageAuto::from(computed_pct(
                (1.0 - norm_pos.get().y) * 100.0,
            )))
        })
        .add_reactive(move || {
            let color = hsv.get();
            let hue_rgb = RGB8::from(HSV {
                hue: color.hue,
                saturation: 1.0,
                value: 1.0,
            });
            KNOB_BACKGROUND_COLOR.declare(CssColor::from(hue_rgb))
        });

    view! {
        <div
            class=classes.add("leptonic-color-palette")
            {..constraint_return.container_props.into_attrs()}
            style=styles
        >
            <div class="leptonic-color-palette-knob-wrapper">
                <div
                    class="leptonic-color-palette-knob"
                    data-variant="round"
                    {..move_return.props.into_attrs()}
                    style=knob_styles
                ></div>
            </div>
        </div>
    }
}

#[component]
pub fn HueSlider(
    #[prop(into)] hue: Signal<f64>,
    #[prop(into)] set_hue: Out<f64>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
) -> impl IntoView {
    let rgb = Signal::derive(move || {
        RGB8::from(HSV {
            hue: hue.get(),
            saturation: 1.0,
            value: 1.0,
        })
    });
    let slider_styles = Styles::new()
        .add_reactive(move || SLIDER_THUMB_BACKGROUND_COLOR.declare(CssColor::from(rgb.get())))
        .add_reactive(move || {
            SLIDER_THUMB_HALO_BACKGROUND_COLOR.declare(CssColor::from(rgb.get()))
        });
    view! {
        <div class=classes.add("leptonic-hue-slider") style=styles>
            <Slider
                min=0.0
                max=360.0
                value=hue
                set_value=set_hue
                marks=SliderMarks::None
                classes="hue-slider"
                styles=slider_styles
            />
        </div>
    }
}

#[component]
pub fn ColorPicker(
    #[prop(into)] hsv: Signal<HSV>,
    #[prop(into)] set_hsv: Out<HSV>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
) -> impl IntoView {
    let hue = Signal::derive(move || hsv.get().hue);
    let saturation = Signal::derive(move || hsv.get().saturation);
    let value = Signal::derive(move || hsv.get().value);

    let set_hue = move |new_hue| set_hsv.set(hsv.get_untracked().with_hue(new_hue));

    let set_saturation = move |new_saturation| {
        set_hsv.set(hsv.get_untracked().with_saturation(new_saturation));
    };

    let set_value = move |new_value| set_hsv.set(hsv.get_untracked().with_value(new_value));

    let rgb = Signal::derive(move || RGB8::from(hsv.get()));

    let flex_row_styles = Styles::builder()
        .with_unchecked("display", "flex")
        .with_unchecked("flex-direction", "row")
        .build();

    let flex_row_centered_styles = Styles::builder()
        .with_unchecked("display", "flex")
        .with_unchecked("flex-direction", "row")
        .with_unchecked("justify-content", "center")
        .with_unchecked("align-items", "center")
        .with(HeightProperty.declare(computed_size(em(20.0))))
        .build();

    let field_styles = Styles::builder()
        .with(WidthProperty.declare(computed_size(pct(32.0))))
        .with(MarginRightProperty.declare(LengthPercentageAuto::from(pct(2.0))))
        .build();

    let field_styles_last = Styles::builder()
        .with(WidthProperty.declare(computed_size(pct(32.0))))
        .with(MarginRightProperty.declare(LengthPercentageAuto::from(pct(0.0))))
        .build();

    view! {
        <div class=classes.add("leptonic-color-picker") style=styles>
            <div style=flex_row_centered_styles>
                <ColorPreview
                    rgb=rgb
                    styles=width_height(20.0, 100.0)
                />
                <ColorPalette
                    hsv=hsv
                    set_saturation=set_saturation
                    set_value=set_value
                    styles=width_height(80.0, 100.0)
                />
            </div>

            <HueSlider hue=hue set_hue=set_hue />

            <div style=flex_row_styles.clone()>
                <Field styles=field_styles.clone()>
                    <FieldLabel>"Hue"</FieldLabel>
                    <NumberInput min=0.0 max=360.0 step=1.0 get=hue set=set_hue />
                </Field>
                <Field styles=field_styles.clone()>
                    <FieldLabel>"Saturation"</FieldLabel>
                    <NumberInput min=0.0 max=1.0 step=0.01 get=saturation set=set_saturation />
                </Field>
                <Field styles=field_styles_last.clone()>
                    <FieldLabel>"Value"</FieldLabel>
                    <NumberInput min=0.0 max=1.0 step=0.01 get=value set=set_value />
                </Field>
            </div>

            <div style=flex_row_styles>
                <Field styles=field_styles.clone()>
                    <FieldLabel>"R"</FieldLabel>
                    <NumberInput
                        min=0.0
                        max=255.0
                        step=1.0
                        get=Signal::derive(move || f64::from(rgb.get().r))
                    />
                </Field>
                <Field styles=field_styles>
                    <FieldLabel>"G"</FieldLabel>
                    <NumberInput
                        min=0.0
                        max=255.0
                        step=1.0
                        get=Signal::derive(move || f64::from(rgb.get().g))
                    />
                </Field>
                <Field styles=field_styles_last>
                    <FieldLabel>"B"</FieldLabel>
                    <NumberInput
                        min=0.0
                        max=255.0
                        step=1.0
                        get=Signal::derive(move || f64::from(rgb.get().b))
                    />
                </Field>
            </div>

            <p>"Hex: #"{move || format!("{:X}", rgb.get())}</p>
        </div>
    }
}
