use std::time::Duration;

use leptonic::{
    atoms::slider::{
        Slider as SliderAtom, SliderOutput, SliderThumb, SliderTrack, SliderTrackFill,
    },
    components::prelude::*,
    hooks::*,
};
use leptos::prelude::*;
use leptos_styles::{
    Style::{
        AlignItems, Background, BackgroundColor, Border, BorderRadius, BoxShadow, Cursor, Display,
        Flex, Gap, Height, MinWidth, Position, TextAlign, Width,
    },
    Styles,
};
use ringbuf::{
    HeapRb,
    traits::{Consumer, RingBuffer},
};

fn track_style() -> Styles {
    Styles::from([
        (Display, "flex"),
        (Width, "200px"),
        (Height, "8px"),
        (Background, "#ddd"),
        (BorderRadius, "4px"),
        (Position, "relative"),
        (Cursor, "pointer"),
    ])
}

fn thumb_style() -> Styles {
    Styles::builder()
        .with(Width, "20px")
        .with(Height, "20px")
        .with(BackgroundColor, "var(--brand-color)")
        .with(BorderRadius, "50%")
        .with(Border, "2px solid white")
        .with(BoxShadow, "0 2px 4px rgba(0,0,0,0.2)")
        .with(Cursor, "grab")
        .build()
}

#[component]
pub fn PressLongDemo() -> impl IntoView {
    let (lp_count, set_lp_count) = signal(0);
    let (lp_events, set_lp_events) = signal(HeapRb::<Oco<'static, str>>::new(20));
    let (lp_disabled, set_lp_disabled) = signal(false);
    let (threshold, set_threshold) = signal(Duration::from_millis(500));

    let lp_string = Memo::new(move |_| {
        lp_events.with(|events| {
            let mut result = String::new();
            for e in events.iter().rev() {
                result.push_str(e.as_str());
                result.push('\n');
            }
            result
        })
    });

    let UsePressReturn {
        props: lp_props,
        is_pressed: _,
    } = use_press(UsePressInput {
        disabled: lp_disabled.into(),
        force_prevent_default: false,
        force_propagation: false,
        allow_text_selection_on_press: false,
        should_cancel_on_pointer_exit: false,
        prevent_focus_on_press: false,
        force_is_pressed: None,
        on_press: Callback::new(|_| {}),
        on_press_up: None,
        on_press_start: None,
        on_press_end: None,
        on_press_change: None,
        on_double_press: None,
        on_long_press_start: Some(Callback::new(move |e: LongPressEvent| {
            set_lp_events.update(|events| {
                events.push_overwrite(Oco::Owned(format!(
                    "LongPressStart: pointer_type={:?}, x={:?}, y={:?}",
                    e.pointer_type, e.x, e.y,
                )));
            });
        })),
        on_long_press: Some(Callback::new(move |e: LongPressEvent| {
            set_lp_count.update(|c| *c += 1);
            set_lp_events.update(|events| {
                events.push_overwrite(Oco::Owned(format!(
                    "LongPress: pointer_type={:?}, x={:?}, y={:?}",
                    e.pointer_type, e.x, e.y,
                )));
            });
        })),
        on_long_press_end: Some(Callback::new(move |e: LongPressEvent| {
            set_lp_events.update(|events| {
                events.push_overwrite(Oco::Owned(format!(
                    "LongPressEnd: pointer_type={:?}, x={:?}, y={:?}",
                    e.pointer_type, e.x, e.y,
                )));
            });
        })),
        long_press_threshold: Some(threshold.into()),
        long_press_accessibility_description: Some("Long press to increment counter".into()),
    });

    let (lp_press_props, lp_press_styles) = lp_props.into_inner();

    view! {
        <p>"Press and hold the button below for " { move || threshold.get().as_millis() } "ms to trigger a long press:"</p>

        <button
            {..lp_press_props.into_attrs()}
            style=lp_press_styles
                .add("display", "inline-flex")
                .add("border", "2px solid var(--brand-color)")
                .add("padding", "1em 2em")
                .add("cursor", "pointer")
                .add("border-radius", "4px")
                .add("background", "var(--brand-color)")
                .add("color", "white")
                .add("font-size", "1em")
        >
            "Long press me"
        </button>

        <FormControl attr:style="flex-direction: row; align-items: center; gap: 0.5em; margin-top: 1em;">
            <Checkbox checked=lp_disabled set_checked=set_lp_disabled />
            <Label>"Disabled"</Label>
        </FormControl>

        <div style="display: flex; align-items: center; gap: 0.5em; margin-top: 0.5em;">
            <Label>"Threshold:"</Label>
            <SliderAtom
                values=SliderValues::Uncontrolled(vec![500.0])
                min=100.0
                max=2000.0
                step=100.0
                on_change=Callback::new(move |vals: Vec<f64>| {
                    set_threshold.set(Duration::from_secs_f64(vals[0] / 1000.0));
                })
                styles=[(Display, "flex"), (AlignItems, "center"), (Gap, "0.5em"), (Flex, "0 0 200px")]
            >
                <SliderTrack styles=track_style()>
                    <SliderTrackFill styles=[(BackgroundColor, "var(--brand-color)"), (BorderRadius, "4px")]/>
                    <SliderThumb styles=thumb_style()/>
                </SliderTrack>
                <SliderOutput let:attrs let:values>
                    <output {..attrs} style=Styles::from([(MinWidth, "60px"), (TextAlign, "right")])>
                        { move || format!("{:.0}ms", values.get()[0]) }
                    </output>
                </SliderOutput>
            </SliderAtom>
        </div>

        <p>"Long press count: " { move || lp_count.get() }</p>

        <p>"Last " { move || lp_events.with(ringbuf::traits::Observer::occupied_len) } " events:"</p>

        <pre style="
            width: 100%;
            height: 10em;
            overflow: auto;
            padding: var(--typography-code-padding);
            border: none;
            border-radius: var(--typography-code-border-radius);
            background-color: var(--typography-code-background-color);
            color: var(--typography-code-color);
        ">
            { move || lp_string.get() }
        </pre>
    }
}
